@echo off
@REM powershell.exe -Command "Start-Process -NoNewWindow -FilePath ./start.ps1"
@REM powershell.exe -Command "Start-Process -FilePath 'powershell' -ArgumentList '-File ./script.ps1' -WindowStyle Hidden"
@REM Start-Process -NoNewWindow -FilePath "zebar" -ArgumentList "open bar --args $monitor"

@REM for /f "tokens=*" %%a in ('zebar monitors') do (
@REM     start /b zebar open bar --args %%a
@REM )
powershell -WindowStyle hidden -Command ^
  $monitors = zebar monitors; ^
  foreach ($monitor in $monitors) { Start-Process -NoNewWindow -FilePath \"zebar\" -ArgumentList \"open bar --args $monitor\" };

  @REM Start-Process -NoNewWindow -FilePath "zebar" -ArgumentList "open bar --args $monitor"Write-Output 'first line'; ^
  @REM Write-Output 'second line';
