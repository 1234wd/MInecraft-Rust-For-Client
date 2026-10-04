#!/usr/bin/env python3
"""Move groups from BLOCKED to COVERED in parity_batch2.rs.

Session-06 loop helper: the coverage guard demands that a group be either claimed by a
test or listed as blocked, so finishing a file means moving its groups between the two
lists. Doing that with a text replace is how stale entries survive, so this edits the
lists structurally and prints exactly what moved.
"""
import io
import re
import sys

PATH = r"_porting\tests\parity_batch2.rs"


def split_list(src, name):
    """Return (start, end, set_of_entries, text) for `const <name>: &[&str] = &[ ... ];`."""
    m = re.search(r"const " + name + r": &\[&str\] = &\[", src)
    if not m:
        sys.exit("could not find const " + name)
    start = m.end()
    end = src.index("];", start)
    body = src[start:end]
    entries = re.findall(r'"([^"]+)"', body)
    return m.start(), end + 2, entries, src[start:end]


def rebuild(entries):
    return "\n".join('    "%s",' % e for e in entries)


def main(move):
    with io.open(PATH, encoding="utf-8") as fh:
        src = fh.read()

    for name, group in (("COVERED", "covered"), ("BLOCKED_ON_UNPORTED_TYPES", "blocked")):
        s, e, entries, _ = split_list(src, name)
        missing = [g for g in move if g not in entries]
        if missing:
            sys.exit("not in %s: %s" % (name, missing))
        kept = [x for x in entries if x not in move]
        src = src[:s] + rebuild(kept) + "\n" + src[e:]
        print("%-26s %d -> %d" % (name, len(entries), len(kept)))

    with io.open(PATH, "w", encoding="utf-8", newline="") as fh:
        fh.write(src)


if __name__ == "__main__":
    main(sys.argv[1:])