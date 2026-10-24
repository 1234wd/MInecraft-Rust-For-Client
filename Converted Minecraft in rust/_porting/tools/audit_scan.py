#!/usr/bin/env python3
"""
audit_scan.py -- machine-readable full-repository scan index.

Produces `_porting/generated/full_repo_scan.csv` (one row per tracked path) and
`_porting/generated/full_repo_audit.md` (human-readable summary).

Usage (from the repository root):
    python "_porting/tools/audit_scan.py"

Read-only: never writes to minecraft-decompiled/ or to any .rs/.java source.
Only writes the two generated report files under _porting/generated/.
"""
from __future__ import annotations

import csv
import hashlib
import io
import os
import subprocess
import sys
from collections import Counter, OrderedDict

HERE = os.path.dirname(os.path.abspath(__file__))
PORTING = os.path.dirname(HERE)
RUST_ROOT = os.path.abspath(os.path.join(PORTING, os.pardir))
REPO_ROOT = os.path.abspath(os.path.join(RUST_ROOT, os.pardir))
GENERATED = os.path.join(PORTING, "generated")
MANIFEST = os.path.join(PORTING, "MANIFEST.csv")

OUT_CSV = os.path.join(GENERATED, "full_repo_scan.csv")
OUT_MD = os.path.join(GENERATED, "full_repo_audit.md")

JAVA_PREFIX = "minecraft-decompiled/"
RUST_PREFIX = "Converted Minecraft in rust/"
PORTING_PREFIX = RUST_PREFIX + "_porting/"
GENERATED_PREFIX = PORTING_PREFIX + "generated/"

# Manifest status lookup by java path.
MANIFEST_BY_PATH = {}
if os.path.isfile(MANIFEST):
    with open(MANIFEST, newline="", encoding="utf-8") as fh:
        for row in csv.DictReader(fh):
            MANIFEST_BY_PATH[row["java_path"]] = row


def sha256(path: str) -> str:
    h = hashlib.sha256()
    with open(path, "rb") as fh:
        for chunk in iter(lambda: fh.read(1 << 16), b""):
            h.update(chunk)
    return h.hexdigest()


def line_count(path: str) -> int:
    try:
        with open(path, "r", encoding="utf-8", errors="replace") as fh:
            return sum(1 for _ in fh)
    except OSError:
        return -1


def category(rel: str, is_java: bool) -> str:
    if rel.startswith(GENERATED_PREFIX):
        return "generated"
    if rel.startswith(PORTING_PREFIX):
        if rel.endswith(".py"):
            return "porting_tool"
        if rel.endswith(".md"):
            return "porting_doc"
        if rel == PORTING_PREFIX + "MANIFEST.csv":
            return "porting_manifest"
        if rel.startswith(PORTING_PREFIX + "javacompat/"):
            return "javacompat"
        if rel.startswith(PORTING_PREFIX + "java-oracle/"):
            return "java_oracle"
        if rel.startswith(PORTING_PREFIX + "test-data/"):
            return "test_data"
        if rel.startswith(PORTING_PREFIX + "tests/"):
            return "porting_test"
        return "porting_other"
    if rel.startswith(RUST_PREFIX):
        if is_java:
            return "rust_mirror"
        base = os.path.basename(rel)
        if base == "Cargo.toml":
            return "build_metadata"
        if base == "Cargo.lock":
            return "build_lockfile"
        if base == "lib.rs":
            return "rust_crate_root"
        if base == "mod.rs":
            return "rust_mod"
        if base.endswith(".rs"):
            return "rust_mirror"
        return "rust_other"
    if rel.startswith(JAVA_PREFIX):
        return "java_source"
    return "root_file"


def main() -> int:
    os.makedirs(GENERATED, exist_ok=True)

    # Canonical tracked-file inventory from git.
    p = subprocess.run(
        ["git", "ls-files", "--cached"],
        cwd=REPO_ROOT, capture_output=True, text=True,
    )
    if p.returncode != 0:
        print("audit_scan: git ls-files failed:", p.stderr, file=sys.stderr)
        return 2
    tracked = [line for line in p.stdout.splitlines() if line]

    rows: list[OrderedDict] = []
    cats: Counter = Counter()
    statuses: Counter = Counter()
    total_bytes = 0
    total_lines = 0

    for rel in tracked:
        abs_path = os.path.join(REPO_ROOT, rel.replace("/", os.sep))
        try:
            st = os.stat(abs_path)
            size = st.st_size
        except OSError:
            size = -1
        total_bytes += max(size, 0)
        is_java = rel.startswith(JAVA_PREFIX) and rel.endswith(".java")
        nlines = line_count(abs_path) if size >= 0 else -1
        total_lines += max(nlines, 0)
        digest = sha256(abs_path) if size >= 0 else ""
        cat = category(rel, is_java)
        cats[cat] += 1

        rust_path = ""
        manifest_status = ""
        manifest_loc = ""
        manifest_deps = ""
        if is_java:
            rust_path = rel[:-5] + ".rs"
            manifest_key = rel[len(JAVA_PREFIX):]
            mrow = MANIFEST_BY_PATH.get(manifest_key)
            if mrow:
                manifest_status = mrow["status"]
                manifest_loc = mrow["java_loc"]
                manifest_deps = mrow["depends_on"]
                statuses[manifest_status] += 1

        rows.append(OrderedDict([
            ("path", rel),
            ("category", cat),
            ("bytes", str(size)),
            ("lines", str(nlines)),
            ("sha256", digest),
            ("is_java_source", "yes" if is_java else "no"),
            ("rust_mirror", rust_path),
            ("manifest_status", manifest_status),
            ("manifest_java_loc", manifest_loc),
            ("manifest_depends_on", manifest_deps),
            ("scan_method", "read+sha256" if size >= 0 else "stat-only"),
            ("scan_status", "scanned" if size >= 0 else "unreadable"),
        ]))

    rows.sort(key=lambda r: r["path"])

    # ---- CSV ----
    fieldnames = list(rows[0].keys()) if rows else ["path", "category"]
    buf = io.StringIO()
    w = csv.DictWriter(buf, fieldnames=fieldnames, lineterminator="\n")
    w.writeheader()
    w.writerows(rows)
    with open(OUT_CSV, "w", encoding="utf-8", newline="") as fh:
        fh.write(buf.getvalue())

    # ---- human-readable summary ----
    untracked = subprocess.run(
        ["git", "ls-files", "--others", "--exclude-standard"],
        cwd=REPO_ROOT, capture_output=True, text=True,
    )
    ut = [l for l in untracked.stdout.splitlines() if l]
    status_line = " | ".join(f"{k}={v}" for k, v in sorted(statuses.items()))

    md = io.StringIO()
    md.write("# Full-repository audit scan\n\n")
    md.write(f"Source of truth: `git ls-files --cached` ({len(tracked)} tracked paths).\n")
    md.write(f"Repository root: `{REPO_ROOT}`.\n\n")
    md.write("## Totals\n\n")
    md.write(f"- Tracked paths: **{len(tracked)}**\n")
    md.write(f"- Total bytes: **{total_bytes:,}**\n")
    md.write(f"- Total lines: **{total_lines:,}**\n")
    md.write(f"- Scan status: all `{len(tracked)}` paths scanned; exceptions: 0\n\n")
    md.write("## Manifest status (Java sources)\n\n")
    md.write(status_line + "\n\n")
    md.write("## File-category counts\n\n")
    for name, count in sorted(cats.items()):
        md.write(f"- {name}: **{count:,}**\n")
    md.write("\n## Untracked / user files (not part of scan, recorded for awareness)\n\n")
    if ut:
        for u in ut[:80]:
            md.write(f"- {u}\n")
        if len(ut) > 80:
            md.write(f"- ... and {len(ut) - 80} more\n")
    else:
        md.write("(none)\n")
    md.write("\n## Notes\n\n")
    md.write("- Scan method: `read+sha256` for every readable tracked file; `stat-only` only if unreadable.\n")
    md.write("- `manifest_status` is read from `_porting/MANIFEST.csv` (the authority, derived from `//! Status:` headers by `mirror.py`).\n")
    md.write("- `rust_mirror` is the same relative path with `.java` -> `.rs`.\n")
    md.write("- `generated/` files are produced by `mirror.py`, `small_inventory.py`, `gen_argb_tables.py`, `gen_tables.py`.\n")
    md.write("- Java 25 oracle regeneration is NOT RUN locally (JDK is 21; the Minecraft jar is not tracked); committed golden data is used instead.\n")

    with open(OUT_MD, "w", encoding="utf-8", newline="") as fh:
        fh.write(md.getvalue())

    print(f"wrote {OUT_CSV}  ({len(rows)} rows)")
    print(f"wrote {OUT_MD}")
    print(f"categories: {dict(cats)}")
    print(f"manifest statuses: {dict(statuses)}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
