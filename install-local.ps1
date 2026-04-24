# The Fuck local installation script
# This script automates the local installation process.

Write-Host "The Fuck Rust installer" -ForegroundColor Green
Write-Host "=================================" -ForegroundColor Green

# Step 1: create the local bin directory
$binDir = "$env:USERPROFILE\.local\bin"
Write-Host "Creating bin directory: $binDir" -ForegroundColor Yellow
New-Item -ItemType Directory -Force -Path $binDir | Out-Null

# Step 2: check whether the build artifacts exist
$buildDir = "D:\fork\thefuck-upgrade\thefuck\target\release"
if (Test-Path "$buildDir\thefuck.exe") {
    Write-Host "✓ Build artifacts found" -ForegroundColor Green

    # Step 3: copy binaries
    Copy-Item "$buildDir\thefuck.exe" "$binDir\thefuck.exe" -Force
    Copy-Item "$buildDir\thefuck_firstuse.exe" "$binDir\thefuck_firstuse.exe" -Force
    Write-Host "✓ Copied executables" -ForegroundColor Green

    # Step 4: add to PATH
    $currentPath = [Environment]::GetEnvironmentVariable("Path", "User")
    if (-not $currentPath.Contains($binDir)) {
        [Environment]::SetEnvironmentVariable("Path", $currentPath + ";$binDir", "User")
        Write-Host "✓ Added to PATH" -ForegroundColor Green
    }
    else {
        Write-Host "✓ PATH already contains the bin directory" -ForegroundColor Green
    }

    # Step 5: configure PowerShell alias support
    Write-Host "Configuring PowerShell alias..." -ForegroundColor Yellow
    $profilePath = $PROFILE
    if (-not (Test-Path $profilePath)) {
        New-Item -ItemType File -Path $profilePath -Force | Out-Null
    }

    $theFuckFunction = @"
# The Fuck function
function Invoke-Fuck {
    # Read the last command from history
    $cmd = Get-History -Count 1 | Select-Object -ExpandProperty CommandLine
    if (-not $cmd) {
        Write-Host "Unable to read command history" -ForegroundColor Red
        return
    }

    # Set environment variables
    $env:TF_HISTORY = $cmd
    $env:THEFUCK_COMMAND_HISTORY = $cmd

    # Run thefuck
    thefuck
}

# Set the fuck alias
Set-Alias -Name fuck -Value Invoke-Fuck

Write-Host "The Fuck loaded successfully!" -ForegroundColor Green
"@

    # Check whether the profile already contains the function
    $existingContent = Get-Content $profilePath -ErrorAction SilentlyContinue -Raw
    if ($existingContent -notlike "*Invoke-Fuck*") {
        Add-Content -Path $profilePath -Value $theFuckFunction
        Write-Host "✓ PowerShell profile updated" -ForegroundColor Green
    }
    else {
        Write-Host "✓ PowerShell profile already configured" -ForegroundColor Green
    }

    Write-Host ""
    Write-Host "Installation complete!" -ForegroundColor Green
    Write-Host "Restart PowerShell for the changes to take effect." -ForegroundColor Yellow
    Write-Host "Then test: gti status, fuck" -ForegroundColor Cyan

}
else {
    Write-Host "⚠ Build is not ready yet" -ForegroundColor Yellow
    Write-Host "Wait for the build to finish before running this script." -ForegroundColor Yellow
    Write-Host "Or run the following in another PowerShell window:" -ForegroundColor Yellow
    Write-Host "cd D:\fork\thefuck-upgrade\thefuck" -ForegroundColor White
    Write-Host "cargo build --release" -ForegroundColor White
}