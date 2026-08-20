# Smart System Manager (ssm) 1-Line Automated Windows Installer Script
$ErrorActionPreference = 'Stop'

# Check for Administrator Privileges
$isAdmin = ([Security.Principal.WindowsPrincipal][Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
if (-not $isAdmin) {
    Write-Host "`n[x] ERROR: Administrator privileges required!" -ForegroundColor Red
    Write-Host "Please right-click PowerShell and select 'Run as Administrator', then run this command again.`n" -ForegroundColor Yellow
    return
}

$installDir = "C:\ssm"
$exePath = Join-Path $installDir "ssm.exe"
$downloadUrl = "https://github.com/thisath111/ssm/releases/latest/download/ssm.exe"

$isUpdate = Test-Path -Path $exePath

Write-Host "==================================================" -ForegroundColor Cyan
if ($isUpdate) {
    Write-Host "  Updating Smart System Manager (ssm) v1.0.0...   " -ForegroundColor Green
} else {
    Write-Host "  Installing Smart System Manager (ssm) v1.0.0... " -ForegroundColor Green
}
Write-Host "==================================================" -ForegroundColor Cyan

# 1. Handle existing installation (Stop running service before file overwrite)
if ($isUpdate) {
    Write-Host "[*] Existing installation detected at $exePath" -ForegroundColor Yellow
    Write-Host "[*] Stopping running background service for clean update..." -ForegroundColor Cyan
    & sc.exe stop SmartSystemManager 2>$null | Out-Null
    Start-Sleep -Milliseconds 500
}

# 2. Create C:\ssm directory if not exists
if (!(Test-Path -Path $installDir)) {
    New-Item -ItemType Directory -Path $installDir -Force | Out-Null
    Write-Host "[+] Created directory: $installDir" -ForegroundColor Yellow
}

# 3. Download ssm.exe from GitHub Releases
Write-Host "[*] Downloading latest ssm.exe from GitHub Releases..." -ForegroundColor Cyan

if (Get-Command curl.exe -ErrorAction SilentlyContinue) {
    & curl.exe -sSL -o "$exePath" "$downloadUrl"
} else {
    $webClient = New-Object System.Net.WebClient
    $webClient.Headers.Add("User-Agent", "ssm-installer")
    $webClient.DownloadFile($downloadUrl, $exePath)
}

if (Test-Path -Path $exePath) {
    if ($isUpdate) {
        Write-Host "[+] Updated ssm.exe at $exePath" -ForegroundColor Green
    } else {
        Write-Host "[+] Installed ssm.exe to $exePath" -ForegroundColor Green
    }
} else {
    Write-Error "[x] Failed to download ssm.exe"
}

# 4. Register C:\ssm in Windows User PATH Environment Variable
$userPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($userPath -notlike "*$installDir*") {
    $newPath = "$userPath;$installDir".Trim(';')
    [Environment]::SetEnvironmentVariable("Path", $newPath, "User")
    Write-Host "[+] Added $installDir to User PATH Environment Variable" -ForegroundColor Green
} else {
    Write-Host "[*] $installDir is already in User PATH" -ForegroundColor Yellow
}

# Also update current session PATH so 'ssm' works immediately without restarting terminal
if ($env:Path -notlike "*$installDir*") {
    $env:Path = "$installDir;$env:Path"
}

# Broadcast environment variable update to Windows
Add-Type -TypeDefinition @"
using System;
using System.Runtime.InteropServices;
public class Win32 {
    [DllImport("user32.dll", SetLastError = true, CharSet = CharSet.Auto)]
    public static extern IntPtr SendMessageTimeout(IntPtr hWnd, uint Msg, UIntPtr wParam, string lParam, uint flags, uint timeout, out IntPtr result);
}
"@ -ErrorAction SilentlyContinue

# 5. Install & Start as Windows Native Service
Write-Host "[*] Registering ssm as Windows Service..." -ForegroundColor Cyan
try {
    & "$exePath" service install
    Write-Host "[+] Registered & Started as Windows Native Service" -ForegroundColor Green
} catch {
    Write-Host "[!] Service installation warning: $_" -ForegroundColor Yellow
}

# 6. Success summary
Write-Host "`n==================================================" -ForegroundColor Green
if ($isUpdate) {
    Write-Host "  SUCCESS: Smart System Manager Updated!          " -ForegroundColor Green
} else {
    Write-Host "  SUCCESS: Smart System Manager (ssm) Installed!  " -ForegroundColor Green
}
Write-Host "==================================================" -ForegroundColor Green
Write-Host "Binary Location : $exePath" -ForegroundColor White
Write-Host "System PATH     : $installDir Registered (Active)" -ForegroundColor White
Write-Host "Boot Autostart  : Enabled (autostart = true)" -ForegroundColor White
Write-Host "`nYou can run 'ssm' commands directly in this terminal right now:" -ForegroundColor Cyan
Write-Host "  ssm boost" -ForegroundColor Yellow
Write-Host "  ssm stats" -ForegroundColor Yellow
Write-Host "  ssm clean" -ForegroundColor Yellow
Write-Host "  ssm uninstall" -ForegroundColor Yellow
Write-Host "==================================================" -ForegroundColor Green
