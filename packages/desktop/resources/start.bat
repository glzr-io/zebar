@echo off
@REM Start hidden powershell script, which runs `zebar open bar --args ...` for every monitor.
powershell -WindowStyle hidden -Command ^
  $monitors = zebar monitors; ^
  foreach ($monitor in $monitors) { Start-Process -WindowStyle Hidden -FilePath \"zebar\" -ArgumentList \"open bar --args $monitor\" };
