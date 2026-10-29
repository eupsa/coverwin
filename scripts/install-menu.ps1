# Adiciona "Converter para" ao clique direito de imagens (apenas usuario atual, sem admin).
# Uso: .\scripts\install-menu.ps1 [-Cli caminho\coverwin-cli.exe]
param(
    [string]$Cli = (Join-Path $PSScriptRoot '..\target\release\coverwin-cli.exe')
)
$ErrorActionPreference = 'Stop'

$Cli = (Resolve-Path $Cli).Path
# extensao de origem -> formatos oferecidos (AVIF nao entra como origem: decode nao habilitado)
$sources = 'png', 'jpg', 'jpeg', 'webp', 'bmp', 'gif', 'tif', 'tiff', 'ico'
$targets = [ordered]@{ png = 'PNG'; jpg = 'JPG'; webp = 'WebP'; avif = 'AVIF'; bmp = 'BMP'; gif = 'GIF'; tiff = 'TIFF'; ico = 'ICO' }
$same = @{ jpeg = 'jpg'; tif = 'tiff' }

foreach ($ext in $sources) {
    $root = "HKCU:\Software\Classes\SystemFileAssociations\.$ext\shell\coverwin"
    if (Test-Path $root) { Remove-Item $root -Recurse }
    New-Item $root -Force | Out-Null
    Set-ItemProperty $root -Name MUIVerb -Value 'Converter para'
    Set-ItemProperty $root -Name SubCommands -Value ''
    Set-ItemProperty $root -Name Icon -Value "`"$Cli`""

    $i = 0
    foreach ($t in $targets.Keys) {
        $norm = if ($same[$ext]) { $same[$ext] } else { $ext }
        if ($t -eq $norm) { continue }
        $i++
        # prefixo numerico mantem a ordem do submenu
        $key = "$root\shell\{0:D2}$t" -f $i
        New-Item "$key\command" -Force | Out-Null
        Set-ItemProperty $key -Name MUIVerb -Value $targets[$t]
        # shortcut: conhost --headless esconde a janela, mas erros ficam invisiveis; trocar por notificacao quando o app existir
        Set-ItemProperty "$key\command" -Name '(default)' -Value "conhost.exe --headless `"$Cli`" convert `"%1`" --to $t"
    }
}
Write-Host "Menu instalado para: $($sources -join ', ')"
