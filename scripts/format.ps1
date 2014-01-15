#!/usr/bin/env pwsh

<#
.SYNOPSIS
    C#代码格式化脚本
.DESCRIPTION
    使用dotnet format工具自动格式化C#代码
.EXAMPLE
    .\format.ps1
#>

param(
    [switch]$CheckOnly = $false,
    [switch]$Verbose = $false
)

Write-Host "🚀 开始C#代码格式化检查..." -ForegroundColor Green

# 检查是否安装了dotnet format工具
$formatTool = dotnet tool list --global | Select-String "dotnet-format"
if (-not $formatTool) {
    Write-Host "📦 安装dotnet format工具..." -ForegroundColor Yellow
    dotnet tool install -g dotnet-format
}

# 设置详细级别
$verbosity = if ($Verbose) { "detailed" } else { "normal" }

if ($CheckOnly) {
    Write-Host "🔍 检查代码格式（不修改文件）..." -ForegroundColor Cyan
    $result = dotnet format --verify-no-changes --verbosity $verbosity
    
    if ($LASTEXITCODE -eq 0) {
        Write-Host "✅ 代码格式检查通过！" -ForegroundColor Green
    } else {
        Write-Host "❌ 代码格式检查失败，请运行格式化脚本修复问题" -ForegroundColor Red
        exit 1
    }
} else {
    Write-Host "🔧 自动格式化代码..." -ForegroundColor Cyan
    $result = dotnet format --verbosity $verbosity
    
    if ($LASTEXITCODE -eq 0) {
        Write-Host "✅ 代码格式化完成！" -ForegroundColor Green
    } else {
        Write-Host "⚠️ 格式化过程中出现问题" -ForegroundColor Yellow
    }
}

Write-Host "📊 格式化结果：" -ForegroundColor Magenta
Write-Host $result

# 运行代码分析
Write-Host "🔬 运行代码分析..." -ForegroundColor Cyan
dotnet build --configuration Release --no-restore /p:RunAnalyzers=true

Write-Host "🎉 格式化流程完成！" -ForegroundColor Green
