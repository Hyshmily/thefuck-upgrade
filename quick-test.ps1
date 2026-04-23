# 快速测试 The Fuck 是否能正常工作

Write-Host "测试 The Fuck..." -ForegroundColor Green

# 测试基本命令修正
echo "git status" | Set-Content test.txt
thefuck --command "git ststus" 2>&1 | Select-Object -First 3

Write-Host ""
Write-Host "如果看到修正建议，说明安装成功！" -ForegroundColor Green
Write-Host "如果没有建议，请检查命令是否正确" -ForegroundColor Yellow