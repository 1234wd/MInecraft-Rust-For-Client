#!/usr/bin/env python3
"""Build a conservative inventory and dependency graph for every small unfinished file.

This is a review scheduler, not a port-readiness oracle. A file with no unported Minecraft
dependency still needs its complete Java contract, external API behavior, and tests reviewed.
The tool therefore never calls a file "ready" or changes manifest status.

Usage from the repository root:
    python "Converted Minecraft in rust/_porting/tools/small_inventory.py"
    python "Converted Minecraft in rust/_porting/tools/small_inventory.py" --check

The default run writes a machine-readable CSV and a human-readable checkpoint under
`_porting/generated/`. `--check` verifies that both committed snapshots match current sources.
"""

from __future__ import annotations

import argparse
import csv
import hashlib
import io
import os
import re
import sys
from collections import Counter, defaultdict
from pathlib import Path

HERE = Path(__file__).resolve().parent
PORTING = HERE.parent
RUST_ROOT = PORTING.parent
REPO_ROOT = RUST_ROOT.parent
MANIFEST = PORTING / "MANIFEST.csv"
JAVA_ROOT = REPO_ROOT / "minecraft-decompiled"
OUT_CSV = PORTING / "generated" / "small_file_inventory.csv"
OUT_MD = PORTING / "generated" / "small_file_checkpoint.md"
TARGET_STATUSES = {"SKELETON", "PARTIAL"}
IMPORT_RE = re.compile(r"^\s*import\s+(static\s+)?([^;]+);", re.MULTILINE)
TODO_RE = re.compile(r"\b(?:todo|unimplemented)!\s*\(")


def read_manifest() -> list[dict[str, str]]:
    with MANIFEST.open(newline="", encoding="utf-8") as handle:
        rows = list(csv.DictReader(handle))
    if not rows:
        raise ValueError(f"empty manifest: {MANIFEST}")
    return rows


def java_path_for_fqcn(name: str, known_paths: set[str] | None = None) -> str:
    """Resolve a class or nested type to its outer mirrored .java file when possible."""
    parts = name.replace("$", ".").split(".")
    while parts:
        candidate = "/".join(parts) + ".java"
        if known_paths is None or candidate in known_paths:
            return candidate
        parts.pop()
    return name.replace("$", ".").replace(".", "/") + ".java"


def direct_deps(row: dict[str, str]) -> list[str]:
    return sorted(
        {
            item.strip().removesuffix(".*")
            for item in row.get("depends_on", "").split(";")
            if item.strip()
        }
    )


def external_imports(source: str, known_paths: set[str]) -> list[str]:
    found: set[str] = set()
    for is_static, raw_name in IMPORT_RE.findall(source):
        name = raw_name.strip()
        if name.endswith(".*"):
            name = name[:-2]
            # Wildcard imports name a package, so retain them as package-level external
            # contracts unless they refer to a package in this source mirror.
            if name.startswith(("net.minecraft.", "com.mojang.")):
                continue
            found.add(("static " if is_static else "") + name + ".*")
            continue
        if java_path_for_fqcn(name, known_paths) in known_paths:
            continue
        found.add(("static " if is_static else "") + name)
    return sorted(found)


def source_surface_hint(source: str, simple_name: str) -> str:
    """Return a deliberately conservative hint, never a claim of exact API coverage."""
    lines = []
    for line in source.splitlines():
        stripped = line.strip()
        if not stripped or stripped.startswith(("//", "*", "/*", "import ", "package ")):
            continue
        if re.search(r"\b(class|interface|enum|record)\s+", stripped):
            lines.append("type:" + stripped[:160])
        elif re.search(r"\b(public|protected)\b", stripped) and ("(" in stripped or ";" in stripped):
            lines.append("decl:" + stripped[:160])
    if not lines:
        return "no public/protected declaration found by line scanner; full source review required"
    # Limit each CSV cell; exact source files remain the authority.
    return " | ".join(lines[:24]) + (" | ..." if len(lines) > 24 else "")


def dependency_graph(rows: list[dict[str, str]], row_by_java: dict[str, dict[str, str]]):
    graph: dict[str, list[str]] = {}
    for row in rows:
        source = row["java_path"]
        graph[source] = []
        for fqcn in direct_deps(row):
            dest = java_path_for_fqcn(fqcn)
            if dest in row_by_java:
                graph[source].append(dest)
        graph[source] = sorted(set(graph[source]))
    return graph


def transitive_skeleton_count(start: str, graph: dict[str, list[str]], status: dict[str, str]) -> int:
    seen: set[str] = set()
    stack = list(graph.get(start, ()))
    while stack:
        item = stack.pop()
        if item in seen:
            continue
        seen.add(item)
        stack.extend(graph.get(item, ()))
    seen.discard(start)
    return sum(status.get(item) == "SKELETON" for item in seen)


def strongly_connected_components(graph: dict[str, list[str]]) -> list[list[str]]:
    """Tarjan SCCs, including self-loops, over the full manifest dependency graph."""
    sys.setrecursionlimit(max(sys.getrecursionlimit(), len(graph) * 2 + 100))
    index = 0
    stack: list[str] = []
    on_stack: set[str] = set()
    indices: dict[str, int] = {}
    low: dict[str, int] = {}
    result: list[list[str]] = []

    def visit(node: str) -> None:
        nonlocal index
        indices[node] = low[node] = index
        index += 1
        stack.append(node)
        on_stack.add(node)
        for neighbor in graph.get(node, ()):
            if neighbor not in indices:
                visit(neighbor)
                low[node] = min(low[node], low[neighbor])
            elif neighbor in on_stack:
                low[node] = min(low[node], indices[neighbor])
        if low[node] == indices[node]:
            component: list[str] = []
            while True:
                item = stack.pop()
                on_stack.remove(item)
                component.append(item)
                if item == node:
                    break
            if len(component) > 1 or node in graph.get(node, ()):
                result.append(sorted(component))

    for node in graph:
        if node not in indices:
            visit(node)
    return result


def build(rows: list[dict[str, str]]):
    row_by_java = {row["java_path"]: row for row in rows}
    fq_status = {
        row["java_path"][:-5].replace("/", "."): row["status"] for row in rows
    }
    known_paths = set(row_by_java)
    graph = dependency_graph(rows, row_by_java)
    targets = [
        row
        for row in rows
        if int(row["java_loc"]) <= 100 and row["status"] in TARGET_STATUSES
    ]
    targets.sort(key=lambda row: (row["side"], int(row["java_loc"]), row["java_path"]))

    output_rows: list[dict[str, str]] = []
    edge_rows: list[tuple[str, str, str]] = []
    direct_blockers: Counter[str] = Counter()
    external_counts: Counter[str] = Counter()
    frontier_by_side: Counter[str] = Counter()

    for target_index, row in enumerate(targets, start=1):
        if target_index % 500 == 0:
            print(f"small_inventory: scanned {target_index:,}/{len(targets):,} targets", flush=True)
        java_path = row["java_path"]
        java_source_path = JAVA_ROOT / Path(java_path)
        rust_path = RUST_ROOT / Path(row["rust_path"])
        if not java_source_path.is_file() or not rust_path.is_file():
            raise FileNotFoundError(f"missing source/mirror for {java_path}")
        source = java_source_path.read_text(encoding="utf-8")
        rust_source = rust_path.read_text(encoding="utf-8")
        deps = direct_deps(row)
        known_internal = [dep for dep in deps if java_path_for_fqcn(dep, known_paths) in known_paths]
        blocked = [dep for dep in known_internal if fq_status.get(dep) == "SKELETON"]
        external = external_imports(source, known_paths)
        imports_by_name = Counter(external)
        direct_blockers.update(blocked)
        external_counts.update(imports_by_name)
        if not blocked:
            frontier_by_side[row["side"]] += 1

        placeholders = TODO_RE.findall(rust_source)
        if row["status"] == "SKELETON":
            behavior_gap = "entire Java source contract remains unimplemented"
        else:
            behavior_gap = "PARTIAL: compare all constructors, fields, initializers, methods, and errors"
        if external:
            behavior_gap += "; external imports require contract-level review"
        output_rows.append(
            {
                "java_path": java_path,
                "rust_path": row["rust_path"],
                "java_loc": row["java_loc"],
                "side": row["side"],
                "status": row["status"],
                "direct_internal_dependencies": ";".join(known_internal),
                "direct_skeleton_dependencies": ";".join(blocked),
                "external_imports": ";".join(external),
                "unfinished_contract": behavior_gap,
                "source_surface_hint": source_surface_hint(source, Path(java_path).stem),
                "rust_todo_or_unimplemented_count": str(len(placeholders)),
            }
        )
        for dep in deps:
            target_path = java_path_for_fqcn(dep, known_paths)
            target_status = row_by_java[target_path]["status"] if target_path in row_by_java else "EXTERNAL_OR_UNRESOLVED"
            edge_rows.append((java_path, dep, target_status))

    manifest_status_counts = Counter(row["status"] for row in rows)
    target_status_counts = Counter(row["status"] for row in targets)
    target_side_counts = Counter(row["side"] for row in targets)
    print(f"small_inventory: computing cycles across {len(graph):,} manifest nodes", flush=True)
    sccs = strongly_connected_components(graph)
    print(f"small_inventory: found {len(sccs):,} cyclic components", flush=True)
    target_set = {row["java_path"] for row in targets}
    relevant_sccs = [component for component in sccs if target_set.intersection(component)]

    csv_buffer = io.StringIO(newline="")
    fieldnames = list(output_rows[0]) if output_rows else [
        "java_path", "rust_path", "java_loc", "side", "status",
        "direct_internal_dependencies", "direct_skeleton_dependencies",
        "external_imports", "unfinished_contract",
        "source_surface_hint", "rust_todo_or_unimplemented_count",
    ]
    writer = csv.DictWriter(csv_buffer, fieldnames=fieldnames, lineterminator="\n")
    writer.writeheader()
    writer.writerows(output_rows)

    edges_buffer = io.StringIO(newline="")
    edge_writer = csv.writer(edges_buffer, lineterminator="\n")
    edge_writer.writerow(("source_java_path", "dependency_type", "dependency_status"))
    edge_writer.writerows(sorted(set(edge_rows)))

    manifest_hash = hashlib.sha256(MANIFEST.read_bytes()).hexdigest()
    md = [
        "# Small-file conversion inventory checkpoint",
        "",
        "Generated by `_porting/tools/small_inventory.py`; regenerate after every status or dependency change.",
        "This is a conservative scheduling aid. A frontier entry only lacks a direct SKELETON Minecraft dependency; it is **not** implementation-ready or verified.",
        "",
        f"Manifest SHA-256: `{manifest_hash}`",
        f"Manifest source files: **{len(rows):,}**",
        f"Target rule: `java_loc <= 100 AND status IN (SKELETON, PARTIAL)`",
        f"Small unfinished targets: **{len(targets):,}**",
        f"Small SKELETON: **{target_status_counts['SKELETON']:,}**; PARTIAL: **{target_status_counts['PARTIAL']:,}**",
        "",
        "## Whole-manifest status",
        "",
        "| Status | Files |",
        "|---|---:|",
    ]
    for status_name in ("VERIFIED", "PORTED", "PARTIAL", "SKELETON", "MISSING"):
        md.append(f"| {status_name} | {manifest_status_counts[status_name]:,} |")
    md.extend(["", "## Target split", "", "| Side | SKELETON | PARTIAL | Total |", "|---|---:|---:|---:|"])
    for side in ("common", "server", "client"):
        counts = Counter(row["status"] for row in targets if row["side"] == side)
        md.append(f"| {side} | {counts['SKELETON']:,} | {counts['PARTIAL']:,} | {target_side_counts[side]:,} |")

    direct_frontier = sum(1 for row in output_rows if not row["direct_skeleton_dependencies"])
    external_target_count = sum(1 for row in output_rows if row["external_imports"])
    md.extend(
        [
            "",
            "## Dependency graph",
            "",
            f"- Direct Minecraft dependency edges from the small targets: **{len(edge_rows):,}**.",
            f"- Internal dependency edges across the full manifest: **{sum(len(v) for v in graph.values()):,}** (the committed edge CSV keeps edges from every small target).",
            f"- Files without a direct SKELETON Minecraft dependency (review frontier only): **{direct_frontier:,}**.",
            f"- Target files with external imports that need contract-level review: **{external_target_count:,}**.",
            f"- Strongly connected components touching the small target set: **{len(relevant_sccs):,}**.",
            "- External imports are retained as explicit unresolved contracts; the scheduler does not assume that a matching Rust helper fully models them.",
            "- Every target still requires full source-to-Rust member and behavior review. SKELETON rows have no ported behavior; PARTIAL rows need a complete member audit.",
            "",
            "### Most referenced direct SKELETON dependencies",
            "",
            "| Dependency | Small-file references |",
            "|---|---:|",
        ]
    )
    for dep, count in direct_blockers.most_common(30):
        md.append(f"| `{dep}` | {count:,} |")
    md.extend(
        [
            "",
            "### Most common external import contracts",
            "",
            "| External import | Target files |",
            "|---|---:|",
        ]
    )
    for name, count in external_counts.most_common(30):
        md.append(f"| `{name}` | {count:,} |")
    md.extend(
        [
            "",
            "## Complete per-file data",
            "",
            "`small_file_inventory.csv` lists every target, direct internal dependencies, external imports, a declaration hint, and placeholder counts. `small_dependency_edges.csv` is the complete direct edge list for those targets; it can be followed to any prerequisite in the full manifest, and cyclic groups touching targets are counted above.",
            "",
            "No file is removed from scope for being client-side, having an external dependency, or being inconvenient. Package-info files are present under the same numeric/status rule.",
            "",
        ]
    )
    return csv_buffer.getvalue(), edges_buffer.getvalue(), "\n".join(md), output_rows, edge_rows


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true", help="fail if generated inventories differ")
    args = parser.parse_args()
    rows = read_manifest()
    inventory, edges, checkpoint, target_rows, edge_rows = build(rows)
    outputs = (
        (OUT_CSV, inventory),
        (PORTING / "generated" / "small_dependency_edges.csv", edges),
        (OUT_MD, checkpoint),
    )
    if args.check:
        dirty = [str(path.relative_to(PORTING)) for path, content in outputs if not path.is_file() or path.read_text(encoding="utf-8") != content]
        if dirty:
            print("DRIFT: regenerate the small-file inventory:")
            for path in dirty:
                print("  " + path)
            return 1
        print(f"OK: {len(target_rows):,} small unfinished files and {len(edge_rows):,} dependency edges match the snapshot.")
        return 0
    for path, content in outputs:
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(content, encoding="utf-8", newline="\n")
    print(f"manifest rows: {len(rows):,}")
    print(f"small unfinished targets: {len(target_rows):,}")
    print(f"dependency edges: {len(edge_rows):,}")
    print(f"wrote: {OUT_CSV.relative_to(REPO_ROOT)}")
    print(f"wrote: {(PORTING / 'generated' / 'small_dependency_edges.csv').relative_to(REPO_ROOT)}")
    print(f"wrote: {OUT_MD.relative_to(REPO_ROOT)}")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except (OSError, ValueError, csv.Error) as error:
        print(f"small_inventory: {error}", file=sys.stderr)
        raise SystemExit(2)
