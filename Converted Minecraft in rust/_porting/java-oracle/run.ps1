# =============================================================================
# Java oracle build + run.
#
#   Compiles the ORIGINAL sources from minecraft-decompiled/ (unmodified) together
#   with the third-party jars in ./lib and the minimal stubs in ./stubs, then
#   regenerates the golden data in ../test-data/.
#
#   Nothing is ever written into minecraft-decompiled/.
#
#   Usage:  pwsh -File "Converted Minecraft in rust/_porting/java-oracle/run.ps1"
# =============================================================================
$ErrorActionPreference = 'Stop'

$Here    = Split-Path -Parent $MyInvocation.MyCommand.Path
$Porting = Split-Path -Parent $Here
$Rust    = Split-Path -Parent $Porting
$Repo    = Split-Path -Parent $Rust
$McSrc   = Join-Path $Repo 'minecraft-decompiled'

if (-not (Test-Path $McSrc)) { throw "minecraft-decompiled/ not found at $McSrc" }

# The exact set of vanilla classes this oracle needs, in dependency order.
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
    'net/minecraft/world/level/levelgen/WorldgenRandom.java'
)

$Classes = Join-Path $Here 'classes'
$TestData = Join-Path $Porting 'test-data'

Write-Host '==> cleaning'
Remove-Item -Recurse -Force $Classes -ErrorAction SilentlyContinue
New-Item -ItemType Directory -Force -Path $Classes  | Out-Null
New-Item -ItemType Directory -Force -Path $TestData | Out-Null

$files = New-Object System.Collections.Generic.List[string]

# (1) every stub
foreach ($s in (Get-ChildItem -Recurse -Filter '*.java' (Join-Path $Here 'stubs'))) { $files.Add($s.FullName) }
$stubCount = $files.Count

# (2) every oracle source
foreach ($s in (Get-ChildItem -Recurse -Filter '*.java' (Join-Path $Here 'src'))) { $files.Add($s.FullName) }

# (3) the ORIGINAL vanilla sources, passed explicitly so javac never has to
#     discover them on a source path (minecraft-decompiled/ is deliberately NOT
#     on the source path -- otherwise javac would drag in the real Vec3i,
#     BlockPos, ... and their entire dependency graph instead of our stubs).
$vanillaFiles = New-Object System.Collections.Generic.List[string]
foreach ($v in $Vanilla) {
    $p = Join-Path $McSrc $v
    if (-not (Test-Path $p)) { throw "missing vanilla source: $v" }
    $vanillaFiles.Add($p)
    $files.Add($p)
}

$jars = (Get-ChildItem -Filter '*.jar' (Join-Path $Here 'lib') | ForEach-Object { $_.FullName })
if (-not $jars) { throw 'no jars in ./lib -- run fetch_libs.ps1 first' }
$cp = ($jars -join ';')

Write-Host "==> javac  (vanilla: $($vanillaFiles.Count), stubs: $stubCount, jars: $($jars.Count))"
& javac -nowarn -Xlint:none -encoding UTF-8 -d $Classes -cp $cp `
    -sourcepath ((Join-Path $Here 'stubs') + ';' + (Join-Path $Here 'src')) `
    -implicit:none -Xmaxerrs 40 $files
if ($LASTEXITCODE -ne 0) { throw "javac failed ($LASTEXITCODE)" }

# Prove the oracle really linked against the ORIGINAL vanilla classes and not
# against a stale copy: print where each class was loaded from at runtime.
Write-Host '==> verifying class origins'
& java -cp "$Classes;$cp" oracle.Oracle $TestData --print-origins
if ($LASTEXITCODE -ne 0) { throw "oracle failed ($LASTEXITCODE)" }

Write-Host '==> done'
Get-ChildItem $TestData | ForEach-Object { "    {0,-14} {1,10} bytes" -f $_.Name, $_.Length }