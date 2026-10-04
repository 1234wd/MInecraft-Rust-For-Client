#!/usr/bin/env python3
"""Set MANIFEST.csv status + notes for finished files.

Session-06 loop helper. `check_manifest.py` is the reader of this file, so the edit has to
be CSV-correct (embedded commas, quotes) -- hence the csv module rather than a text replace.
"""
import csv
import io
import sys

PATH = "_porting/MANIFEST.csv"

rows = list(csv.reader(io.open(PATH, newline="", encoding="utf-8")))
header = rows[0]
si = header.index("status")
ni = header.index("notes")

wanted = {}
for arg in sys.argv[1:]:
    path, status, note = arg.split("::", 2)
    wanted[path] = (status, note)

n = 0
for r in rows[1:]:
    if r[0] in wanted:
        r[si], r[ni] = wanted[r[0]]
        n += 1

missing = set(wanted) - {r[0] for r in rows[1:]}
if missing:
    sys.exit("not in manifest: %s" % sorted(missing))
if n != len(wanted):
    sys.exit("expected %d updates, made %d" % (len(wanted), n))

with io.open(PATH, "w", newline="", encoding="utf-8") as fh:
    csv.writer(fh).writerows(rows)

from collections import Counter

c = Counter(r[si] for r in rows[1:])
print("updated %d rows" % n)
print("statuses: %s" % dict(c))
tot = sum(int(r[2]) for r in rows[1:])
ver = sum(int(r[2]) for r in rows[1:] if r[si] == "VERIFIED")
part = sum(int(r[2]) for r in rows[1:] if r[si] == "PARTIAL")
print(
    "LOC verified %d/%d = %.3f%%   verified+partial %.3f%%"
    % (ver, tot, 100.0 * ver / tot, 100.0 * (ver + part) / tot)
)