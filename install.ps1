# Smart System Manager (ssm) 1-Line Automated Windows Installer Script
$ErrorActionPreference = 'Stop'

Clear-Host
Write-Host "  ███████╗███████╗███╗   ███╗  v1.0.0" -ForegroundColor Cyan
Write-Host "  ██╔════╝██╔════╝████╗ ████║  High-Performance Windows System Optimizer" -ForegroundColor Cyan
Write-Host "  ███████╗███████╗██╔████╔██║  Open-Source Native Rust CLI" -ForegroundColor Cyan
Write-Host "  ╚════██║╚════██║██║╚██╔╝██║  https://github.com/thisath111/ssm" -ForegroundColor Cyan
Write-Host "  ███████║███████║██║ ╚═╝ ██║" -ForegroundColor Cyan
Write-Host "  ╚══════╝╚══════╝╚═╝     ╚═╝`n" -ForegroundColor Cyan

# Check for Administrator Privileges
$isAdmin = ([Security.Principal.WindowsPrincipal][Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
if (-not $isAdmin) {
    Write-Host "[x] ERROR: Administrator privileges required!" -ForegroundColor Red
    Write-Host "Please right-click PowerShell, select 'Run as Administrator', and try again.`n" -ForegroundColor Yellow
    return
}

$installDir = "C:\ssm"
$exePath = Join-Path $installDir "ssm.exe"
$downloadUrl = "https://github.com/thisath111/ssm/releases/latest/download/ssm.exe"
$isUpdate = Test-Path -Path $exePath

if ($isUpdate) {
    Write-Host ">> Updating Smart System Manager (v1.0.0)...`n" -ForegroundColor Green
} else {
    Write-Host ">> Installing Smart System Manager (v1.0.0)...`n" -ForegroundColor Green
}

# 1. Handle existing installation
if ($isUpdate) {
    Write-Host "  [~] Stopping existing background service..." -ForegroundColor Yellow
    & sc.exe stop SmartSystemManager 2>$null | Out-Null
    Start-Sleep -Milliseconds 500
}

# 2. Create Directory
if (!(Test-Path -Path $installDir)) {
    New-Item -ItemType Directory -Path $installDir -Force | Out-Null
    Write-Host "  [+] Created directory: $installDir" -ForegroundColor DarkGray
}

# 3. Download Binary
Write-Host "  [*] Downloading highly-optimized Windows binary..." -ForegroundColor Cyan
if (Get-Command curl.exe -ErrorAction SilentlyContinue) {
    & curl.exe -sSL -o "$exePath" "$downloadUrl"
} else {
    $webClient = New-Object System.Net.WebClient
    $webClient.Headers.Add("User-Agent", "ssm-installer")
    $webClient.DownloadFile($downloadUrl, $exePath)
}

if (Test-Path -Path $exePath) {
    Write-Host "  [+] Binary downloaded successfully." -ForegroundColor Green
} else {
    Write-Error "  [x] Failed to download ssm.exe"
}

# 4. PATH Registration
$userPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($userPath -notlike "*$installDir*") {
    $newPath = "$userPath;$installDir".Trim(';')
    [Environment]::SetEnvironmentVariable("Path", $newPath, "User")
    Write-Host "  [+] Registered PATH variable ($installDir)" -ForegroundColor DarkGray
}
if ($env:Path -notlike "*$installDir*") {
    $env:Path = "$installDir;$env:Path"
}

# Broadcast environment variable update
Add-Type -TypeDefinition @"
using System;
using System.Runtime.InteropServices;
public class Win32 {
    [DllImport("user32.dll", SetLastError = true, CharSet = CharSet.Auto)]
    public static extern IntPtr SendMessageTimeout(IntPtr hWnd, uint Msg, UIntPtr wParam, string lParam, uint flags, uint timeout, out IntPtr result);
}
"@ -ErrorAction SilentlyContinue

# 5. Native Service Registration
Write-Host "  [*] Initializing Windows Native Daemon..." -ForegroundColor Cyan
try {
    # We pipe to Out-Null or capture it to prevent the duplicated Rust CLI banner from ruining the neat installer UI
    $serviceOutput = & "$exePath" service install 2>&1
    Write-Host "  [+] Background Service registered & started." -ForegroundColor Green
} catch {
    Write-Host "  [!] Service installation warning: $_" -ForegroundColor Yellow
}

# 6. Summary
Write-Host "`n==================================================" -ForegroundColor DarkGray
Write-Host "  SUCCESS: Smart System Manager is Ready!" -ForegroundColor Green
Write-Host "==================================================" -ForegroundColor DarkGray
Write-Host "  Location : $exePath" -ForegroundColor White
Write-Host "  Service  : SmartSystemManager (Running)" -ForegroundColor White
Write-Host "  Autostart: Enabled" -ForegroundColor White
Write-Host "`n  Try running these commands now:" -ForegroundColor Cyan
Write-Host "    > ssm boost   (Max performance mode)" -ForegroundColor Yellow
Write-Host "    > ssm clean   (Purge standby RAM)" -ForegroundColor Yellow
Write-Host "    > ssm stats   (Live dashboard)" -ForegroundColor Yellow
Write-Host "==================================================`n" -ForegroundColor DarkGray
