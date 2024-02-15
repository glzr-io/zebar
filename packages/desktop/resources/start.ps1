$monitors = zebar monitors
foreach ($monitor in $monitors) {
    Start-Process -NoNewWindow -FilePath "zebar" -ArgumentList "open bar --args $monitor"
}
