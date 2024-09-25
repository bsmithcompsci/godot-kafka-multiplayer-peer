@echo off

set DOWNLOAD_URL=https://github.com/godotengine/godot/releases/download/4.3-stable/Godot_v4.3-stable_win64.exe.zip
set UNZIP_DIR=.godot-editor

REM Create the target directory if it doesn't exist
if not exist %UNZIP_DIR% mkdir %UNZIP_DIR%

REM Check if the executable already exists
if not exist %UNZIP_DIR%\Godot_v4.3-stable_win64.exe (
    pushd %UNZIP_DIR%
    REM Download the Godot editor zip file
    curl -L -o godot-editor.zip %DOWNLOAD_URL%

    REM Unzip the downloaded file into the target directory
    powershell -Command "Expand-Archive -Path godot-editor.zip -DestinationPath ."

    REM Clean up the downloaded zip file
    del godot-editor.zip
    popd
)

REM Prompt the user to choose between console or regular executable
choice /C YN /M "Do you want to open the Godot Editor in Console Mode?"

REM Open the chosen executable
if errorlevel 2 (
    call %UNZIP_DIR%\Godot_v4.3-stable_win64.exe --editor --path %CD%\demo\client
) else (
    %UNZIP_DIR%\Godot_v4.3-stable_win64_console.exe --editor --path %CD%\demo\client
)
