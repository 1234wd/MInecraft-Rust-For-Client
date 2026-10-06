#!/usr/bin/env python3
"""One-time byte-level repair of net/minecraft/world/phys/Vec2.rs. Session 10.

    python _porting/tools/repair_vec2_bytes.py

APPROVED EXCEPTION to the "edit `.rs` files with the edit tool only" rule, and only for this
one operation, because the edit tool CANNOT OPEN THE FILE: three NUL bytes make git and the
read/edit tools classify it as binary.

WHAT WENT WRONG, from the reviewer's diagnosis
--------------------------------------------
Every one of the six defects sits where a Markdown backtick opened a code span in a doc
comment -- `` `f32::MIN_POSITIVE` `` and `` `0x00800000` ``. In a **PowerShell double-quoted
string the backtick is the escape character**, so a PowerShell write of those comments did:

    `0   ->  \0  ->  NUL byte          (0x00000001, 0x00800000 x2)
    `f   ->  \f  ->  form feed 0x0C   (f32::, x3)
    `l / `M     ->  not an escape      ->  backtick silently DROPPED

That is the whole mechanism. It also vindicates the edit-tool rule: three damages in three
sessions, and the last two shared this cause.

WHAT THIS SCRIPT DOES, AND DELIBERATELY DOES NOT DO
---------------------------------------------------
It replaces exactly two byte values in exactly one file:

    0x00 -> '0'      restoring  0x00000001  and 0x00800000
    0x0C -> 'f'      restoring  f32::MIN_POSITIVE, f32::MIN, f32::EPSILON

and asserts, before writing, that it found exactly 3 of each.

It does NOT restore the backticks. That is the second half of the repair and it needs the edit
tool, which can now read the file. Putting the backticks back from here would be the same class
of blind text surgery that caused this, just with a nicer story.

Reads and writes bytes only. It never decodes and re-encodes, because a decode/encode round trip
is exactly what lost the characters in the first place.
"""
import io
import sys

PATH = "net/minecraft/world/phys/phys_PLACEHOLDER"  # replaced below; kept explicit for clarity
PATH = "net/minecraft/world/phys/Vec2.rs"

NUL, FF = b"\x00", b"\x0c"
EXPECTED_EACH = 3

data = io.open(PATH, "rb").read()
n_nul = data.count(NUL)
n_ff = data.count(FF)

print("%s: %d bytes, %d NUL, %d form feed" % (PATH, len(data), n_nul, n_ff))

if n_nul != EXPECTED_EACH or n_ff != EXPECTED_EACH:
    sys.exit(
        "REFUSING: expected exactly %d NUL and %d form feed, found %d and %d.\n"
        "The counts are the evidence that this is the known damage and not something new.\n"
        "If they differ, stop and look -- do not widen this script."
        % (EXPECTED_EACH, EXPECTED_EACH, n_nul, n_ff)
    )

fixed = data.replace(NUL, b"0").replace(FF, b"f")

assert NUL not in fixed and FF not in fixed, "replacement left a NUL or form feed behind"
# The repair must be byte-for-byte length-preserving, so `git diff` shows only content changes.
assert len(fixed) == len(data), "length changed, which it must not"

io.open(PATH, "wb").write(fixed)
print("repaired: %d NUL -> '0', %d form feed -> 'f'" % (n_nul, n_ff))
print("next step: put the backticks back and drop the duplicate #[test] WITH THE EDIT TOOL.")