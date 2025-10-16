@echo off
set "dataFolder=data"

if not exist "%dataFolder%" (
    echo The %dataFolder% folder does not exist.
    pause
    exit /b
)

echo This will delete the %dataFolder% folder and all its contents.
set /p confirm=Are you sure you want to continue? (y/N): 

if /i "%confirm%"=="y" (
    echo Deleting %dataFolder% folder...
    rmdir /s /q "%dataFolder%"
    echo %dataFolder% folder has been deleted.
) else (
    echo Operation cancelled.
)

pause