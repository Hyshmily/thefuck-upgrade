# The Fuck 本地安装脚本
# 这个脚本会自动完成安装过程

Write-Host "The Fuck Rust 版本安装程序" -ForegroundColor Green
Write-Host "=================================" -ForegroundColor Green

# 第一步：创建本地 bin 目录
$binDir = "$env:USERPROFILE\.local\bin"
Write-Host "创建 bin 目录: $binDir" -ForegroundColor Yellow
New-Item -ItemType Directory -Force -Path $binDir | Out-Null

# 第二步：检查构建是否完成
$buildDir = "D:\fork\thefuck-upgrade\thefuck\target\release"
if (Test-Path "$buildDir\thefuck.exe") {
    Write-Host "✓ 找到构建文件" -ForegroundColor Green

    # 第三步：复制文件
    Copy-Item "$buildDir\thefuck.exe" "$binDir\thefuck.exe" -Force
    Copy-Item "$buildDir\thefuck_firstuse.exe" "$binDir\thefuck_firstuse.exe" -Force
    Write-Host "✓ 复制可执行文件完成" -ForegroundColor Green

    # 第四步：添加到 PATH
    $currentPath = [Environment]::GetEnvironmentVariable("Path", "User")
    if (-not $currentPath.Contains($binDir)) {
        [Environment]::SetEnvironmentVariable("Path", $currentPath + ";$binDir", "User")
        Write-Host "✓ 已添加到 PATH" -ForegroundColor Green
    } else {
        Write-Host "✓ PATH 中已包含 bin 目录" -ForegroundColor Green
    }

    # 第五步：设置 PowerShell 配置
    Write-Host "正在配置 PowerShell 别名..." -ForegroundColor Yellow
    $profilePath = $PROFILE
    if (-not (Test-Path $profilePath)) {
        New-Item -ItemType File -Path $profilePath -Force | Out-Null
    }

    $theFuckFunction = @"
# The Fuck 函数
function Invoke-Fuck {
    # 获取上一条命令
    $cmd = Get-History -Count 1 | Select-Object -ExpandProperty CommandLine
    if (-not $cmd) {
        Write-Host "无法获取命令历史" -ForegroundColor Red
        return
    }

    # 设置环境变量
    $env:TF_HISTORY = $cmd
    $env:THEFUCK_COMMAND_HISTORY = $cmd

    # 执行 thefuck
    thefuck
}

# 设置 fuck 别名
Set-Alias -Name fuck -Value Invoke-Fuck

Write-Host "The Fuck 已加载！" -ForegroundColor Green
"@

    # 检查是否已经配置过
    $existingContent = Get-Content $profilePath -ErrorAction SilentlyContinue -Raw
    if ($existingContent -notlike "*Invoke-Fuck*") {
        Add-Content -Path $profilePath -Value $theFuckFunction
        Write-Host "✓ PowerShell 配置已添加" -ForegroundColor Green
    } else {
        Write-Host "✓ PowerShell 已配置过" -ForegroundColor Green
    }

    Write-Host ""
    Write-Host "安装完成！" -ForegroundColor Green
    Write-Host "请重新打开 PowerShell 使配置生效" -ForegroundColor Yellow
    Write-Host "然后测试：gti status, fuck" -ForegroundColor Cyan

} else {
    Write-Host "⚠ 构建尚未完成" -ForegroundColor Yellow
    Write-Host "请等待构建完成后再运行此脚本" -ForegroundColor Yellow
    Write-Host "或者在另一个 PowerShell 窗口中运行：" -ForegroundColor Yellow
    Write-Host "cd D:\fork\thefuck-upgrade\thefuck" -ForegroundColor White
    Write-Host "cargo build --release" -ForegroundColor White
}