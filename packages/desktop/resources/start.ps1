$monitors = zebar monitors
foreach ($monitor in $monitors) {
    Start-Process -NoNewWindow -FilePath "zebar" -ArgumentList "open bar --args $monitor"
}
# Start-Process -FilePath "cmd.exe" -ArgumentList "/c start.bat" -WindowStyle Hidden
# Start-Process -FilePath "cmd.exe" -ArgumentList "/c ./start.bat"
