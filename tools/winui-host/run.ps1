# Builds the WinUI host (WinUiHost.csproj, Release, for the runner's
# architecture) with the .NET SDK on PATH, runs it twice, and checks that the
# two runs wrote the same frames.toml and the same PNGs (SHA-256). Writes into
# -Out:
#   build.txt        dotnet build's output
#   run<N>/          the host's frames.toml, PNGs, stdout.txt, stderr.txt
#   run.txt          each step's exit status, the tools' versions, the comparison
#   sha256.txt       every output file's SHA-256
param([Parameter(Mandatory = $true)] [string] $Out)

$ErrorActionPreference = 'Continue'
New-Item -ItemType Directory -Force -Path $Out | Out-Null
$Out = (Resolve-Path $Out).Path
$log = Join-Path $Out 'run.txt'
function Note([string] $text) { $text | Tee-Object -FilePath $log -Append }

$arch = if ($env:PROCESSOR_ARCHITECTURE -eq 'ARM64') { 'ARM64' } else { 'x64' }
$rid = 'win-' + $arch.ToLowerInvariant()
Note "processor: $env:PROCESSOR_IDENTIFIER ($env:PROCESSOR_ARCHITECTURE)"
Note "os: $([System.Environment]::OSVersion.VersionString)"
Note "dotnet: $(dotnet --version)"

$project = Join-Path $PSScriptRoot 'WinUiHost.csproj'
dotnet build $project -c Release -p:Platform=$arch -r $rid *>&1 | Tee-Object -FilePath (Join-Path $Out 'build.txt')
$build = $LASTEXITCODE
Note "dotnet build ($arch, $rid): exit $build"
if ($build -ne 0) { exit $build }

$exe = Get-ChildItem -Path (Join-Path $PSScriptRoot 'bin') -Recurse -Filter 'WinUiHost.exe' | Select-Object -First 1
if ($null -eq $exe) { Note 'no WinUiHost.exe'; exit 1 }
Note "host: $($exe.FullName)"

$failed = $false
foreach ($n in 1, 2) {
    $dir = Join-Path $Out "run$n"
    New-Item -ItemType Directory -Force -Path $dir | Out-Null
    $process = Start-Process -FilePath $exe.FullName -ArgumentList @('--out', "`"$dir`"") -PassThru `
        -RedirectStandardOutput (Join-Path $dir 'stdout.txt') -RedirectStandardError (Join-Path $dir 'stderr.txt')
    $null = $process.Handle
    if ($process.WaitForExit(240000)) {
        Note "run${n}: exit $($process.ExitCode)"
        if ($process.ExitCode -ne 0) { $failed = $true }
    } else {
        $process.Kill()
        Note "run${n}: timed out after 240 s"
        $failed = $true
    }
    if (-not (Test-Path (Join-Path $dir 'frames.toml'))) { Note "run${n}: no frames.toml"; $failed = $true }
}

Get-ChildItem -Path $Out -Recurse -File |
    Where-Object { $_.Name -notin @('stdout.txt', 'stderr.txt', 'run.txt', 'sha256.txt', 'build.txt') } |
    ForEach-Object { "$((Get-FileHash -Algorithm SHA256 $_.FullName).Hash.ToLowerInvariant())  $($_.FullName.Substring($Out.Length + 1))" } |
    Set-Content -Path (Join-Path $Out 'sha256.txt')

foreach ($file in Get-ChildItem -Path (Join-Path $Out 'run1') -File | Where-Object { $_.Name -notin @('stdout.txt', 'stderr.txt') }) {
    $other = Join-Path (Join-Path $Out 'run2') $file.Name
    if (-not (Test-Path $other)) { Note "compare $($file.Name): missing in run2"; continue }
    $same = (Get-FileHash -Algorithm SHA256 $file.FullName).Hash -eq (Get-FileHash -Algorithm SHA256 $other).Hash
    Note "compare $($file.Name): $(if ($same) { 'identical' } else { 'differs' })"
    if (-not $same -and $file.Extension -eq '.toml') {
        Compare-Object (Get-Content $file.FullName) (Get-Content $other) | Format-Table -AutoSize | Out-String -Width 400 | Tee-Object -FilePath $log -Append
    }
}

if ($failed) { exit 1 }
