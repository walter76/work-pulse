@AGENTS.md

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
.\scripts\work-pulse-db.cmd
```

Run migrations (local DbMate):
```cmd
.\scripts\db-migrate.cmd up
```

Run migrations (Docker):
```cmd
.\scripts\db-migrate-docker.cmd up
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

## Repository Layout

```
work-pulse/           ← git root (NOT the Cargo workspace root)
├── db/migrations/    ← DbMate SQL migrations (bidirectional: up/down sections)
├── db/schema.sql     ← auto-generated snapshot; do not edit by hand
├── src/              ← Cargo workspace root (run all `cargo` commands here)
│   ├── Cargo.toml              ← workspace (resolver = "1")
│   ├── Cargo.docker.toml       ← Docker-only workspace (excludes CLI crate)
│   ├── work-pulse-core/        ← domain library (entities, adapters, use cases, infra)
│   ├── work-pulse-service/     ← Axum REST API binary
│   ├── work-pulse-cli/         ← CLI binary (HTTP client only; excluded from Docker)
│   └── work-pulse-app/         ← React frontend (standalone npm package)
├── scripts/          ← all task automation scripts (.cmd, .py)
└── *.cmd             ← moved to scripts/
```

## Critical: Workspace Roots

- **Rust**: workspace root is `src/`, not the repo root. Always run `cargo` from `src/`.
- **Frontend**: `src/work-pulse-app/` is a standalone npm package — no pnpm/yarn workspaces.

## Dev Commands

### Rust (from `src/`)

```cmd
cargo build --workspace
cargo test --workspace
cargo test --workspace <test_name>        # run a single test by name substring
cd src\work-pulse-service && cargo run -- --use-in-memory-repositories  # no DB needed
cd src\work-pulse-service && cargo run    # requires PostgreSQL on :5432
```

### Frontend (from `src/work-pulse-app/`)

```cmd
npm install
npm run dev          # Vite dev server
npm test             # Jest (single pass)
npm run test:watch
npm run test:coverage
npm run lint         # ESLint 9 flat config
npm run build        # production Vite build
```

No `typecheck` script — the project is plain JavaScript, not TypeScript.

### Database (from repo root)

```cmd
.\scripts\work-pulse-db.cmd          # start postgres:16 container on :5432
.\scripts\db-migrate.cmd up          # apply pending migrations (local dbmate)
.\scripts\db-migrate.cmd down        # rollback one migration
.\scripts\db-migrate.cmd status
.\scripts\db-migrate.cmd new <name>  # create new migration file
.\scripts\db-migrate.cmd reset       # wipe and re-apply all migrations
```

## Architecture Notes

### Clean Architecture in `work-pulse-core`

```
entities/      → pure domain models (Activity, AccountingCategory, UUID newtypes)
adapters/      → async_trait repository traits (the contracts)
use_cases/     → business logic; orchestrates repos
infra/         → two implementations per trait: postgres/ and in_memory/
```

Both sets of repos implement the same traits — swap via `--use-in-memory-repositories` at service startup.

### Service Wiring

- Port: always `0.0.0.0:8080`.
- Swagger UI: `/swagger-ui`; OpenAPI JSON: `/api-docs/openapi.json`.
- All REST routes are under `/api/v1/`.
- CORS is fully open (`Any` origin/method/headers).
- The connection string is **hardcoded** in `src/work-pulse-service/src/main.rs` (`const CONNECTION_STRING`). The service does **not** read `DATABASE_URL` from the environment in non-Docker mode. Only dbmate and docker-compose consume `DATABASE_URL`.

### Non-Standard API Behavior

- `PUT /api/v1/activities` — update an activity. The activity ID goes in the **request body**, not the URL path.

### Schema Gotchas

- `activities.end_time` is nullable — an activity without an end time has duration zero in the domain.
- `activities.comment` exists in the DB but is **not mapped** in the Rust `Activity` entity (unused field).
- Deleting a category cascades to delete all its activities (`ON DELETE CASCADE`).
- Migration `20241016000003` seeds 9 default categories. In-memory repos start **empty** — seed data must be created via the API when using `--use-in-memory-repositories`.

## Testing

### Rust

- All tests are unit tests co-located with source in `#[cfg(test)]`. No integration test files.
- CI (`cargo test --workspace`) runs without a database — safe to run anywhere.
- No mock infrastructure is set up; tests use the in-memory repository implementations.

### Frontend

- Test runner is **Jest** (not Vitest), even though Vite is the build tool. There is no `vitest.config.*`.
- Jest uses Babel (`babel-jest`) with manual config in `jest.config.js` — Vite's native ESM pipeline is not used by Jest.
- Test environment: `jsdom`. Setup file: `src/setupTests.js` (adds `@testing-library/jest-dom` matchers).
- Path alias `@/` maps to `src/` (configured in `moduleNameMapper`).
- Only one test file currently exists: `src/lib/dateUtils.test.js` (pure unit, no network, no DOM).
- No running service or database needed for any existing frontend test.

## Code Style (Frontend)

Prettier config (`.prettierrc`): single quotes, no semicolons, trailing commas, 100-char line width, 2-space indent.

## Docker

```cmd
.\scripts\build.cmd                        # build both container images
docker compose up -d               # full stack
.\scripts\db-migrate-docker.cmd up         # run migrations against the Docker stack
```

- `src/certificates/` **must exist** (even if empty) — the service Dockerfile has an unconditional `COPY` of it.
- Docker build uses `Cargo.docker.toml` (renamed to `Cargo.toml` inside the image) to exclude the CLI crate.
- `./data/` is the PostgreSQL data volume on the host; delete it with `.\scripts\clean-data-folder.cmd` to wipe the DB.

## CI

- Workflow: `.github/workflows/ci.yml`. Triggers: push/PR to `main` only.
- Steps: `cargo build --workspace --verbose` then `cargo test --workspace --verbose` (from `./src`).
- **Frontend is not tested in CI** — no lint, test, or build step for the React app.
