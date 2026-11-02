$ErrorActionPreference = "Stop"

$Repo = "jhosein58/abyss"
$Version = if ($env:ABYSS_VERSION) { $env:ABYSS_VERSION } else { "latest" }
$InstallDir = if ($env:ABYSS_INSTALL_DIR) { $env:ABYSS_INSTALL_DIR } else { Join-Path $HOME ".abyss" }
$BinDir = Join-Path $InstallDir "bin"

if ($Version -eq "latest") {
    $ZipUrl = "https://github.com/$Repo/releases/latest/download/abyss-windows-x86_64.zip"
} else {
    $ZipUrl = "https://github.com/$Repo/releases/download/$Version/abyss-windows-x86_64.zip"
}

$TempZip = Join-Path ([System.IO.Path]::GetTempPath()) "abyss.zip"
$TempExtract = Join-Path ([System.IO.Path]::GetTempPath()) "abyss_extract"

Write-Host "Downloading Abyss toolchain from $ZipUrl..."
[Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12
Invoke-WebRequest -Uri $ZipUrl -OutFile $TempZip

if (Test-Path $TempExtract) {
    Remove-Item -Recurse -Force $TempExtract
}
Expand-Archive -Path $TempZip -DestinationPath $TempExtract -Force

if (-not (Test-Path $InstallDir)) {
    New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
} else {
    $OldBin = Join-Path $InstallDir "bin"
    if (Test-Path $OldBin) { Remove-Item -Recurse -Force $OldBin }
    $OldPods = Join-Path $InstallDir "pods"
    if (Test-Path $OldPods) { Remove-Item -Recurse -Force $OldPods }
}

$SourceRoot = (Get-ChildItem -Path $TempExtract | Where-Object { $_.PSIsContainer } | Select-Object -First 1).FullName
if (-not $SourceRoot) {
    $SourceRoot = $TempExtract
}

Copy-Item -Path "$SourceRoot\*" -Destination $InstallDir -Recurse -Force

Remove-Item -Force $TempZip -ErrorAction SilentlyContinue
Remove-Item -Recurse -Force $TempExtract -ErrorAction SilentlyContinue

$UserPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($UserPath -notlike "*$BinDir*") {
    $NewPath = if ($UserPath) { "$UserPath;$BinDir" } else { $BinDir }
    [Environment]::SetEnvironmentVariable("Path", $NewPath, "User")
    Write-Host "Added $BinDir to User PATH."
}

Write-Host ""
Write-Host "✔ Abyss & Sonar installed successfully to $InstallDir!"
Write-Host "Binaries available in: $BinDir"
Write-Host "Please restart your PowerShell session to apply PATH changes."
