#!/usr/bin/env python3
"""api_list.py -- print the real API of a Minecraft class, straight from the jar.

    python _porting/tools/api_list.py net.minecraft.world.phys.Vec3
    python _porting/tools/api_list.py Vec3 --kind method
    python _porting/tools/api_list.py Vec3 --json

READ-ONLY. This script opens the jar with `javap` and prints. It never writes, edits, or
generates anything. That is a deliberate rule, not modesty: two sessions running, a helper
script silently corrupted the very files it was meant to protect (`Oracle.java`'s `--only`
filter matched the jar path and ran nothing; `move_groups.py` ate two `const` headers).
A reporter that cannot write cannot corrupt.

WHY IT EXISTS
-------------
Sessions 02 and 03 carried a checklist of `Mth` method names that turned out to include
two that DO NOT EXIST in 26.2 (`getSeedVec3i`, `lerpVec3`), and session 06 found a fourth
disagreement in the DECOMPILED SOURCE (`Vec3#zRot`'s signs are swapped relative to the
bytecode). Both were caught late, by the compiler or by a golden row.

A method list typed from memory or from a plan document is a guess with the authority of a
checklist. This script makes the jar the only source of member names, so a name in a
document can be CHECKED instead of trusted.

USAGE
-----
    python _porting/tools/api_list.py <fqcn-or-simple-name> [options]

    -k, --kind method|field|all   what to list          (default: all)
    -p, --private                include private members (default: yes)
        --public                 public and protected only
    -s, --static                 static members only
        --no-static              instance members only
        --json                   machine-readable output
        --jar PATH              override the jar location
"""
import argparse
import glob
import json
import os
import re
import subprocess
import sys

# ---------------------------------------------------------------------------
# Locate the jar and a JDK. Both are discovered, never hardcoded.
# ---------------------------------------------------------------------------

DEFAULT_JAR_GLOB = os.path.join(
    os.environ.get("USERPROFILE", ""),
    ".gradle", "caches", "fabric-loom", "minecraftMaven", "net", "minecraft",
    "minecraft-merged-deobf", "*", "minecraft-merged-deobf-*.jar",
)

# Searched in order. The oracle needs JDK 25 (26.2 is class-file v69); any JDK that can
# run `javap` on it works, and javap does not need to match the runtime exactly.
JDK_GLOBS = [
    r"C:\Program Files\Eclipse Adoptium\jdk-25*",
    r"C:\Program Files\Eclipse Adoptium\jdk-21*",
    "/usr/lib/jvm/java-25*",
    "/usr/lib/jvm/java-21*",
    "/usr/lib/jvm/default",
]


def find_jar(override=None):
    if override:
        if not os.path.isfile(override):
            sys.exit("jar not found: " + override)
        return override
    hits = sorted(glob.glob(DEFAULT_JAR_GLOB))
    if not hits:
        sys.exit(
            "could not find the Minecraft jar under\n  " + DEFAULT_JAR_GLOB + "\n"
            "Pass --jar PATH, or run fetch_libs.ps1 / let Gradle populate the cache."
        )
    return hits[-1]


def find_javap():
    from shutil import which

    found = which("javap")
    if found:
        return found
    for pattern in JDK_GLOBS:
        for home in sorted(glob.glob(pattern)):
            for leaf in ("bin/javap.exe", "bin/javap"):
                cand = os.path.join(home, *leaf.split("/"))
                if os.path.isfile(cand):
                    return cand
    sys.exit("no javap found; install a JDK 21+ or put it on PATH")


# ---------------------------------------------------------------------------
# javap
# ---------------------------------------------------------------------------

# A member line looks like:
#   "  public double x;"                        -> field
#   "  public net.minecraft.world.phys.Vec3 add(double);"  -> method
#   "  public static final net.minecraft... ZERO;"         -> static field
# `javap -p` prints the members of the class itself; nested classes are separate javap
# targets, so `$` names are resolved separately by the caller if it wants them.
MODIFIERS = {
    "public", "protected", "private", "static", "final", "abstract", "synchronized",
    "native", "transient", "volatile", "strictfp", "default",
}

# `javap -p` prints one member per line, e.g.
#
#   public net.minecraft.world.phys.Vec3 add(double, double, double);
#   public static final net.minecraft.world.phys.Vec3 ZERO;
#   public net.minecraft.world.phys.Vec2(float, float);          <- constructor
#   private static final int[] SOMETHING;
#
# The only ambiguity is where the modifiers stop and the return type begins, and that is
# decided by a fixed vocabulary: strip the known modifier words off the left and the first
# word left over is the return type. Guessing with a regex instead is what produced
# `void length()` and `? add(float)` in the first version of this script.
NOT_A_MEMBER_PREFIX = ("Compiled from", "class ", "interface ", "enum ", "abstract class",
                       "final class", "public final class", "public class")


def parse(text):
    """Return (methods, fields) parsed from `javap -p` output. Pure; no side effects."""
    methods, fields = [], []
    for raw in text.splitlines():
        line = raw.rstrip()
        if not line.startswith("  "):
            continue
        s = line.strip()
        if not s or not s.endswith(";"):
            continue
        if s.startswith(NOT_A_MEMBER_PREFIX):
            continue
        s = s[:-1]  # drop the ';'

        had_parens = "(" in s
        if had_parens:
            head, _, argtext = s.partition("(")
            args = [a.strip() for a in argtext.rstrip(")").split(",") if a.strip()]
        else:
            head, args = s, []

        words = head.split()
        if len(words) < 2:
            continue
        name = words[-1]
        rest = words[:-1]
        mods = []
        while rest and rest[0] in MODIFIERS:
            mods.append(rest.pop(0))
        ret = " ".join(rest) if rest else "void"   # no return type == constructor
        is_ctor = not rest
        entry = {
            "name": name,
            "type": "void" if is_ctor else ret,
            "ctor": is_ctor,
            "args": args,
            "static": "static" in mods,
            "mods": " ".join(mods),
        }
        # Classify by whether the line had parentheses, NOT by whether `args` is
        # non-empty: `public int hashCode();` has none and is still a method. Classifying
        # on `args` filed hashCode, length, lengthSquared, negated and normalized under
        # "fields" in the first version of this script.
        (methods if had_parens else fields).append(entry)
    return methods, fields


# Compiler-generated members. They are in the bytecode but NOT in the source, so a member
# list built from them would name things a porter can never call and can never be asked to
# port: `lambda$static$0` is a lambda body, `access$000` is a synthetic accessor, and
# `static void {}()` is javap's rendering of a static initialiser's marker.
SYNTHETIC_PREFIXES = ("lambda$", "access$", "this$")


def is_synthetic(name):
    return name.startswith(SYNTHETIC_PREFIXES) or name in ("{}", "<clinit>")


def javap(javap_exe, jar, cls):
    out = subprocess.run(
        [javap_exe, "-p", "-cp", jar, cls],
        capture_output=True, text=True,
    )
    if out.returncode != 0:
        sys.exit("javap failed for %s:\n%s" % (cls, out.stderr.strip()))
    return out.stdout


def main():
    ap = argparse.ArgumentParser(
        description="Print a Minecraft class's real API from the jar. READ-ONLY."
    )
    ap.add_argument("cls", help="fully-qualified class name, or a unique simple name")
    ap.add_argument("-k", "--kind", choices=["method", "field", "all"], default="all")
    ap.add_argument("-p", "--private", dest="vis", action="store_const", const="all",
                    default="all", help="include private members (default)")
    ap.add_argument("--public", dest="vis", action="store_const", const="public",
                    help="public and protected only")
    ap.add_argument("-s", "--static", dest="static", action="store_const", const="only",
                    default="all", help="static members only")
    ap.add_argument("--no-static", dest="static", action="store_const", const="no",
                    help="instance members only")
    ap.add_argument("--json", action="store_true")
    ap.add_argument("--jar", help="override the jar path")
    args = ap.parse_args()

    jar = find_jar(args.jar)
    javap_exe = find_javap()

    # Resolve a simple name by scanning the jar's index for classes ending in it.
    name = args.cls
    if "." not in name:
        import zipfile

        with zipfile.ZipFile(jar) as z:
            suffix = "/" + name + ".class"
            hits = [n[:-6].replace("/", ".") for n in z.namelist() if n.endswith(suffix)]
        if not hits:
            sys.exit("no class named %s in the jar" % name)
        if len(hits) > 1:
            sys.exit(
                "%s is ambiguous (%d classes):\n  %s"
                % (name, len(hits), "\n  ".join(hits[:20]))
            )
        name = hits[0]

    methods, fields = parse(javap(javap_exe, jar, name))
    synth = len(methods) + len(fields)
    methods = [m for m in methods if not is_synthetic(m["name"])]
    fields = [f for f in fields if not is_synthetic(f["name"])]
    synth -= len(methods) + len(fields)

    if args.static == "only":
        methods = [m for m in methods if m["static"]]
        fields = [f for f in fields if f["static"]]
    elif args.static == "no":
        methods = [m for m in methods if not m["static"]]
        fields = [f for f in fields if not f["static"]]
    if args.vis == "public":
        methods = [m for m in methods if "public" in m["mods"] or "protected" in m["mods"]]
        fields = [f for f in fields if "public" in f["mods"] or "protected" in f["mods"]]

    total_methods = len(methods)
    total_fields = len(fields)

    if args.json:
        print(json.dumps(
            {"class": name, "jar": jar,
             "methods": methods, "fields": fields,
             "method_count": total_methods, "field_count": total_fields},
            indent=2, sort_keys=True))
        return

    print("class   : %s" % name)
    print("jar     : %s" % jar)
    print("javap   : %s" % javap_exe)
    print("methods : %d      fields: %d      (synthetic members hidden: %d)"
          % (total_methods, total_fields, synth))
    print()
    if args.kind in ("method", "all"):
        print("-- methods (%d) --" % total_methods)
        for m in sorted(methods, key=lambda m: (m["ctor"], m["name"], m["args"])):
            # A constructor's "return type" is really its class name; print it as a
            # constructor so the line reads like the Java source.
            head = "<init>" if m["ctor"] else m["type"]
            print("  %s%s %s(%s)" % (
                "static " if m["static"] else "",
                head, m["name"], ", ".join(m["args"]),
            ))
        print()
    if args.kind in ("field", "all"):
        print("-- fields (%d) --" % total_fields)
        for f in sorted(fields, key=lambda f: f["name"]):
            print("  %s%s %s" % ("static " if f["static"] else "", f["type"], f["name"]))


if __name__ == "__main__":
    main()