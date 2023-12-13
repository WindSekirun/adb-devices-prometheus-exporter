cat > ~/Library/LaunchAgents/com.github.windsekirun.adb-devices-prometheus-exporter.plist <<EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
	<key>EnvironmentVariables</key>
	<dict>
		<key>PATH</key>
		<string>/opt/homebrew/bin:/opt/homebrew/sbin:/usr/bin:/bin:/usr/sbin:/sbin</string>
	</dict>
	<key>KeepAlive</key>
	<true/>
	<key>Label</key>
	<string>com.github.windsekirun.adb-devices-prometheus-exporter</string>
	<key>LimitLoadToSessionType</key>
	<array>
		<string>Aqua</string>
		<string>Background</string>
		<string>LoginWindow</string>
		<string>StandardIO</string>
		<string>System</string>
	</array>
	<key>ProcessType</key>
	<string>Interactive</string>
	<key>ProgramArguments</key>
	<array>
		<string>/opt/homebrew/opt/adb-devices-prometheus-exporter/bin/adb-devices-prometheus-exporter</string>
        <string>-f</string>
	</array>
	<key>RunAtLoad</key>
	<true/>
</dict>
</plist>
EOF

launchctl unload -w ~/Library/LaunchAgents/com.github.windsekirun.adb-devices-prometheus-exporter.plist
launchctl load -w ~/Library/LaunchAgents/com.github.windsekirun.adb-devices-prometheus-exporter.plist
launchctl start com.github.windsekirun.adb-devices-prometheus-exporter