@echo off
REM Docker-based database migration helper script

if "%1"=="up" (
    echo Running database migrations with Docker...
    docker compose --profile migration run --rm work-pulse-migrate up
    goto end
)

if "%1"=="down" (
    echo Rolling back last migration with Docker...
    docker compose --profile migration run --rm work-pulse-migrate down
    goto end
)

if "%1"=="status" (
    echo Checking migration status with Docker...
    docker compose --profile migration run --rm work-pulse-migrate status
    goto end
)

if "%1"=="new" (
    if "%2"=="" (
        echo Please provide a migration name: db-migrate-docker.cmd new migration_name
        goto end
    )
    echo Creating new migration with Docker: %2
    docker compose --profile migration run --rm work-pulse-migrate new %2
    goto end
)

if "%1"=="reset" (
    echo Resetting database with Docker...
    docker compose --profile migration run --rm work-pulse-migrate drop
    docker compose --profile migration run --rm work-pulse-migrate create  
    docker compose --profile migration run --rm work-pulse-migrate up
    goto end
)

echo Usage:
echo   db-migrate-docker.cmd up      - Run pending migrations
echo   db-migrate-docker.cmd down    - Rollback last migration
echo   db-migrate-docker.cmd status  - Show migration status  
echo   db-migrate-docker.cmd new ^<name^> - Create new migration
echo   db-migrate-docker.cmd reset   - Reset database completely

:end