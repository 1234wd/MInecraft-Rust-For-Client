#!/usr/bin/env python3
"""
check_mirror.py -- fail loudly if the Rust mirror and the Java source tree drift apart.

Fails (exit 1) when any of these is true:

  1. A `.java` file in minecraft-decompiled/ has no matching `.rs` file in
     "Converted Minecraft in rust/" at the same relative path with the same base name.
  2. A `.rs` file in "Converted Minecraft in rust/" (excluding `lib.rs`, every
     `mod.rs`, and everything under `_porting/`) has no matching `.java` file.
  3. `minecraft-decompiled/` changed vs. the git commit recorded at session start
     (stored in _porting/.mc_head).  Detects edits, deletions AND untracked files.

Usage
-----
    python "Converted Minecraft in rust/_porting/tools/check_mirror.py"
    python "Converted Minecraft in rust/_porting/tools/check_mirror.py" --quiet
"""

from __future__ import annotations

import argparse
import os
import subprocess
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
RUST_ROOT = os.path.abspath(os.path.join(HERE, os.pardir, os.pardir))   # .../Converted Minecraft in rust
REPO_ROOT = os.path.abspath(os.path.join(RUST_ROOT, os.pardir))        # repo root
SRC = os.path.join(REPO_ROOT, "minecraft-decompiled")
BASELINE = os.path.join(RUST_ROOT, "_porting", ".mc_head")

EXEMPT_FILES = {"lib.rs"}
EXEMPT_DIRS = {"_porting"}
EXEMPT_BASENAMES = {"mod.rs"}


def git(*args: str) -> tuple[int, str]:
    p = subprocess.run(["git", *args], cwd=REPO_ROOT, capture_output=True, text=True)
    return p.returncode, p.stdout.strip()


def walk(root: str, suffix: str, skip_dirs=frozenset()) -> list:
    out = []
    for dp, dn, fn in os.walk(root):
        dn[:] = sorted(d for d in dn if d not in skip_dirs)
        for f in sorted(fn):
            if f.endswith(suffix):
                out.append(os.path.relpath(os.path.join(dp, f), root).replace("\\", "/"))
    return out


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--quiet", action="store_true")
    args = ap.parse_args()
    problems: list = []

    def say(*a):
        if not args.quiet:
            print(*a)

    if not os.path.isdir(SRC):
        print(f"FATAL: missing {SRC}", file=sys.stderr)
        return 2

    java = walk(SRC, ".java")
    say(f"[check_mirror] source tree : {SRC}")
    say(f"[check_mirror] rust tree   : {RUST_ROOT}")
    say(f"[check_mirror] .java files : {len(java)}")

    # ---- 1. every .java has a .rs -------------------------------------------
    missing_rs = []
    for rel in java:
        rs = rel[:-5] + ".rs"
        if not os.path.isfile(os.path.join(RUST_ROOT, rs.replace("/", os.sep))):
            missing_rs.append(rs)
    if missing_rs:
        problems.append(f"{len(missing_rs)} .java file(s) with NO matching .rs:")
        problems += ["  " + p for p in missing_rs[:50]]
        if len(missing_rs) > 50:
            problems.append(f"  ... and {len(missing_rs) - 50} more")
    say(f"[check_mirror] .java without .rs : {len(missing_rs)}")

    # ---- 2. every .rs has a .java -------------------------------------------
    rs_files = walk(RUST_ROOT, ".rs", skip_dirs=EXEMPT_DIRS)
    orphans = []
    for rel in rs_files:
        base = rel.rsplit("/", 1)[-1]
        if base in EXEMPT_FILES or base in EXEMPT_BASENAMES:
            continue
        java_rel = rel[:-3] + ".java"
        if java_rel not in set(java):
            orphans.append(rel)
    if orphans:
        problems.append(f"{len(orphans)} .rs file(s) with NO matching .java:")
        problems += ["  " + p for p in orphans[:50]]
        if len(orphans) > 50:
            problems.append(f"  ... and {len(orphans) - 50} more")
    say(f"[check_mirror] .rs total / orphans : {len(rs_files)} / {len(orphans)}")

    # ---- 3. minecraft-decompiled/ untouched ----------------------------------
    rc, tracked = git("diff", "--name-only", "HEAD", "--", "minecraft-decompiled")
    dirty = [x for x in tracked.splitlines() if x] if rc == 0 else ["<git diff failed>"]
    rc2, untracked = git("ls-files", "--others", "--exclude-standard", "--", "minecraft-decompiled")
    extra = [x for x in untracked.splitlines() if x] if rc2 == 0 else ["<git ls-files failed>"]
    if dirty or extra:
        problems.append("minecraft-decompiled/ MODIFIED vs git HEAD:")
        problems += ["  " + p for p in (dirty + extra)[:50]]

    rc3, base_head = git("rev-parse", "HEAD")
    baseline = None
    if os.path.isfile(BASELINE):
        with open(BASELINE, "r", encoding="ascii") as fh:
            baseline = fh.read().strip()
    if rc3 == 0 and baseline:
        rc4, tree_now = git("rev-parse", f"{base_head}:minecraft-decompiled")
        rc5, tree_then = git("rev-parse", f"{baseline}:minecraft-decompiled")
        if rc4 == 0 and rc5 == 0:
            if tree_now != tree_then:
                problems.append(
                    f"minecraft-decompiled/ tree changed since session start:\n"
                    f"    baseline {baseline[:12]} -> tree {tree_then[:12]}\n"
                    f"    current  {base_head[:12]} -> tree {tree_now[:12]}")
        else:
            problems.append("could not resolve minecraft-decompiled tree object (shallow clone?)")
    else:
        problems.append("no _porting/.mc_head baseline recorded")

    say(f"[check_mirror] source tree vs git : {'CLEAN' if not (dirty or extra) else 'MODIFIED'}"
        f"  (baseline {baseline[:12] if baseline else '??'})")

    if problems:
        print()
        print("=" * 72)
        print("check_mirror.py: FAIL")
        print("=" * 72)
        for line in problems:
            print(line)
        print()
        return 1

    print("[check_mirror] OK -- mirror is 1:1 with minecraft-decompiled/, source untouched.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())