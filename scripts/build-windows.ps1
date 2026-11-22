$ErrorActionPreference = 'Stop'
$projectDir = Split-Path -Parent $PSScriptRoot
Set-Location -LiteralPath $projectDir

# The caller must initialize the MSVC environment and add Cargo to PATH first.
if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) { throw 'Cargo no esta en PATH. Consulta DEVELOPMENT.md.' }
if (-not (Get-Command link.exe -ErrorAction SilentlyContinue)) { throw 'MSVC no esta inicializado. Consulta DEVELOPMENT.md.' }
& npm run tauri -- build --no-bundle
if ($LASTEXITCODE -ne 0) { throw "La compilacion fallo (codigo $LASTEXITCODE)." }

$builtExe = Join-Path $projectDir 'src-tauri\target\release\wachin-wifi.exe'
if (-not (Test-Path -LiteralPath $builtExe -PathType Leaf)) { throw 'No se encontro el ejecutable compilado.' }
$deliveryDir = Join-Path $projectDir 'WACHIN-WIFI - Entrega'
New-Item -ItemType Directory -Path $deliveryDir -Force | Out-Null
Copy-Item -LiteralPath $builtExe -Destination (Join-Path $projectDir 'WACHIN-WIFI.exe') -Force
Copy-Item -LiteralPath $builtExe -Destination (Join-Path $deliveryDir 'WACHIN-WIFI.exe') -Force
Copy-Item -LiteralPath (Join-Path $projectDir 'scripts\LEEME.txt') -Destination (Join-Path $deliveryDir 'LEEME.txt') -Force
Write-Host "Ejecutable: $(Join-Path $projectDir 'WACHIN-WIFI.exe')"
Write-Host "Carpeta de entrega: $deliveryDir"
