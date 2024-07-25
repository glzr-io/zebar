# Usage: ./resources/scripts/sign.ps1 -FilePath zebar.exe
param(
  [Parameter(Mandatory=$true)]
  [string]$FilePath
)

if (!(Get-Command "azuresigntool" -ErrorAction SilentlyContinue)) {
  Write-Output "Skipping signing because AzureSignTool is not installed."
  Return
}

$secrets = @(
  "AZ_VAULT_URL",
  "AZ_CERT_NAME",
  "AZ_CLIENT_ID",
  "AZ_CLIENT_SECRET",
  "AZ_TENANT_ID",
  "RFC3161_TIMESTAMP_URL"
)

foreach ($secret in $secrets) {
  if (!(Test-Path "env:$secret")) {
    Write-Output "Skipping signing due to missing secret '$secret'."
    Return
  }
}

Write-Output "Signing $FilePath."
azuresigntool sign -kvu $ENV:AZ_VAULT_URL `
  -kvc $ENV:AZ_CERT_NAME `
  -kvi $ENV:AZ_CLIENT_ID `
  -kvs $ENV:AZ_CLIENT_SECRET `
  -kvt $ENV:AZ_TENANT_ID `
  -tr $ENV:RFC3161_TIMESTAMP_URL `
  -td sha256 $FilePath

if ($LASTEXITCODE -ne 0) {
  Exit 1
}
