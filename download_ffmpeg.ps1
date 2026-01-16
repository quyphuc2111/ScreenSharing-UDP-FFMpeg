# Script tự động download và setup FFmpeg cho Windows

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  FFmpeg Auto-Download Script (Windows)" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

# Configuration
$ffmpegUrl = "https://www.gyan.dev/ffmpeg/builds/ffmpeg-release-essentials.zip"
$downloadPath = "ffmpeg-temp.zip"
$extractPath = "ffmpeg-temp"
$targetPath = "src-tauri\resources\windows"

# Create resources directory if not exists
if (-not (Test-Path $targetPath)) {
    Write-Host "[1/5] Creating resources directory..." -ForegroundColor Yellow
    New-Item -ItemType Directory -Path $targetPath -Force | Out-Null
    Write-Host "      ✓ Created: $targetPath" -ForegroundColor Green
} else {
    Write-Host "[1/5] Resources directory exists" -ForegroundColor Green
}

# Check if FFmpeg already exists
if ((Test-Path "$targetPath\ffmpeg.exe") -and (Test-Path "$targetPath\ffplay.exe")) {
    Write-Host ""
    Write-Host "FFmpeg already exists in resources folder!" -ForegroundColor Green
    Write-Host "Files found:" -ForegroundColor Cyan
    Write-Host "  - ffmpeg.exe: $([math]::Round((Get-Item "$targetPath\ffmpeg.exe").Length / 1MB, 2)) MB" -ForegroundColor White
    Write-Host "  - ffplay.exe: $([math]::Round((Get-Item "$targetPath\ffplay.exe").Length / 1MB, 2)) MB" -ForegroundColor White
    Write-Host ""
    $response = Read-Host "Do you want to re-download? (y/N)"
    if ($response -ne "y" -and $response -ne "Y") {
        Write-Host "Skipping download. Using existing files." -ForegroundColor Yellow
        Write-Host ""
        Write-Host "========================================" -ForegroundColor Cyan
        Write-Host "  Ready to Build!" -ForegroundColor Green
        Write-Host "========================================" -ForegroundColor Cyan
        Write-Host ""
        Write-Host "Next steps:" -ForegroundColor Yellow
        Write-Host "  1. Run: npm run tauri dev" -ForegroundColor White
        Write-Host "  2. Or build: npm run tauri build" -ForegroundColor White
        Write-Host ""
        exit 0
    }
}

# Download FFmpeg
Write-Host ""
Write-Host "[2/5] Downloading FFmpeg..." -ForegroundColor Yellow
Write-Host "      URL: $ffmpegUrl" -ForegroundColor Gray
Write-Host "      Size: ~150 MB" -ForegroundColor Gray
Write-Host "      This may take a few minutes..." -ForegroundColor Gray
Write-Host ""

try {
    $ProgressPreference = 'SilentlyContinue'
    Invoke-WebRequest -Uri $ffmpegUrl -OutFile $downloadPath -UseBasicParsing
    Write-Host "      ✓ Downloaded successfully" -ForegroundColor Green
} catch {
    Write-Host "      ✗ Download failed: $_" -ForegroundColor Red
    Write-Host ""
    Write-Host "Please download manually from:" -ForegroundColor Yellow
    Write-Host "  $ffmpegUrl" -ForegroundColor Cyan
    exit 1
}

# Extract archive
Write-Host ""
Write-Host "[3/5] Extracting archive..." -ForegroundColor Yellow
try {
    Expand-Archive -Path $downloadPath -DestinationPath $extractPath -Force
    Write-Host "      ✓ Extracted successfully" -ForegroundColor Green
} catch {
    Write-Host "      ✗ Extraction failed: $_" -ForegroundColor Red
    Remove-Item $downloadPath -Force -ErrorAction SilentlyContinue
    exit 1
}

# Find and copy binaries
Write-Host ""
Write-Host "[4/5] Copying binaries..." -ForegroundColor Yellow

$ffmpegExe = Get-ChildItem -Path $extractPath -Recurse -Filter "ffmpeg.exe" | Select-Object -First 1
$ffplayExe = Get-ChildItem -Path $extractPath -Recurse -Filter "ffplay.exe" | Select-Object -First 1

if ($ffmpegExe -and $ffplayExe) {
    Copy-Item $ffmpegExe.FullName -Destination "$targetPath\ffmpeg.exe" -Force
    Copy-Item $ffplayExe.FullName -Destination "$targetPath\ffplay.exe" -Force
    Write-Host "      ✓ Copied ffmpeg.exe ($([math]::Round($ffmpegExe.Length / 1MB, 2)) MB)" -ForegroundColor Green
    Write-Host "      ✓ Copied ffplay.exe ($([math]::Round($ffplayExe.Length / 1MB, 2)) MB)" -ForegroundColor Green
} else {
    Write-Host "      ✗ Could not find ffmpeg.exe or ffplay.exe in archive" -ForegroundColor Red
    Remove-Item $downloadPath -Force -ErrorAction SilentlyContinue
    Remove-Item $extractPath -Recurse -Force -ErrorAction SilentlyContinue
    exit 1
}

# Cleanup
Write-Host ""
Write-Host "[5/5] Cleaning up..." -ForegroundColor Yellow
Remove-Item $downloadPath -Force -ErrorAction SilentlyContinue
Remove-Item $extractPath -Recurse -Force -ErrorAction SilentlyContinue
Write-Host "      ✓ Temporary files removed" -ForegroundColor Green

# Summary
Write-Host ""
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  Setup Complete!" -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""
Write-Host "FFmpeg binaries are now in:" -ForegroundColor White
Write-Host "  $targetPath\" -ForegroundColor Cyan
Write-Host ""
Write-Host "Files:" -ForegroundColor White
Write-Host "  ✓ ffmpeg.exe ($([math]::Round((Get-Item "$targetPath\ffmpeg.exe").Length / 1MB, 2)) MB)" -ForegroundColor Green
Write-Host "  ✓ ffplay.exe ($([math]::Round((Get-Item "$targetPath\ffplay.exe").Length / 1MB, 2)) MB)" -ForegroundColor Green
Write-Host ""
Write-Host "Total size: $([math]::Round(((Get-Item "$targetPath\ffmpeg.exe").Length + (Get-Item "$targetPath\ffplay.exe").Length) / 1MB, 2)) MB" -ForegroundColor Cyan
Write-Host ""
Write-Host "Next steps:" -ForegroundColor Yellow
Write-Host "  1. Development: npm run tauri dev" -ForegroundColor White
Write-Host "  2. Production:  npm run tauri build" -ForegroundColor White
Write-Host ""
Write-Host "FFmpeg will be bundled with your Windows application!" -ForegroundColor Green
Write-Host "No need to install FFmpeg on target machines!" -ForegroundColor Green
Write-Host ""
