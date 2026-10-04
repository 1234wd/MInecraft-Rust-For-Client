# =============================================================================
# Java oracle build + run.
#
#   pwsh -File "Converted Minecraft in rust/_porting/java-oracle/run.ps1"
#   pwsh -File ".../run.ps1" -Mode Source -OutDir ../test-data-src
#
# NOTHING is ever written into minecraft-decompiled/.
#
# ---------------------------------------------------------------------------
# THE REAL MINECRAFT JAR IS GROUND TRUTH
# ---------------------------------------------------------------------------
# Session 03 recompiled the decompiled sources and treated them as authoritative. That
# was wrong in principle, and it already had a concrete cost: `Util.java` does not
# compile under JDK 25 at all (a decompiler artifact), and the only options were to
# patch a copy or leave the class untestable.
#
# So as of session 04 there are two modes and the JAR WINS:
#
#   Mode Jar     (DEFAULT, and the only mode that produces golden data)
#                Nothing from minecraft-decompiled/ is compiled. Every game class is
#                loaded from minecraft-merged-deobf-26.2.jar -- Mojang's own bytecode,
#                the thing we are actually porting behaviour from. `--expect-origin`
#                then FAILS THE BUILD if any tracked class did not come from the jar,
#                so a stale `classes/` entry cannot quietly shadow it.
#
#   Mode Source  (cross-check only; never used to generate golden data)
#                Additionally compiles the decompiled sources and puts them first on
#                the classpath. Used to diff source-vs-jar behaviour. Any row that
#                differs is a DECOMPILER ARTIFACT and is recorded in
#                _porting/DESIGN_DECISIONS.md under #decompiler-artifacts.
#
# Rule for the whole port: if the decompiled source and the jar bytecode ever disagree,
# THE JAR IS CORRECT. Port the jar's behaviour, and log it.
#
# ---------------------------------------------------------------------------
# WHY THERE ARE NO SOURCE STUBS
# ---------------------------------------------------------------------------
# Session 02 needed 17 hand-written stubs. All 17 are gone: the third-party ones are
# real jars at the versions 26.2 resolves, and the Minecraft value types come from the
# Minecraft jar. `stubs/` stays empty and the check below fails loudly if anything
# reappears -- a stub that shadows a real class is the single most dangerous way for
# this harness to lie to us.
# =============================================================================
param(
    # Which side of the source-vs-jar comparison to run.
    [ValidateSet('Jar', 'Source')]
    [string] $Mode = 'Jar',

    # Where to write the golden files. Defaults to ../test-data. Pass a different
    # directory to produce the Source-mode output for a diff.
    [string] $OutDir = '',

    # Skip writing; used by the cross-check when you only want the exit code.
    [switch] $NoWrite
)

$ErrorActionPreference = 'Stop'

# ---------------------------------------------------------------------------
# JDK 25 is REQUIRED.
#
# The 26.2 jar is class-file version 69.0 (Java 25). JDK 21's javac caps at 65.0 and
# reports that as a cascade of `cannot access FriendlyByteBuf` on ordinary imports --
# which reads like a missing dependency rather than a wrong JDK.
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

$Here    = Split-Path -Parent $MyInvocation.MyCommand.Path
$Porting = Split-Path -Parent $Here
$Rust    = Split-Path -Parent $Porting
$Repo    = Split-Path -Parent $Rust
$McSrc   = Join-Path $Repo 'minecraft-decompiled'

if (-not (Test-Path $McSrc)) { throw "minecraft-decompiled/ not found at $McSrc" }

# The Loom-remapped Minecraft 26.2 jar. This is the artifact we are porting.
$McJar = Join-Path $env:USERPROFILE '.gradle\caches\fabric-loom\minecraftMaven\net\minecraft\minecraft-merged-deobf\26.2\minecraft-merged-deobf-26.2.jar'
if (-not (Test-Path $McJar)) {
    throw "mapped Minecraft jar not found at $McJar`n" +
          'It comes from the Fabric Loom cache. Run the Loom build once to populate it.'
}

# The decompiled sources, used only in Source mode. Listed explicitly so javac can
# never discover anything else: minecraft-decompiled/ is deliberately NOT on the
# source path.
$Vanilla = @(
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
    'net/minecraft/core/Vec3i.java',
    'net/minecraft/core/BlockPos.java',
    'net/minecraft/core/Direction.java',
    'net/minecraft/world/level/ChunkPos.java',
    'net/minecraft/core/SectionPos.java',
    'net/minecraft/world/phys/Vec3.java',
    'net/minecraft/world/phys/AABB.java',
    'net/minecraft/util/ARGB.java',
    'net/minecraft/resources/Identifier.java',
    'net/minecraft/core/Rotations.java'
)

$Classes  = Join-Path $Here 'classes'
if (-not $OutDir) { $OutDir = Join-Path $Porting 'test-data' }

Write-Host "==> mode: $Mode   (jar = ground truth)"
Write-Host "    goldens -> $OutDir"
Write-Host '==> cleaning'
Remove-Item -Recurse -Force $Classes -ErrorAction SilentlyContinue
New-Item -ItemType Directory -Force -Path $Classes  | Out-Null
New-Item -ItemType Directory -Force -Path $OutDir   | Out-Null

# --- the stubs/ tree must stay empty ---------------------------------------
$stubFiles = @(Get-ChildItem -Recurse -Filter '*.java' (Join-Path $Here 'stubs') -ErrorAction SilentlyContinue)
if ($stubFiles.Count -gt 0) {
    Write-Host "FAIL: $($stubFiles.Count) stub source(s) present:"
    $stubFiles | ForEach-Object { "    $($_.FullName)" }
    Write-Host 'Use the real library at its pinned version, or the real Minecraft jar.'
    exit 1
}

$files = New-Object System.Collections.Generic.List[string]
foreach ($s in (Get-ChildItem -Recurse -Filter '*.java' (Join-Path $Here 'src'))) { $files.Add($s.FullName) }

$compiledVanilla = 0
if ($Mode -eq 'Source') {
    foreach ($v in $Vanilla) {
        $p = Join-Path $McSrc $v
        if (-not (Test-Path $p)) { throw "missing vanilla source: $v" }
        $files.Add($p)
        $compiledVanilla++
    }
}

$jars = @($McJar) + @((Get-ChildItem -Filter '*.jar' (Join-Path $Here 'lib') | ForEach-Object { $_.FullName }))
if ($jars.Count -lt 2) { throw 'no jars in ./lib -- run fetch_libs.ps1 first' }

# CLASSPATH ORDER IS THE WHOLE POINT OF THE MODE SWITCH.
#   Jar    -> the Minecraft jar first, and `classes/` holds only oracle code, so no
#             game class can come from anywhere else.
#   Source -> `classes/` first, so the recompiled decompiled sources shadow the jar.
$cp = if ($Mode -eq 'Source') {
    (@($Classes) + $jars) -join ';'
} else {
    $jars -join ';'
}

Write-Host "==> javac  (oracle sources + $compiledVanilla vanilla, jars: $($jars.Count))"
& javac -nowarn -Xlint:none -encoding UTF-8 -d $Classes -cp $cp `
    -sourcepath (Join-Path $Here 'src') `
    -implicit:none -Xmaxerrs 40 $files
if ($LASTEXITCODE -ne 0) { throw "javac failed ($LASTEXITCODE)" }

# Prove every tracked class came from where this mode says it should. In Jar mode a
# class coming from `classes/` would mean we are testing our own recompilation rather
# than Mojang's bytecode -- the exact mistake this mode exists to prevent.
$expectFrom = if ($Mode -eq 'Source') { $Classes } else { $McJar }
Write-Host '==> verifying class origins'
if ($NoWrite) {
    $tmp = Join-Path $env:TEMP ("oracle-nowrite-" + [guid]::NewGuid().ToString('N'))
    New-Item -ItemType Directory -Force -Path $tmp | Out-Null
    & java -cp "$Classes;$cp" oracle.Oracle $tmp --expect-origin $expectFrom --no-write
    Remove-Item -Recurse -Force $tmp -ErrorAction SilentlyContinue
} else {
    & java -cp "$Classes;$cp" oracle.Oracle $OutDir --expect-origin $expectFrom
}
if ($LASTEXITCODE -ne 0) { throw "oracle failed ($LASTEXITCODE)" }

Write-Host '==> done'
Get-ChildItem $OutDir -Filter *.txt | ForEach-Object { "    {0,-14} {1,10} bytes" -f $_.Name, $_.Length }
