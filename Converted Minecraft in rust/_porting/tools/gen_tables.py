#!/usr/bin/env python3
"""
gen_tables.py -- splice the lookup tables dumped by the Java oracle into Mth.rs.

    python "Converted Minecraft in rust/_porting/tools/gen_tables.py"

Reads   : "Converted Minecraft in rust/_porting/test-data/mth_tables.txt"
Writes  : the region between the BEGIN/END GENERATED TABLES markers in
          "Converted Minecraft in rust/net/minecraft/util/Mth.rs"

Why embed instead of calling the host libm -- see DESIGN_DECISIONS.md
(#embedded-trig-tables). Short version: Java's Math.sin may be 1 ulp off and
Rust's f64::sin links whatever libm the OS has; one flipped f32 entry in the
65536-entry table changes every sin()/cos() call that lands on it.

Idempotent: running it twice changes nothing.
"""

from __future__ import annotations

import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
RUST_ROOT = os.path.abspath(os.path.join(HERE, os.pardir, os.pardir))
TABLES = os.path.join(RUST_ROOT, "_porting", "test-data", "mth_tables.txt")
TARGET = os.path.join(RUST_ROOT, "net", "minecraft", "util", "Mth.rs")

BEGIN = "// >>> BEGIN GENERATED TABLES (regenerate with _porting/tools/gen_tables.py)"
END = "// <<< END GENERATED TABLES"


def load() -> dict[str, list[str]]:
    groups: dict[str, list[str]] = {}
    current = None
    with open(TABLES, "r", encoding="utf-8") as fh:
        for raw in fh:
            line = raw.rstrip("\r\n")
            if not line.strip():
                continue
            if line.startswith("#fn "):
                current = line[4:].split()[0]
                groups[current] = []
                continue
            if line.startswith("#"):
                continue
            if current is None:
                continue
            _, value = line.split(" -> ", 1)
            groups[current].append(value.split(":", 1)[1])
    return groups


def f32_array(name: str, values: list[str], per_line: int = 8) -> str:
    out = [f"pub static {name}: [f32; {len(values)}] = ["]
    for i in range(0, len(values), per_line):
        row = ", ".join(f"f32::from_bits({v})" for v in values[i:i + per_line])
        out.append(f"    {row},")
    out.append("];")
    return "\n".join(out)


def f64_array(name: str, values: list[str], per_line: int = 4) -> str:
    out = [f"pub static {name}: [f64; {len(values)}] = ["]
    for i in range(0, len(values), per_line):
        row = ", ".join(f"f64::from_bits({v})" for v in values[i:i + per_line])
        out.append(f"    {row},")
    out.append("];")
    return "\n".join(out)


def main() -> int:
    if not os.path.isfile(TABLES):
        print(f"FATAL: missing {TABLES}\nRun _porting/java-oracle/run.ps1 first.", file=sys.stderr)
        return 2
    if not os.path.isfile(TARGET):
        print(f"FATAL: missing {TARGET}", file=sys.stderr)
        return 2

    groups = load()
    for required, count in (("SIN", 65536), ("ASIN_TAB", 257), ("COS_TAB", 257)):
        if len(groups.get(required, [])) != count:
            print(f"FATAL: {required} has {len(groups.get(required, []))} entries, expected {count}", file=sys.stderr)
            return 2

    block = "\n".join([
        BEGIN,
        "//",
        "// Dumped verbatim from the ORIGINAL net.minecraft.util.Mth by",
        "// _porting/java-oracle and spliced in by _porting/tools/gen_tables.py.",
        "// Do NOT hand-edit: `cargo test` re-checks the whole 65536-entry table",
        "// against a golden FNV-1a hash, so a typo cannot survive.",
        "//",
        "// Mth's own SIN_SCALE / SIN_QUANTIZATION / COS_OFFSET constants are",
        "// reproduced by hand above; these arrays are the precomputed result.",
        "",
        "/// Port of `Mth.SIN`: `sin[i] = (float) Math.sin(i / 10430.378350470453)`.",
        f32_array("SIN", groups["SIN"]),
        "",
        "/// Port of `Mth.ASIN_TAB`, filled by Mth's static initialiser.",
        f64_array("ASIN_TAB", groups["ASIN_TAB"]),
        "",
        "/// Port of `Mth.COS_TAB`, filled by Mth's static initialiser.",
        f64_array("COS_TAB", groups["COS_TAB"]),
        END,
    ])

    with open(TARGET, "r", encoding="utf-8") as fh:
        text = fh.read()

    start = text.find(BEGIN)
    stop = text.find(END)
    if start < 0 or stop < 0:
        print(f"FATAL: {TARGET} is missing the GENERATED TABLES markers", file=sys.stderr)
        return 2
    new_text = text[:start] + block + text[stop + len(END):]

    if new_text == text:
        print("Mth.rs tables already up to date.")
        return 0

    with open(TARGET, "w", encoding="utf-8", newline="\n") as fh:
        fh.write(new_text)
    print(f"spliced {len(groups['SIN'])} SIN + {len(groups['ASIN_TAB'])} ASIN + "
          f"{len(groups['COS_TAB'])} COS entries into Mth.rs "
          f"({os.path.getsize(TARGET)} bytes)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())