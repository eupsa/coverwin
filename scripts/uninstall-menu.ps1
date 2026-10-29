# Remove o menu "Converter para" instalado por install-menu.ps1.
$ErrorActionPreference = 'Stop'
foreach ($ext in 'png', 'jpg', 'jpeg', 'webp', 'bmp', 'gif', 'tif', 'tiff', 'ico') {
    $root = "HKCU:\Software\Classes\SystemFileAssociations\.$ext\shell\coverwin"
    if (Test-Path $root) { Remove-Item $root -Recurse }
}
Write-Host 'Menu removido.'
