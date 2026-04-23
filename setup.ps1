# The Fuck PowerShell Setup Script
# 运行此脚本以在 PowerShell 中设置 The Fuck

Write-Host "正在设置 The Fuck..." -ForegroundColor Green

# 检查 thefuck 是否已安装
if (-not (Get-Command thefuck -ErrorAction SilentlyContinue)) {
    Write-Host "TheFuck 未安装，请先运行 install.ps1" -ForegroundColor Red
    exit 1
}

# 获取用户 PowerShell 配置文件路径
$profilePath = $PROFILE

# 检查配置文件是否存在，如果不存在则创建
if (-not (Test-Path $profilePath)) {
    Write-Host "创建 PowerShell 配置文件..." -ForegroundColor Yellow
    New-Item -ItemType File -Path $profilePath -Force | Out-Null
}

# 添加 The Fuck 函数和别名到配置文件
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

Write-Host "The Fuck 已配置！" -ForegroundColor Green
Write-Host "请重新启动 PowerShell 或运行: . $PROFILE" -ForegroundColor Yellow
Write-Host "然后尝试: gti status, fuck" -ForegroundColor Cyan
"@

# 将函数写入配置文件
Add-Content -Path $profilePath -Value $theFuckFunction

Write-Host "配置完成！" -ForegroundColor Green
Write-Host "请重新启动 PowerShell 使配置生效"