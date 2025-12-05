# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Work Pulse is a work hours tracking application with report generation capabilities. The project uses a polyglot architecture with:
- **Backend**: Rust (Axum web framework)
- **Frontend**: React (Vite + MUI Joy)
- **CLI**: Rust (for CSV import/export)
- **Database**: PostgreSQL with DbMate migrations

## Project Structure

The repository contains three main Rust crates and a React app:

- `src/work-pulse-core/`: Domain logic and repository abstractions (Clean Architecture)
  - `entities/`: Domain entities (Activity, AccountingCategory)
  - `adapters/`: Repository trait definitions
  - `use_cases/`: Business logic (activities list, reports, category management)
  - `infra/`: Infrastructure implementations (in-memory and PostgreSQL repositories, CSV importer)

- `src/work-pulse-service/`: REST API service using Axum
  - Serves OpenAPI/Swagger UI at `/swagger-ui`
  - Routes under `/api/v1/` for activities, categories, and reports
  - Supports both PostgreSQL and in-memory repositories via `--use-in-memory-repositories` flag

- `src/work-pulse-cli/`: Command-line tool for CSV operations
  - Imports/exports activities via REST API

- `src/work-pulse-app/`: React frontend (Vite + MUI Joy)
  - Pages for activity log, daily/weekly reports, category configuration
  - Custom hooks: `useActivities`, `useCategories`

## Development Commands

### Backend (Rust)

Build all workspace crates:
```cmd
cd src
cargo build --workspace
```

Run tests:
```cmd
cd src
cargo test --workspace
```

Run service with PostgreSQL (default):
```cmd
cd src/work-pulse-service
cargo run
```

Run service with in-memory repositories (for development):
```cmd
cd src/work-pulse-service
cargo run -- --use-in-memory-repositories
```

### Frontend (React)

Install dependencies:
```cmd
cd src/work-pulse-app
npm install
```

Development server:
```cmd
cd src/work-pulse-app
npm run dev
```

Build for production:
```cmd
cd src/work-pulse-app
npm run build
```

Lint:
```cmd
cd src/work-pulse-app
npm run lint
```

Run tests:
```cmd
cd src/work-pulse-app
npm test
```

Run tests in watch mode:
```cmd
cd src/work-pulse-app
npm run test:watch
```

Test coverage:
```cmd
cd src/work-pulse-app
npm run test:coverage
```

### Database

Start PostgreSQL database:
```cmd
.\work-pulse-db.cmd
```

Run migrations (local DbMate):
```cmd
.\db-migrate.cmd up
```

Run migrations (Docker):
```cmd
.\db-migrate-docker.cmd up
```

Create new migration:
```cmd
.\db-migrate.cmd new migration_name
```

Check migration status:
```cmd
.\db-migrate.cmd status
```

### Docker

Build service container (with custom CA certificates support):
```cmd
docker build -t work-pulse-service --build-arg INCLUDE_CA=true -f src/work-pulse-service.Dockerfile .
```

Run full stack with docker-compose:
```cmd
docker compose up -d
```

Run migrations in docker environment:
```cmd
.\db-migrate-docker.cmd up
```

## Architecture Patterns

### Clean Architecture (Rust Backend)

The `work-pulse-core` crate follows Clean Architecture principles:

1. **Entities** (`entities/`): Pure domain models (Activity, AccountingCategory) with no external dependencies
2. **Adapters** (`adapters/mod.rs`): Repository traits defining contracts for data access
3. **Use Cases** (`use_cases/`): Business logic that orchestrates entities and repositories
4. **Infrastructure** (`infra/`): Concrete implementations of adapters (PostgreSQL, in-memory)

When adding new features:
- Define domain entities in `entities/`
- Add repository traits in `adapters/mod.rs`
- Implement business logic in `use_cases/`
- Create infrastructure implementations in `infra/repositories/`
- Wire up in `work-pulse-service` as Axum routes

### Repository Pattern

Two repository implementations are available:
- **PostgreSQL** (`infra/repositories/postgres/`): Production implementation using sqlx
- **In-Memory** (`infra/repositories/in_memory/`): For testing and development without database

Both implement the same traits from `adapters/mod.rs`, allowing easy swapping via dependency injection in `work-pulse-service/src/main.rs`.

### API Structure

The service uses utoipa for OpenAPI generation:
- Each service module defines routes with OpenAPI annotations
- Tags are defined in `main.rs` prelude
- Router composition happens in `create_open_api_router()`
- All routes are nested under `/api/v1/`

### Frontend Architecture

React app uses:
- **Hooks for data fetching**: `useActivities`, `useCategories` encapsulate API calls
- **MUI Joy components**: Consistent UI using Joy UI design system
- **React Router**: Page navigation
- **API config**: Centralized in `src/config/api.js`

## Database Schema

Main tables (see `db/migrations/`):
- `accounting_categories`: Categories for time tracking (billable/non-billable)
- `activities`: Time log entries with date, duration, category, description

Connection string format:
```
postgres://workpulse:supersecret@localhost:5432/workpulse
```

## Testing

- **Rust**: Unit tests live alongside source files, run with `cargo test --workspace`
- **React**: Jest with Testing Library, run with `npm test` in `work-pulse-app/`
- Integration tests can use `--use-in-memory-repositories` flag to avoid database setup

## Environment Variables

- `RUST_LOG`: Controls Rust logging level (default: "debug")
- `DATABASE_URL`: PostgreSQL connection string (docker-compose only)
- `VITE_API_BASE_URL`: API endpoint for frontend (docker-compose only)

## Certificate Handling

The `INCLUDE_CA=true` build argument in Docker builds copies certificates from `src/certificates/` into the container. This supports corporate networks with custom root CAs. The `certificates/` directory must exist even if empty.
