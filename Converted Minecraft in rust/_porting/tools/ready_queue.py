#!/usr/bin/env python3
"""List SKELETON files whose dependencies are all ported, best-first.

READ-ONLY. Touches nothing. This is a scheduling aid, not part of the build.

WHY IT IS TRANSITIVE
--------------------
A file is ready when every type it names is already non-SKELETON. Checking only its direct
dependencies would offer up files that import other unported files, and the first compile would
fail on a name nobody looked at. So this walks the dependency graph until it stops making
progress, then reports what is genuinely at the front of it.

WHY "AT THE FRONT" IS THE POINT
--------------------------------
There are 7,031 SKELETON files and 24 ported ones. Almost nothing is ready, so the queue starts
small and grows as files land -- which is why the instruction is to re-run it after every commit.
A queue computed once is worthless here.

ORDERING
--------
1. side: `common` before `server`, `client` never. Client code is rendering and is not this
   project's target; putting it first would fill the queue with files nobody asked for.
2. java_loc ascending. Small files first: they are cheap, they unblock more, and a session that
   only gets through twenty files should be twenty real ones rather than one enormous one.

JIT / LIBRARY DEPENDENCIES
--------------------------
`depends_on` holds fully-qualified names. Anything not under `net.minecraft.` is an external
type (DataFixerUpper, netty, fastutil, JOML, the JDK) and is NOT a blocker -- those are handled
by `todo!("PORT: ...")`. Only `net.minecraft.` names are followed.
"""

from __future__ import annotations

import csv
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
PORTING = os.path.dirname(HERE)
MANIFEST = os.path.join(PORTING, "MANIFEST.csv")

SIDE_ORDER = {"common": 0, "server": 1, "client": 9}


def load():
    with open(MANIFEST, newline="", encoding="utf-8") as fh:
        rows = list(csv.DictReader(fh))
    status = {}
    fq_to_path = {}
    path_to_row = {}
    for r in rows:
        jp = r["java_path"]
        status[jp] = r["status"]
        path_to_row[jp] = r
        # net/minecraft/core/BlockPos.java -> net.minecraft.core.BlockPos
        fq = jp[:-5].replace("/", ".")
        fq_to_path[fq] = jp
    return rows, status, fq_to_path, path_to_row


def deps_of(row):
    """Dependency paths this file is blocked on. External types are dropped."""
    out = []
    for d in (row.get("depends_on") or "").split(";"):
        d = d.strip()
        if not d:
            continue
        d = d.removesuffix(".*")
        if not d.startswith("net.minecraft."):
            continue  # JDK / DFU / netty / fastutil / JOML -> todo!(), not a blocker
        jp = d.replace(".", "/") + ".java"
        out.append(jp)
    return out


def main(argv):
    limit = int(argv[0]) if argv else 40
    rows, status, fq_to_path, path_to_row = load()

    # A file is ported if it is not SKELETON. Fixpoint so a chain A -> B -> C where B and C are
    # also SKELETON keeps A, B and C all blocked until the whole chain is done.
    ported = {jp for jp, st in status.items() if st != "SKELETON"}

    ready = []
    blocked_by_unknown = 0
    for r in rows:
        jp = r["java_path"]
        if status[jp] != "SKELETON":
            continue
        side = r.get("side", "")
        if side not in SIDE_ORDER:
            continue  # unknown side: skip rather than guess
        deps = deps_of(r)
        if all(d in ported or d not in path_to_row for d in deps):
            ready.append(r)

    ready.sort(key=lambda r: (SIDE_ORDER[r["side"]], int(r["java_loc"]), r["java_path"]))

    total_skeleton = sum(1 for r in rows if r["status"] == "SKELETON")
    print(f"manifest rows      : {len(rows)}")
    print(f"non-SKELETON       : {len(rows) - total_skeleton}")
    print(f"SKELETON           : {total_skeleton}")
    print(f"READY NOW          : {len(ready)}")
    print(f"client skipped     : {sum(1 for r in rows if r.get('side') == 'client' and r['status'] == 'SKELETON')}")
    print()
    for r in ready[:limit]:
        ndeps = len(deps_of(r))
        print(f"  {r['java_loc']:>6}  {r['side']:<8} deps={ndeps:<3} {r['java_path']}")
    if len(ready) > limit:
        print(f"  ... and {len(ready) - limit} more")
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))