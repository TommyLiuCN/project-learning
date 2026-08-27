# 用法: .\run.ps1 src/01_hello/HelloWorld.java
# 等价于 java 单文件运行，但会先把控制台切成 UTF-8，避免中文乱码
param(
    [Parameter(Mandatory = $true)]
    [string]$Path
)

if (-not (Test-Path $Path)) {
    Write-Error "文件不存在: $Path"
    exit 1
}

chcp 65001 > $null
[Console]::OutputEncoding = [System.Text.Encoding]::UTF8
java $Path @args
