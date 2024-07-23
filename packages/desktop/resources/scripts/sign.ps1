# Usage: ./resources/scripts/sign.ps1 -FilePath zebar.exe
param(
  [Parameter(Mandatory=$true)]
  [string]$FilePath
)

$secrets = @(
  $ENV:AZ_VAULT_URL,
  $ENV:AZ_CERT_NAME,
  $ENV:AZ_CLIENT_ID,
  $ENV:AZ_CLIENT_SECRET,
  $ENV:AZ_TENANT_ID,
  $ENV:RFC3161_TIMESTAMP_URL
)

foreach ($secret in $secrets) {
  if (!$secret) {
    Write-Output "Skipping signing due to missing secret."
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
