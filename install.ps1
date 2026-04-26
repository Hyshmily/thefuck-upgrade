# The Fuck Installer for Windows

param(
    [string]$Destination = "$env:USERPROFILE\.cargo\bin",
    [switch]$Force = $false
)

Write-Host "Installing The Fuck..." -ForegroundColor Green

# Check if Rust is installed
if (-not (Get-Command rustup -ErrorAction SilentlyContinue)) {
    Write-Host "Rust not found. Installing Rust..."
    & $env:USERPROFILE\.cargo\bin\rustup.exe init --default-toolchain stable -y | Out-Null
    $env:PATH += ";$env:USERPROFILE\.cargo\bin"
}

# Clone the repository
$repoPath = "$env:TEMP\thefuck-upgrade"
if (Test-Path $repoPath) {
    if ($Force) {
        Remove-Item $repoPath -Recurse -Force
    }
    else {
        Write-Host "The Fuck is already installed or the target directory exists. Use -Force to overwrite." -ForegroundColor Yellow
        exit 0
    }
}

Write-Host "Cloning repository..." -ForegroundColor Yellow
git clone https://github.com/HyShmily/thefuck-upgrade.git $repoPath

# Build The Fuck
Write-Host "Building The Fuck..." -ForegroundColor Yellow
Set-Location $repoPath
cd thefuck
cargo build --release

# Copy to destination
$binPath = "$env:USERPROFILE\.cargo\bin"
if (-not (Test-Path $binPath)) {
    New-Item -ItemType Directory -Path $binPath | Out-Null
}

Copy-Item "target\release\thefuck.exe" $binPath -Force
Copy-Item "target\release\thefuck_firstuse.exe" $binPath -Force

Write-Host "The Fuck installed successfully!" -ForegroundColor Green
Write-Host ""
Write-Host "Add the following to your PowerShell profile:"
Write-Host '  Add-Content $PROFILE @"
function Invoke-Fuck {
    $cmd = Get-History -Count 1 | Select-Object -ExpandProperty CommandLine
    $env:TF_HISTORY = $cmd
    $env:THEFUCK_COMMAND_HISTORY = $cmd
    thefuck
}
Set-Alias -Name fuck -Value Invoke-Fuck
@"'

Write-Host ""
Write-Host "Reload PowerShell or run:"
Write-Host "  . $PROFILE"