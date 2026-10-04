# =============================================================================
# Fetches the third-party jars the Java oracle needs into ./lib.
#
#   pwsh -File "Converted Minecraft in rust/_porting/java-oracle/fetch_libs.ps1"
#
# The jars are committed to the repo so the oracle is reproducible offline.
# They are NOT game assets and contain no Mojang code.
#
# =============================================================================
# VERSIONS ARE NOT CHOSEN -- THEY ARE READ FROM METADATA
# =============================================================================
#
# Every coordinate below is the exact version Minecraft 26.2 resolves to. They were
# read out of the Fabric Loom / Gradle cache on this machine, i.e. the real
# resolved dependency tree for the 26.2 client jar:
#
#   %USERPROFILE%\.gradle\caches\modules-2\files-2.1\<group>\<artifact>\<version>\
#
# That is the authoritative source -- it is what Loom actually downloaded and put
# on the compile classpath. Guessing "close enough current" versions is how a port
# ends up subtly wrong, so nothing here is hand-picked.
#
# | coordinate                          | version   | used by the oracle for            |
# |-------------------------------------|-----------|-----------------------------------|
# | org.joml:joml                       | 1.10.8    | Mth#invSqrt, Mth#rotationAroundAxis |
# | org.apache.commons:commons-lang3    | 3.20.0    | Mth#getInt, Mth#mulAndTruncate    |
# | com.google.guava:guava              | 33.6.0-jre| Hashing.md5, Longs, ImmutableList |
# | com.mojang:datafixerupper           | 10.0.21   | Codec, DataResult                  |
# | io.netty:netty-common               | 4.2.15.Fin| ThreadLocalRandom                  |
# | it.unimi.dsi:fastutil               | 8.5.18    | SectionPos LongConsumer            |
# | org.jspecify:jspecify               | 1.0.0     | @Nullable                          |
# | org.jetbrains:annotations           | 26.0.2    | @Contract, @Nullable               |
# | com.google.guava:failureaccess      | 1.0.3     | Guava runtime dep                  |
# | com.google.code.gson:gson           | 2.14.0    | DFU runtime dep                    |
# | com.google.errorprone:...           | 2.48.0    | Guava/DFU compile dep              |
#
# joml specifically matters: `Math.invsqrt` is a version-specific bit trick, so a
# different joml release could change `Mth#invSqrt` bit for bit.
#
# ---------------------------------------------------------------------------
# WHY THERE ARE NO SOURCE STUBS ANY MORE
# ---------------------------------------------------------------------------
# Session 02 needed 17 hand-written stubs. Every one of them has since been
# eliminated:
#
#  * Guava (Hashing/HashCode/HashFunction/Longs/VisibleForTesting),
#    DFU (Codec/DataResult), netty ThreadLocalRandom and jspecify @Nullable
#    were all replaced by the REAL jars above, at the pinned versions.
#  * Vec3i, Vec3, BlockPos, AABB, Identifier, ARGB, Util and ThreadingDetector
#    were stubs only because batch 2 had not compiled the genuine class yet.
#    They are now compiled from minecraft-decompiled/ like every other class.
#
# The stubs/ tree is therefore empty, and that is the invariant to preserve: if a
# future batch needs a third-party type, add the jar here -- never a stub.
# =============================================================================
$ErrorActionPreference = 'Stop'
$Lib = Join-Path (Split-Path -Parent $MyInvocation.MyCommand.Path) 'lib'
New-Item -ItemType Directory -Force -Path $Lib | Out-Null

$base = 'https://repo1.maven.org/maven2'

$jars = @(
    @{ n = 'joml-1.10.8.jar';                g = 'org/joml/joml/1.10.8/joml-1.10.8.jar' },
    @{ n = 'commons-lang3-3.20.0.jar';       g = 'org/apache/commons/commons-lang3/3.20.0/commons-lang3-3.20.0.jar' },
    @{ n = 'guava-33.6.0-jre.jar';           g = 'com/google/guava/guava/33.6.0-jre/guava-33.6.0-jre.jar' },
    @{ n = 'failureaccess-1.0.3.jar';        g = 'com/google/guava/failureaccess/1.0.3/failureaccess-1.0.3.jar' },
    @{ n = 'datafixerupper-10.0.21.jar';     g = 'com/mojang/datafixerupper/10.0.21/datafixerupper-10.0.21.jar' },
    @{ n = 'netty-common-4.2.15.Final.jar';  g = 'io/netty/netty-common/4.2.15.Final/netty-common-4.2.15.Final.jar' },
    @{ n = 'fastutil-8.5.18.jar';            g = 'it/unimi/dsi/fastutil/8.5.18/fastutil-8.5.18.jar' },
    @{ n = 'jspecify-1.0.0.jar';             g = 'org/jspecify/jspecify/1.0.0/jspecify-1.0.0.jar' },
    @{ n = 'annotations-26.0.2.jar';         g = 'org/jetbrains/annotations/26.0.2/annotations-26.0.2.jar' },
    @{ n = 'gson-2.14.0.jar';                g = 'com/google/code/gson/gson/2.14.0/gson-2.14.0.jar' },
    @{ n = 'error_prone_annotations-2.48.0.jar'; g = 'com/google/errorprone/error_prone_annotations/2.48.0/error_prone_annotations-2.48.0.jar' }
)

foreach ($j in $jars) {
    $dest = Join-Path $Lib $j.n
    if (Test-Path $dest) { Write-Host "skip   $($j.n)"; continue }
    Write-Host "fetch  $($j.n)"
    Invoke-WebRequest -Uri "$base/$($j.g)" -OutFile $dest -TimeoutSec 120
    "        {0} bytes" -f (Get-Item $dest).Length
}

# Fail loudly if the tree drifts from the pinned versions, rather than silently
# linking against whatever happens to be on the classpath.
$expected = @(
    'joml-1.10.8.jar', 'commons-lang3-3.20.0.jar', 'guava-33.6.0-jre.jar',
    'failureaccess-1.0.3.jar', 'datafixerupper-10.0.21.jar',
    'netty-common-4.2.15.Final.jar', 'fastutil-8.5.18.jar',
    'jspecify-1.0.0.jar', 'annotations-26.0.2.jar', 'gson-2.14.0.jar',
    'error_prone_annotations-2.48.0.jar'
)
$actual = (Get-ChildItem $Lib -Filter *.jar | ForEach-Object Name) | Sort-Object
$extra = $actual | Where-Object { $expected -notcontains $_ }
if ($extra) {
    Write-Host "STALE jars in lib/ (not pinned by this script): $($extra -join ', ')"
    Write-Host "Remove them -- an unpinned jar makes the oracle non-reproducible."
    exit 1
}
Write-Host "lib/ matches the pinned 26.2 dependency set."
