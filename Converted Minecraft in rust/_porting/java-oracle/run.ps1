# =============================================================================
# Java oracle build + run.
#
#   Compiles the ORIGINAL sources from minecraft-decompiled/ (unmodified) together
#   with the pinned third-party jars in ./lib, then regenerates the golden data in
#   ../test-data/.
#
#   Nothing is ever written into minecraft-decompiled/.
#
#   Usage:  pwsh -File "Converted Minecraft in rust/_porting/java-oracle/run.ps1"
#
# ---------------------------------------------------------------------------
# WHY THERE ARE NO SOURCE STUBS
# ---------------------------------------------------------------------------
# Session 02 compiled against 17 hand-written stubs. As of session 03 every one of
# them is gone: the third-party ones are satisfied by real jars at the versions
# Minecraft 26.2 actually resolves (see fetch_libs.ps1), and the Minecraft value
# types are compiled from minecraft-decompiled/ like everything else.
#
# The oracle now has ZERO stubs. `stubs/` is kept as an empty directory so the
# layout stays stable, and the check below fails loudly if anything reappears --
# a stub that shadows a real class is the single most dangerous way for this
# harness to lie to us.
#
# Because the genuine Vec3i/BlockPos/Vec3/AABB/Identifier/ARGB are compiled, their
# whole dependency graph has to come along. That is why the list below is explicit:
# we name every file, and javac is forbidden from finding anything else.
# =============================================================================
$ErrorActionPreference = 'Stop'

# ---------------------------------------------------------------------------
# JDK 25 is REQUIRED, not optional.
#
# Minecraft 26.2 ships as class-file version 69.0 (Java 25). JDK 21's javac can
# only read up to 65.0 and fails with the deeply unhelpful
#
#     bad class file: .../minecraft-merged-deobf-26.2.jar(/net/minecraft/network/FriendlyByteBuf.class)
#           class file has wrong version 69.0, should be 65.0
#
# which surfaces as a cascade of "cannot access FriendlyByteBuf" / "cannot access
# StreamCodec" on perfectly ordinary imports. That cascade is what a wrong JDK
# looks like; it is not a missing dependency.
#
# Override with $env:ORACLE_JDK if the install path differs.
# ---------------------------------------------------------------------------
if ($env:ORACLE_JDK) {
    $jdk = $env:ORACLE_JDK
} else {
    $adoptium = Join-Path $env:ProgramFiles 'Eclipse Adoptium'
    $jdk = Get-ChildItem $adoptium -Directory -ErrorAction SilentlyContinue |
           Where-Object { $_.Name -match '^jdk-25' } |
           Sort-Object Name -Descending | Select-Object -First 1 -ExpandProperty FullName
    if (-not $jdk) { throw "no JDK 25 found under $adoptium -- set `$env:ORACLE_JDK" }
}
$env:JAVA_HOME = $jdk
$env:PATH = (Join-Path $jdk 'bin') + ';' + $env:PATH
$javacVersion = (& javac -version 2>&1) -join ''
if ($javacVersion -notmatch '^javac 25\.') {
    throw "need javac 25 (found `"$javacVersion`" from $jdk)"
}
Write-Host "==> using $($javacVersion.Trim())  ($jdk)"

$Here    = Split-Path -Parent $MyInvocation.MyCommand.Path
$Porting = Split-Path -Parent $Here
$Rust    = Split-Path -Parent $Porting
$Repo    = Split-Path -Parent $Rust
$McSrc   = Join-Path $Repo 'minecraft-decompiled'

if (-not (Test-Path $McSrc)) { throw "minecraft-decompiled/ not found at $McSrc" }

# ---------------------------------------------------------------------------
# The exact set of vanilla classes this oracle compiles, in dependency order.
#
# Order matters only for readability (javac resolves regardless), but keeping it
# topological makes it obvious when a file is missing a dependency: add the new
# file ABOVE its dependents, not below.
#
# Session 02: util + RNG.
# Session 03: core value types, which is why Util/ThreadingDetector/ByIdMap/
#             StringRepresentable/Position had to come in.
# ---------------------------------------------------------------------------
$Vanilla = @(
    # --- session 02: util + RNG ---------------------------------------------
    'net/minecraft/util/Mth.java',
    'net/minecraft/util/RandomSource.java',
    'net/minecraft/util/LinearCongruentialGenerator.java',
    'net/minecraft/world/level/levelgen/BitRandomSource.java',
    'net/minecraft/world/level/levelgen/MarsagliaPolarGaussian.java',
    'net/minecraft/world/level/levelgen/PositionalRandomFactory.java',
    'net/minecraft/world/level/levelgen/RandomSupport.java',
    'net/minecraft/world/level/levelgen/LegacyRandomSource.java',
    'net/minecraft/world/level/levelgen/SingleThreadedRandomSource.java',
    'net/minecraft/world/level/levelgen/ThreadSafeLegacyRandomSource.java',
    'net/minecraft/world/level/levelgen/Xoroshiro128PlusPlus.java',
    'net/minecraft/world/level/levelgen/XoroshiroRandomSource.java',
    'net/minecraft/world/level/levelgen/WorldgenRandom.java',
    # --- session 03: core value types --------------------------------------
    'net/minecraft/core/Vec3i.java',
    'net/minecraft/core/BlockPos.java',
    'net/minecraft/core/Direction.java',
    'net/minecraft/world/level/ChunkPos.java',
    'net/minecraft/world/phys/Vec3.java',
    'net/minecraft/world/phys/AABB.java',
    'net/minecraft/util/ARGB.java',
    'net/minecraft/resources/Identifier.java'
)

$Classes  = Join-Path $Here 'classes'
$TestData = Join-Path $Porting 'test-data'

Write-Host '==> cleaning'
Remove-Item -Recurse -Force $Classes -ErrorAction SilentlyContinue
New-Item -ItemType Directory -Force -Path $Classes  | Out-Null
New-Item -ItemType Directory -Force -Path $TestData | Out-Null

# --- the stubs/ tree must stay empty ---------------------------------------
# A stub that shadows a real class would make the oracle report vanilla
# behaviour for something that is actually our own approximation.
$stubFiles = @(Get-ChildItem -Recurse -Filter '*.java' (Join-Path $Here 'stubs') -ErrorAction SilentlyContinue)
if ($stubFiles.Count -gt 0) {
    Write-Host "FAIL: $($stubFiles.Count) stub source(s) reappeared:"
    $stubFiles | ForEach-Object { "    $($_.FullName)" }
    Write-Host 'Add the real library to ./lib (pinned version) or compile the real'
    Write-Host 'vanilla source instead. Never stub a class that has a real version.'
    exit 1
}

$files = New-Object System.Collections.Generic.List[string]

# (1) every oracle source
foreach ($s in (Get-ChildItem -Recurse -Filter '*.java' (Join-Path $Here 'src'))) { $files.Add($s.FullName) }

# (2) the ORIGINAL vanilla sources, passed EXPLICITLY so javac can never discover
#     anything on a source path. minecraft-decompiled/ is deliberately NOT on the
#     source path: if it were, javac would silently pull in classes we did not
#     review, and the "these files are unmodified" claim would become unverifiable.
#
#     The list above contains ONLY the classes this port is actually testing.
#     Everything else they merely reference -- Util, ThreadingDetector, ByIdMap,
#     StringRepresentable, Position, ResourceKey, StreamCodec, Component, Entity,
#     BlockHitResult, BoundingBox -- is resolved from the real Minecraft jar.
#
#     That is a deliberate trade. Using the jar means we are not testing our reading
#     of those helper classes, but they are only reached for trivial delegation
#     (Util.make, ByIdMap construction) and the jar is the genuine article rather
#     than an approximation we wrote. Compiling Util.java from source, by contrast,
#     fails: the decompiler emits `for (K key : (Enum[])keyType.getEnumConstants())`
#     which JDK 25's javac rejects with "Enum cannot be converted to K". Patching
#     that would mean no longer compiling an unmodified file.
$vanillaFiles = New-Object System.Collections.Generic.List[string]
foreach ($v in $Vanilla) {
    $p = Join-Path $McSrc $v
    if (-not (Test-Path $p)) { throw "missing vanilla source: $v" }
    $vanillaFiles.Add($p)
    $files.Add($p)
}

$jars = (Get-ChildItem -Filter '*.jar' (Join-Path $Here 'lib') | ForEach-Object { $_.FullName })
if (-not $jars) { throw 'no jars in ./lib -- run fetch_libs.ps1 first' }

# ---------------------------------------------------------------------------
# The real, Loom-remapped Minecraft 26.2 jar.
#
# The classes we actually test are compiled from minecraft-decompiled/ and listed
# EXPLICITLY above, so they are compiled from source and take precedence. This jar
# supplies every OTHER Minecraft type they merely reference -- StreamCodec,
# ByteBufCodecs, Component, Block, BoundingBox, LevelHeightAccessor, ...
#
# This matters a lot. Without it we would have to stub each of those, and a stub
# on a golden path is exactly the failure mode this harness exists to prevent: the
# oracle would report "vanilla behaviour" for something that is our own guess.
#
# It also means adding a class to the list above is the ONLY way to make the oracle
# test OUR reading of that class, so the list stays a deliberate, reviewable set.
# ---------------------------------------------------------------------------
$McJar = Join-Path $env:USERPROFILE '.gradle\caches\fabric-loom\minecraftMaven\net\minecraft\minecraft-merged-deobf\26.2\minecraft-merged-deobf-26.2.jar'
if (-not (Test-Path $McJar)) {
    throw "mapped Minecraft jar not found at $McJar`n" +
          'It comes from the Fabric Loom cache. Run the Loom build once to populate it,' +
          'or point fetch_libs.ps1 at your own copy.'
}
$jars += $McJar
$cp = ($jars -join ';')

Write-Host "==> javac  (vanilla: $($vanillaFiles.Count), stubs: 0, jars: $($jars.Count), +minecraft jar)"
& javac -nowarn -Xlint:none -encoding UTF-8 -d $Classes -cp $cp `
    -sourcepath (Join-Path $Here 'src') `
    -implicit:none -Xmaxerrs 40 $files
if ($LASTEXITCODE -ne 0) { throw "javac failed ($LASTEXITCODE)" }

# Prove the oracle linked against the ORIGINAL vanilla classes rather than the
# prebuilt jar: every class we compiled from source must report the classes/
# directory as its CodeSource. Anything else means the jar shadowed our source and
# we are testing Mojang's bytecode instead of the decompiled text we port from.
Write-Host '==> verifying class origins'
& java -cp "$Classes;$cp" oracle.Oracle $TestData --print-origins --require-origins $Classes
if ($LASTEXITCODE -ne 0) { throw "oracle failed ($LASTEXITCODE)" }

Write-Host '==> done'
Get-ChildItem $TestData | ForEach-Object { "    {0,-14} {1,10} bytes" -f $_.Name, $_.Length }
