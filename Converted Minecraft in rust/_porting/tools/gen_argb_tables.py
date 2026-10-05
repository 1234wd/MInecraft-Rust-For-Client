#!/usr/bin/env python3
"""Generate the embedded sRGB tables for `net/minecraft/util/ARGB.rs` from the golden data.

    python _porting/tools/gen_argb_tables.py

READS:  `_porting/test-data/batch2.txt`, groups `argb.srgbToLinearTable` and
        `argb.linearToSrgbTable`.
WRITES: `_porting/generated/argb_srgb_tables.rs`

WHY A GENERATOR EXISTS FOR THIS, AND WHY IT IS THE RIGHT SHAPE
-----------------------------------------------------------
`ARGB`'s two lookup tables hold 1280 values, and they must be **embedded rather than
recomputed**: the initialisers call `Math.pow`, which is a HotSpot intrinsic (measured in
session 07: 51,268 of 145 million sweep values differ from FdLibm). A pure-Rust recomputation
would be a third unported intrinsic and would be wrong in a way no test could distinguish from
"the table is fine". This is the same `embedded-trig-tables` decision already taken for `Mth`'s
65,536-entry SIN/COS tables.

That leaves two ways to get 1280 numbers into a `.rs` file:

* type them, or
* derive them from the jar.

Typing them is exactly the hand-derived-constant error this project forbids, and 1280 chances to
make it. So they are derived -- from the jar, by the oracle, into the golden, and from the golden
into this generated file. Every step is checked: `parity_batch2::argb_tables_match_the_golden`
compares this file's contents against the golden row by row, so if either drifts, the build fails.

WHY THE TABLES LIVE IN `_porting/` AND NOT IN `ARGB.rs`
------------------------------------------------------
Session 09's rule is that `.rs` files are edited with the edit tool only, and that generated data
may be written by a generator. 1280 values cannot go through the edit tool without being typed, so
they go here and `ARGB.rs` pulls them in with `include!`. The alternative -- pasting the generator
output into `ARGB.rs` by hand -- would reintroduce exactly the transcription risk this file
removes, while looking tidier.

Nothing else in `_porting/generated/` is allowed to be edited by hand either. If a table here is
wrong, fix the oracle and regenerate; do not patch the output.
"""
import io
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.normpath(os.path.join(HERE, "..", ".."))
GOLDEN = os.path.join(ROOT, "_porting", "test-data", "batch2.txt")
OUT = os.path.join(ROOT, "_porting", "generated", "argb_srgb_tables.rs")


def read_group(path, name):
    """Return {index: value} for one golden group, asserting the row shape.

    A dict, not a list, and deliberately order-independent. The first version returned a list and
    then asserted `index == position`, which turned a parser bug into a misleading "the golden is
    not dense" error when the golden was perfectly fine. Keying by index and then checking that
    the key SET is exactly `range(n)` says what is actually true -- every index present exactly
    once -- and does not care what order they arrived in.

    Any line beginning with `#` ends or begins a block, so a comment line inside a group can
    never be parsed as data. These groups take NO arguments, so a row is exactly
    `i32:<index> -> i32:<value>` -- two fields; splitting on the arrow and checking both sides
    keeps the parser honest about the arity instead of counting tokens.
    """
    values = {}
    with io.open(path, encoding="utf-8") as f:
        in_group = False
        for line in f:
            line = line.strip()
            if line.startswith("#"):
                in_group = line.startswith("#fn ") and line.split()[1:2] == [name]
                continue
            if not in_group:
                continue
            if "->" not in line:
                sys.exit("MALFORMED row in group %s (no `->`): %r" % (name, line))
            left, _, right = line.partition("->")
            lf = left.split()
            rf = right.split()
            if len(lf) != 1 or len(rf) != 1:
                sys.exit("MALFORMED row in group %s: expected one arg and one value, got %r"
                         % (name, line))
            if not lf[0].startswith("i32:") or not rf[0].startswith("i32:"):
                sys.exit("MALFORMED row in group %s: expected i32 on both sides: %r" % (name, line))
            # BASE 10, not 16. `f64:` and `i64:` values carry an explicit `0x` prefix and are hex;
            # `i32:` values carry no prefix and are DECIMAL. Parsing them as hex made index 10
            # come out as 16, which surfaced as "the golden is not dense" -- a confident,
            # completely wrong complaint about the data, caused by the parser.
            idx = int(lf[0][4:], 10)
            if idx in values:
                sys.exit("DUPLICATE index %d in group %s" % (idx, name))
            values[idx] = int(rf[0][4:], 10)
    return values
    return rows


def fmt_u16(values, per_line, indent="    "):
    out = []
    for i in range(0, len(values), per_line):
        chunk = values[i:i + per_line]
        out.append(indent + " ".join("%d," % v for v in chunk))
    return "\n".join(out)


def main():
    stl = read_group(GOLDEN, "argb.srgbToLinearTable")
    lts = read_group(GOLDEN, "argb.linearToSrgbTable")

    # The table size is load-bearing: `meanLinear`, `linearChannelMean` and `linearLerp` index
    # LINEAR_TO_SRGB at up to 1023, and srgbToLinearChannel indexes SRGB_TO_LINEAR at up to 255.
    # A silently-short table would be an out-of-bounds read at runtime, so the length is asserted
    # here rather than assumed, and again in Rust by the length checks on the arrays.
    if len(stl) != 256:
        sys.exit("expected 256 SRGB_TO_LINEAR rows, golden has %d" % len(stl))
    if len(lts) != 1024:
        sys.exit("expected 1024 LINEAR_TO_SRGB rows, golden has %d" % len(lts))
    for label, table, n in (("SRGB_TO_LINEAR", stl, 256), ("LINEAR_TO_SRGB", lts, 1024)):
        missing = sorted(set(range(n)) - set(table))
        if missing:
            sys.exit("%s is missing index/indices %s -- the table size is load-bearing"
                     % (label, missing[:8]))
        extra = sorted(set(table) - set(range(n)))
        if extra:
            sys.exit("%s has out-of-range index/indices %s" % (label, extra[:8]))

    stl_vals = [stl[i] for i in range(256)]
    lts_vals = [lts[i] for i in range(1024)]
    if max(stl_vals) > 1023:
        sys.exit("SRGB_TO_LINEAR holds %d, above the 1023 the Java divides by" % max(stl_vals))
    if max(lts_vals) > 255:
        sys.exit("LINEAR_TO_SRGB holds %d, which does not fit the unsigned byte Java masks to"
                 % max(lts_vals))

    header = '''// ============================================================================
// GENERATED FILE -- DO NOT EDIT BY HAND.
//
// Produced by `_porting/tools/gen_argb_tables.py` from
// `_porting/test-data/batch2.txt`, groups `argb.srgbToLinearTable` (256 rows) and
// `argb.linearToSrgbTable` (1024 rows), which the Java oracle reads out of
// `net.minecraft.util.ARGB` by reflection.
//
// WHY EMBEDDED AND NOT COMPUTED: the Java initialisers call `Math.pow`, a HotSpot
// intrinsic, so these values cannot be reproduced by evaluating a formula in Rust without
// adding a third unported intrinsic. They are embedded, exactly as `Mth`'s 65,536-entry
// SIN/COS tables are. See DESIGN_DECISIONS.md (`embedded-trig-tables`).
//
// WHY THE VALUES LIVE HERE AND NOT IN `ARGB.rs`: 1280 of them cannot be typed, and typing
// them is the hand-derived-constant error this project forbids. They are derived from the jar
// instead, and `parity_batch2::argb_tables_match_the_golden` compares this file against the
// golden row by row so neither side can drift.
//
// Field types and lengths confirmed with:
//   javap -p -cp minecraft-merged-deobf-26.2.jar net.minecraft.util.ARGB
//     private static final short[] SRGB_TO_LINEAR;   // 256 entries, values 0..1023
//     private static final byte[]  LINEAR_TO_SRGB;   // 1024 entries, values 0..255
//
// Regenerate:  python _porting/tools/gen_argb_tables.rs's generator, i.e.
//   python _porting/tools/gen_argb_tables.py
// ============================================================================

/// `ARGB.SRGB_TO_LINEAR`, a Java `short[256]`, stored unsigned.
///
/// Java returns `SRGB_TO_LINEAR[srgb] / 1023.0F`, so the values run 0..1023 -- which is why a
/// `short` is enough and why the read divides by 1023 rather than by 255. The array index is an
/// sRGB CHANNEL, so 256 entries.
#[rustfmt::skip]
pub const SRGB_TO_LINEAR: [u16; 256] = [
%s
];

/// `ARGB.LINEAR_TO_SRGB`, a Java `byte[1024]`, stored unsigned.
///
/// Java reads it as `LINEAR_TO_SRGB[Mth.floor(linear * 1023.0F)] & 0xFF`. The `& 0xFF` is
/// because Java's `byte` is signed; the 1024 entries come from the `* 1023.0F` scaling, not
/// from the channel count.
#[rustfmt::skip]
pub const LINEAR_TO_SRGB: [u8; 1024] = [
%s
];
''' % (fmt_u16(stl_vals, 12), fmt_u16(lts_vals, 16))

    os.makedirs(os.path.dirname(OUT), exist_ok=True)
    io.open(OUT, "w", encoding="utf-8", newline="\n").write(header)
    print("wrote %s" % OUT)
    print("  SRGB_TO_LINEAR  %d values, max %d" % (len(stl_vals), max(stl_vals)))
    print("  LINEAR_TO_SRGB  %d values, max %d" % (len(lts_vals), max(lts_vals)))


if __name__ == "__main__":
    main()