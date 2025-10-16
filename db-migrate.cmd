@echo off
REM Database migration helper script

set "DB_URL=postgres://workpulse:supersecret@localhost:5432/workpulse?sslmode=disable"

if "%1"=="up" (
    echo Running database migrations...
    dbmate --url "%DB_URL%" up
    goto end
)

if "%1"=="down" (
    echo Rolling back last migration...
    dbmate --url "%DB_URL%" down
    goto end
)

if "%1"=="status" (
    echo Checking migration status...
    dbmate --url "%DB_URL%" status
    goto end
)

if "%1"=="new" (
    if "%2"=="" (
        echo Please provide a migration name: db-migrate.cmd new migration_name
        goto end
    )
    echo Creating new migration: %2
    dbmate --url "%DB_URL%" new %2
    goto end
)

if "%1"=="reset" (
    echo Resetting database (DROP and CREATE)...
    dbmate --url "%DB_URL%" drop
    dbmate --url "%DB_URL%" create
    dbmate --url "%DB_URL%" up
    goto end
)

echo Usage:
echo   db-migrate.cmd up      - Run pending migrations
echo   db-migrate.cmd down    - Rollback last migration  
echo   db-migrate.cmd status  - Show migration status
echo   db-migrate.cmd new ^<name^> - Create new migration
echo   db-migrate.cmd reset   - Reset database completely

:end