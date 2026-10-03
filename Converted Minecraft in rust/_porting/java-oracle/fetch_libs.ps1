# =============================================================================
# Fetches the two third-party jars the Java oracle needs into ./lib.
#
#   pwsh -File "Converted Minecraft in rust/_porting/java-oracle/fetch_libs.ps1"
#
# The jars are committed to the repo so the oracle is reproducible offline.
# They are NOT game assets and contain no Mojang code.
#
#   joml          -- org.joml.Math.invsqrt / Quaternionf / Vector3f used by Mth.
#                   The invsqrt routines are version-specific bit tricks, so we
#                   compile against the real library rather than guess.
#   commons-lang3 -- Mth.getInt(String,int) and Mth.mulAndTruncate(Fraction,int).
#
# Everything else (Guava's MD5/Longs, the DFU Codec API, jspecify, netty's
# ThreadLocalRandom and a handful of tiny Minecraft value types) is stubbed from
# source in ./stubs -- see ../DESIGN_DECISIONS.md (#stubs).
# =============================================================================
$ErrorActionPreference = 'Stop'
$Lib = Join-Path (Split-Path -Parent $MyInvocation.MyCommand.Path) 'lib'
New-Item -ItemType Directory -Force -Path $Lib | Out-Null

$jars = @(
    @{ name = 'joml-1.10.8.jar';            url = 'https://repo1.maven.org/maven2/org/joml/joml/1.10.8/joml-1.10.8.jar' },
    @{ name = 'commons-lang3-3.17.0.jar';   url = 'https://repo1.maven.org/maven2/org/apache/commons/commons-lang3/3.17.0/commons-lang3-3.17.0.jar' }
)

foreach ($j in $jars) {
    $dest = Join-Path $Lib $j.name
    if (Test-Path $dest) { Write-Host "skip   $($j.name)"; continue }
    Write-Host "fetch  $($j.name)"
    Invoke-WebRequest -Uri $j.url -OutFile $dest -TimeoutSec 120
    "        {0} bytes" -f (Get-Item $dest).Length
}