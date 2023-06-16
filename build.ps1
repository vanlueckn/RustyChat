# Builds a .ts3_plugin file for teamspeak 3. Only for development, the ci will build the plugin itself
# 7z is required
$ErrorActionPreference = "stop"

$ts3_package_dir = "./ts3_package"
$ts3_plugin_dll_dir = "$($ts3_package_dir)/plugins"
$plugin_name = "rustychat"
$ts3_plugin_output = ".\$($plugin_name).ts3_plugin"

cargo b
Remove-Item "$($ts3_plugin_dll_dir)/*.dll"
Copy-Item "./target/debug/$($plugin_name).dll" "$($ts3_plugin_dll_dir)/$($plugin_name)_win32.dll"
Copy-Item "./target/debug/$($plugin_name).dll" "$($ts3_plugin_dll_dir)/$($plugin_name)_win64.dll"

Add-Type -AssemblyName System.IO.Compression.FileSystem
[System.IO.Compression.ZipFile]::CreateFromDirectory($ts3_package_dir, $ts3_plugin_output)

Start-Process $ts3_plugin_output
