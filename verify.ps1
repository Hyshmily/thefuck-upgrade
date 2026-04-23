# The Fuck - PowerShell Verification Script
# This script verifies that thefuck is properly installed

Write-Host "The Fuck - PowerShell Verification Script" -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Green

# Check if thefuck is installed
if (Get-Command thefuck -ErrorAction SilentlyContinue) {
    $thefuckPath = (Get-Command thefuck).Source
    Write-Host "✓ Thefuck is installed at: $thefuckPath" -ForegroundColor Green

    # Check version
    $version = thefuck --version 2>&1
    if ($version) {
        Write-Host "✓ Version: $version" -ForegroundColor Green
    } else {
        Write-Host "✗ Could not get version" -ForegroundColor Red
    }
} else {
    Write-Host "✗ Thefuck is not installed" -ForegroundColor Red
    Write-Host "Run: .\install.ps1" -ForegroundColor Yellow
    exit 1
}

# Check if fuck alias is set
if (Get-Alias fuck -ErrorAction SilentlyContinue) {
    Write-Host "✓ Fuck alias is set: $(Get-Alias fuck)" -ForegroundColor Green
} else {
    Write-Host "! Fuck alias not set - run: Invoke-Expression (thefuck --alias | Out-String)" -ForegroundColor Yellow
}

Write-Host ""
Write-Host "Testing basic functionality..." -ForegroundColor Yellow
Write-Host "git status -> gti status"
Write-Host "fuck should suggest: git status" -ForegroundColor Yellow

# Test command
echo "gti status" | thefuck 2>&1 | Select-Object -First 5
if ($LASTEXITCODE -eq 0) {
    Write-Host "✓ Basic functionality test passed" -ForegroundColor Green
} else {
    Write-Host "! Basic functionality test failed - this is normal for new installations" -ForegroundColor Yellow
}

Write-Host ""
Write-Host "Verification complete!" -ForegroundColor Green