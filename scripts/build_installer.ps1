<#
PowerShell helper to build the NSIS installer for Snake Game.

Requirements:
- NSIS installed and `makensis` available in PATH.
- Run this from the repository root.
#>

param(
    [string]$Version = "0.1.0"
)

$exeSource = Join-Path -Path "target\release" -ChildPath "snake_game.exe"
$workDir = Join-Path -Path "installers\windows" -ChildPath ""
if (-not (Test-Path $exeSource)) {
    Write-Error "Release executable not found at $exeSource. Run `cargo build --release` first."
    exit 1
}

Write-Host "Copying $exeSource -> $workDir"
Copy-Item -Path $exeSource -Destination $workDir -Force

$nsi = Join-Path $workDir "snake_game.nsi"
if (-not (Test-Path $nsi)) {
    Write-Error "NSIS script not found at $nsi"
    exit 1
}

Write-Host "Running makensis to build installer..."
$makensis = "makensis"
$proc = Start-Process -FilePath $makensis -ArgumentList $nsi -NoNewWindow -Wait -PassThru -ErrorAction SilentlyContinue
if ($proc -eq $null -or $proc.ExitCode -ne 0) {
    Write-Error "makensis failed or is not on PATH. Ensure NSIS is installed and makensis is available."
    exit 1
}

Write-Host "Installer built: snake_game-installer-$Version.exe"
