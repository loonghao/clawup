# clawup installer for Windows
# Usage: irm https://raw.githubusercontent.com/loonghao/clawup/main/install.ps1 | iex
#
# Environment variables:
#   CLAWUP_VERSION  - Specific version to install (e.g., "0.1.6"). Default: latest
#   CLAWUP_INSTALL  - Installation directory. Default: $HOME\.clawup\bin
#   CLAWUP_NO_PATH  - Set to "1" to skip adding to PATH. Default: auto-add

$ErrorActionPreference = 'Stop'

$Repo = "loonghao/clawup"
$BinaryName = "clawup"

function Write-Info { param([string]$Message); Write-Host "info: " -ForegroundColor Blue -NoNewline; Write-Host $Message }
function Write-Success { param([string]$Message); Write-Host "success: " -ForegroundColor Green -NoNewline; Write-Host $Message }
function Write-Warn { param([string]$Message); Write-Host "warn: " -ForegroundColor Yellow -NoNewline; Write-Host $Message }
function Write-Err { param([string]$Message); Write-Host "error: " -ForegroundColor Red -NoNewline; Write-Host $Message; exit 1 }

function Get-Architecture {
    $arch = [System.Runtime.InteropServices.RuntimeInformation]::OSArchitecture
    switch ($arch) {
        "X64"   { return "x86_64" }
        "Arm64" { return "aarch64" }
        default { Write-Err "Unsupported architecture: $arch" }
    }
}

function Get-LatestVersion {
    try {
        $release = Invoke-RestMethod -Uri "https://api.github.com/repos/$Repo/releases/latest" -UseBasicParsing
        return $release.tag_name -replace '^v', ''
    }
    catch {
        Write-Err "Failed to fetch latest version: $_"
    }
}

function Install-Clawup {
    $arch = Get-Architecture
    $target = "${arch}-pc-windows-msvc"

    Write-Info "Detected platform: windows $arch ($target)"

    # Determine version
    $version = $env:CLAWUP_VERSION
    if ([string]::IsNullOrEmpty($version) -or $version -eq "latest") {
        Write-Info "Fetching latest version..."
        $version = Get-LatestVersion
    }

    if ([string]::IsNullOrEmpty($version)) {
        Write-Err "Failed to determine version. Set CLAWUP_VERSION or check your network connection."
    }

    Write-Info "Installing clawup v$version..."

    # Determine install directory
    $installDir = $env:CLAWUP_INSTALL
    if ([string]::IsNullOrEmpty($installDir)) {
        $installDir = Join-Path $HOME ".clawup\bin"
    }

    if (-not (Test-Path $installDir)) {
        New-Item -ItemType Directory -Path $installDir -Force | Out-Null
    }

    # Create temp directory
    $tmpDir = Join-Path ([System.IO.Path]::GetTempPath()) "clawup-install-$([System.Guid]::NewGuid().ToString('N').Substring(0,8))"
    New-Item -ItemType Directory -Path $tmpDir -Force | Out-Null

    try {
        $archiveName = "$BinaryName-$target.zip"
        $downloadUrl = "https://github.com/$Repo/releases/download/v$version/$archiveName"
        $archivePath = Join-Path $tmpDir $archiveName

        Write-Info "Downloading $downloadUrl..."
        try {
            Invoke-WebRequest -Uri $downloadUrl -OutFile $archivePath -UseBasicParsing
        }
        catch {
            Write-Err "Download failed. Check if v$version has pre-built binaries for $target. Error: $_"
        }

        # Extract
        Write-Info "Extracting..."
        Expand-Archive -Path $archivePath -DestinationPath $tmpDir -Force

        # Find binary
        $binaryPath = Get-ChildItem -Path $tmpDir -Recurse -Filter "$BinaryName.exe" | Select-Object -First 1
        if ($null -eq $binaryPath) {
            Write-Err "Could not find $BinaryName.exe in the downloaded archive"
        }

        # Install
        $destPath = Join-Path $installDir "$BinaryName.exe"
        Copy-Item -Path $binaryPath.FullName -Destination $destPath -Force

        Write-Success "clawup v$version installed to $destPath"

        # Check if install_dir is in PATH
        $userPath = [System.Environment]::GetEnvironmentVariable("Path", "User")
        if ($userPath -notlike "*$installDir*") {
            if ($env:CLAWUP_NO_PATH -eq "1") {
                Write-Host ""
                Write-Warn "'$installDir' is not in your PATH."
                Write-Host ""
                Write-Host "  To add it permanently, run:" -ForegroundColor Gray
                Write-Host ""
                Write-Host "    [Environment]::SetEnvironmentVariable('Path', `"$installDir;`" + [Environment]::GetEnvironmentVariable('Path', 'User'), 'User')" -ForegroundColor Cyan
                Write-Host ""
            }
            else {
                [System.Environment]::SetEnvironmentVariable("Path", "$installDir;" + $userPath, "User")
                $env:Path = "$installDir;$env:Path"
                Write-Success "Added $installDir to user PATH"
            }
        }

        # Verify installation
        try {
            $installedVersion = & $destPath --version 2>&1 | Select-Object -First 1
            Write-Success "Verified: $installedVersion"
        }
        catch {
            Write-Warn "Could not verify installation, but binary was placed at $destPath"
        }

        Write-Host ""
        Write-Info "Run 'clawup --help' to get started"
    }
    finally {
        # Cleanup
        if (Test-Path $tmpDir) {
            Remove-Item -Path $tmpDir -Recurse -Force -ErrorAction SilentlyContinue
        }
    }
}

Install-Clawup
