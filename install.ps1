# @impl:FR-CLI-18
# One-liner installer for lab-report on Windows PowerShell
# Usage: irm https://codeberg.org/civitas-project/lab-report-template/raw/branch/main/install.ps1 | iex

$ErrorActionPreference = "Stop"

$Repo = "civitas-project/lab-report-template"
$BaseUrl = "https://codeberg.org/$Repo"

Write-Host "==> Installing lab-report CLI from Codeberg..." -ForegroundColor Cyan

$InstallDir = "$HOME\.cargo\bin"
if (-not (Test-Path $InstallDir)) {
    $InstallDir = "$HOME\AppData\Local\Programs\lab-report"
    New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null
}

$ArchiveName = "lab-report-windows-x86_64.zip"
$DownloadUrl = "$BaseUrl/releases/download/latest/$ArchiveName"

$TempDir = [System.IO.Path]::Combine([System.IO.Path]::GetTempPath(), [System.Guid]::NewGuid().ToString())
New-Item -ItemType Directory -Force -Path $TempDir | Out-Null

try {
    Write-Host "--> Downloading $DownloadUrl..." -ForegroundColor Yellow
    Invoke-WebRequest -Uri $DownloadUrl -OutFile "$TempDir\$ArchiveName" -UseBasicParsing

    Write-Host "--> Verifying SHA-256 checksum..." -ForegroundColor Yellow
    try {
        $ChecksumUrl = "$DownloadUrl.sha256"
        $ChecksumFile = "$TempDir\$ArchiveName.sha256"
        Invoke-WebRequest -Uri $ChecksumUrl -OutFile $ChecksumFile -UseBasicParsing
        $ExpectedHash = (Get-Content $ChecksumFile).Split(" ")[0].Trim()
        $ActualHash = (Get-FileHash "$TempDir\$ArchiveName" -Algorithm SHA256).Hash.ToLower()
        if ($ExpectedHash.ToLower() -ne $ActualHash) {
            Write-Error "SHA-256 checksum mismatch! Expected: $ExpectedHash, Got: $ActualHash"
            exit 1
        }
        Write-Host ">> Checksum verified successfully." -ForegroundColor Green
    } catch {
        Write-Host "Warning: Could not verify checksum, continuing..." -ForegroundColor DarkYellow
    }

    Expand-Archive -Path "$TempDir\$ArchiveName" -DestinationPath $InstallDir -Force
    Write-Host ">> Successfully installed lab-report to $InstallDir\lab-report.exe" -ForegroundColor Green
} catch {
    Write-Host "Note: Release binary download failed, checking for cargo..." -ForegroundColor Yellow
    if (Get-Command cargo -ErrorAction SilentlyContinue) {
        cargo install --git "$BaseUrl.git" --bin lab-report
        Write-Host ">> Successfully installed via cargo!" -ForegroundColor Green
    } else {
        Write-Error "Failed to download binary and cargo is not available. Please visit $BaseUrl"
        exit 1
    }
} finally {
    Remove-Item -Recurse -Force $TempDir -ErrorAction SilentlyContinue
}

Write-Host ""
Write-Host "Run 'lab-report init' to start setup wizard!" -ForegroundColor Cyan
