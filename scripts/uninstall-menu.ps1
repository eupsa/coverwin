# Remove o menu "Converter para" instalado por install-menu.ps1.
$ErrorActionPreference = 'Stop'
$classes = 'HKCU:\Software\Classes'
foreach ($ext in 'png', 'jpg', 'jpeg', 'webp', 'bmp', 'gif', 'tif', 'tiff', 'ico') {
    $root = "$classes\SystemFileAssociations\.$ext\shell\coverwin"
    if (Test-Path $root) { Remove-Item $root -Recurse }
}
Get-ChildItem $classes | Where-Object PSChildName -like 'coverwin.menu.*' | Remove-Item -Recurse
Write-Host 'Menu removido.'
