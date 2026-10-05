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
    """Return (start, end, entries) for `const <name>: &[&str] = &[ ... ];`.

    The `];` must be found by SCANNING FORWARD, not by `src.index("];", start)`: if the
    list is damaged (or a later list is missing its `const` header) `index` happily finds
    the NEXT list's terminator and this function silently merges two lists. That is exactly
    how `const BLOCKED_ON_UNPORTED_TYPES` got eaten in session 06.
    """
    m = re.search(r"^const " + name + r": &\[&str\] = &\[", src, re.M)
    if not m:
        sys.exit("could not find the `const " + name + ": &[&str] = &[` header")
    start = m.end()
    end = src.find("];", start)
    if end < 0:
        sys.exit("const " + name + " has no closing `];` -- the list is damaged")
    body = src[start:end]
    entries = re.findall(r'"([^"]+)"', body)
    if not entries:
        sys.exit("const " + name + " is EMPTY -- refusing to rewrite it")
    # Sanity: nothing but whitespace and entries between the brackets.
    residue = re.sub(r'"[^"]+"', "", body)
    if residue.strip():
        sys.exit(
            "const " + name + " has unexpected content between entries: %r" % residue[:120]
        )
    return m.start(), end + 2, entries


def rebuild(entries):
    return "\n".join('    "%s",' % e for e in entries)


def main(move):
    with io.open(PATH, encoding="utf-8") as fh:
        src = fh.read()

    # COVERED: add the groups (each now has a test).
    s, e, entries, _ = split_list(src, "COVERED")
    added = [g for g in move if g not in entries]
    kept = entries + added
    src = src[:s] + rebuild(kept) + chr(10) + src[e:]
    print("COVERED                  %d -> %d (+%d)" % (len(entries), len(kept), len(added)))

    # BLOCKED: remove them, and insist they were actually listed -- a typo here would
    # leave a group claimed-as-blocked while a test also claims it.
    s, e, entries, _ = split_list(src, "BLOCKED_ON_UNPORTED_TYPES")
    missing = [g for g in move if g not in entries]
    if missing:
        sys.exit("not in BLOCKED_ON_UNPORTED_TYPES: %s" % missing)
    kept = [x for x in entries if x not in move]
    src = src[:s] + rebuild(kept) + chr(10) + src[e:]
    print("BLOCKED_ON_UNPORTED_TYPES %d -> %d" % (len(entries), len(kept)))

    with io.open(PATH, "w", encoding="utf-8", newline="") as fh:
        fh.write(src)


if __name__ == "__main__":
    main(sys.argv[1:])