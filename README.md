# work-pulse
Track my working hours and create reports.

## Initial Setup

This project uses [DbMate](https://github.com/amacneil/dbmate) for database migrations.

Run initial database migrations either with local DbMate installation or using Docker as described below. This will
create the `accounting_categories` and `activities` tables with some default data.

### Using Local DbMate Installation:

__Prerequisites for Local DbMate Installation:__

1. Install DbMate:
   - **Windows (Chocolatey)**: `choco install dbmate`
   - **Go**: `go install github.com/amacneil/dbmate/v2@latest`
   - **Docker**: Use the provided Docker scripts

2. Start PostgreSQL database:
   ```cmd
   .\work-pulse-db.cmd
   ```

__Migration Commands:__

```cmd
# Run pending migrations
.\db-migrate.cmd up

# Check migration status
.\db-migrate.cmd status

# Create new migration
.\db-migrate.cmd new migration_name

# Rollback last migration
.\db-migrate.cmd down

# Reset database completely
.\db-migrate.cmd reset
```

### Using Docker:

__Migration Commands:__

```cmd
# Run pending migrations
.\db-migrate-docker.cmd up

# Check migration status  
.\db-migrate-docker.cmd status

# Create new migration
.\db-migrate-docker.cmd new migration_name

# Rollback last migration
.\db-migrate-docker.cmd down

# Reset database completely
.\db-migrate-docker.cmd reset
```

## Building Instructions

### Building

The command to build all containers for the backend and frontend is:

```cmd
.\build.cmd
```

### How to Build the Container for the Services

The command to build the container for the services with Docker is:

```cmd
docker build -t work-pulse-service --build-arg INCLUDE_CA=true .
```

The build argument `INCLUDE_CA=true` tells the build process that certificates from a different CA (Certificate
Authority) should be included. If this is set, all certificates that are in the subfolder `certificates` are copied into
the build container and registered for the build process. This might be required if you are in a company network which
is changing the Root-CA because of security reasons.

If you omit the build argument `INCLUDE_CA=true` no certificates will be copied and registered. You still require to
have the empty directory `certificates` because building based on the condition whether a directory exists or not is not
so easy with Docker.

## Developer Instructions

### Docker Compose Setup

Run the entire stack with:
```cmd
docker compose up -d
```

To run migrations in Docker environment:
```cmd
.\db-migrate-docker.cmd up
```

### Without Docker and Docker Compose

Without defining a network communication between the containers is prohibited by the system. Therefore, you can only run
the system like that for testing purpose:

Run the database as a container:
```cmd
.\work-pulse-db.cmd
```

Run the backend services:
```cmd
cd src\work-pulse-service
cargo run
```

Run the frontend services:
```cmd
cd src\work-pulse-app
npm run dev
```

### Reset / Delete the whole database

Delete the complete database with:
```cmd
.\clean-data-folder.cmd
```

Run the initial database migrations as described above (either with a local installation or using Docker).
