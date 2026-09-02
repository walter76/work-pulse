# work-pulse
Track working hours and create different reports.

## Database Initial Setup
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
   .\scripts\work-pulse-db.cmd
   ```

__Migration Commands:__

```cmd
# Run pending migrations
.\scripts\db-migrate.cmd up

# Check migration status
.\scripts\db-migrate.cmd status

# Create new migration
.\scripts\db-migrate.cmd new migration_name

# Rollback last migration
.\scripts\db-migrate.cmd down

# Reset database completely
.\scripts\db-migrate.cmd reset
```

### Using Docker:
__Migration Commands:__

```cmd
# Run pending migrations
.\scripts\db-migrate-docker.cmd up

# Check migration status  
.\scripts\db-migrate-docker.cmd status

# Create new migration
.\scripts\db-migrate-docker.cmd new migration_name

# Rollback last migration
.\scripts\db-migrate-docker.cmd down

# Reset database completely
.\scripts\db-migrate-docker.cmd reset
```

### Reset / Delete the whole database

Delete the complete database with:
```cmd
.\scripts\clean-data-folder.cmd
```

Run the initial database migrations as described above (either with a local installation or using Docker).

## Container Building Instructions
The command to build all containers for the backend and frontend is:

```cmd
.\scripts\build.cmd
```

### How to Build the Container for the work-pulse Backend Service

The command to build the container for the work-pulse backend service with Docker is:

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
.\scripts\db-migrate-docker.cmd up
```

### Without Docker and Docker Compose
Without defining a network communication between the containers is prohibited by the system. Therefore, you can only run
the system like that for testing purpose:

Run the database as a container:
```cmd
.\scripts\work-pulse-db.cmd
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

## Production Deployment

To deploy the application stack to a production host without the full source code repository, set up a deployment directory with the required configuration and schema files.

### 1. Required Files and Folder Structure

Copy the following files and directories from the repository into your deployment folder:

```text
deployment-folder/
├── docker-compose.yml
├── db/
│   ├── schema.sql
│   └── migrations/
│       ├── 20241016000001_create_accounting_categories.sql
│       ├── 20241016000002_create_activities.sql
│       └── 20241016000003_insert_default_categories.sql
└── scripts/
    └── db-migrate-docker.cmd
```

### 2. Initial Setup and Database Migrations

Before running the main application stack for the first time, you **must** apply the database migrations to create the database schema and seed default accounting categories.

Run the migration container:

**Windows (using script):**
```cmd
.\scripts\db-migrate-docker.cmd up
```

**Linux / macOS (or directly with Docker Compose):**
```bash
docker compose --profile migration run --rm work-pulse-migrate up
```

> **Note on Initial Database Creation:**
> On a fresh deployment with an empty database directory, PostgreSQL initializes its engine files (`initdb`) and briefly restarts its daemon. If the migration container attempts to connect during this short restart window, it may return a `connection refused` error. If this occurs, simply rerun the migration command above.

### 3. Starting the Application Stack

Once migrations complete successfully, start the full application stack in detached mode:

```bash
docker compose up -d
```

The services will be accessible at:
- **Frontend App**: `http://<host>:3000`
- **Backend API**: `http://<host>:8080`
- **OpenAPI / Swagger UI**: `http://<host>:8080/swagger-ui`
