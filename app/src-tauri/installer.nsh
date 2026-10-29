; Instala/remove o menu "Converter para" do Explorer reaproveitando os scripts de scripts/.
; Os .ps1 e o coverwin-cli.exe (sidecar) ficam em $INSTDIR.

!macro NSIS_HOOK_POSTINSTALL
  nsExec::Exec 'powershell.exe -NoProfile -ExecutionPolicy Bypass -File "$INSTDIR\install-menu.ps1" -Cli "$INSTDIR\coverwin-cli.exe"'
!macroend

!macro NSIS_HOOK_PREUNINSTALL
  nsExec::Exec 'powershell.exe -NoProfile -ExecutionPolicy Bypass -File "$INSTDIR\uninstall-menu.ps1"'
!macroend
