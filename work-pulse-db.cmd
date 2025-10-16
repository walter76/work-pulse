@echo off
set "DB_DATA_FOLDER_PATH=.\data"

if not exist "%DB_DATA_FOLDER_PATH%" (
    mkdir "%DB_DATA_FOLDER_PATH%"
    echo Folder created: %DB_DATA_FOLDER_PATH%
) else (
    echo Folder already exists: %DB_DATA_FOLDER_PATH%
)

echo Starting PostgreSQL database...
docker run -d ^
    --name work-pulse-db ^
    -e POSTGRES_PASSWORD=supersecret ^
    -e POSTGRES_USER=workpulse ^
    -e POSTGRES_DB=workpulse ^
    -v "%CD%\data":/var/lib/postgresql/data ^
    -p 5432:5432 ^
    --health-cmd="pg_isready -U workpulse -d workpulse" ^
    --health-interval=10s ^
    --health-timeout=2s ^
    --health-retries=5 ^
    postgres:16
