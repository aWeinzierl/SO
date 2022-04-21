$ErrorActionPreference = "Stop"

### Workardound: https://github.com/PowerShell/PowerShell/issues/13138
Import-Module Appx -usewindowspowershell
###

cargo build
if ( $LASTEXITCODE -eq 0 ) {
    Copy-Item appx\* target\debug
    Set-Location target\debug
    Add-AppxPackage -Register AppxManifest.xml
    Set-Location ..\..\
    Start-Process "shell:AppsFolder\$(Get-StartApps "ABC" | Select-Object -ExpandProperty AppId)"
}