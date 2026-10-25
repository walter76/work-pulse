@AGENTS.md

# CLAUDE.md
This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview
Work Pulse is a work hours tracking application with report generation capabilities. The project uses a polyglot
architecture with:
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
See [doc/guidelines/rust-coding-guidelines.md](doc/guidelines/rust-coding-guidelines.md) for commands, architecture
rules, and testing.

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

### Backend Architecture (Rust)
Clean Architecture, repository pattern, and REST API rules: see
[doc/guidelines/rust-coding-guidelines.md](doc/guidelines/rust-coding-guidelines.md).

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
- **React**: Jest with Testing Library, run with `npm test` in `work-pulse-app/`
- **Rust**: see [doc/guidelines/rust-coding-guidelines.md](doc/guidelines/rust-coding-guidelines.md)

## Environment Variables
- `RUST_LOG`: Controls Rust logging level (default: "debug")
- `DATABASE_URL`: PostgreSQL connection string. Optional override for `work-pulse-service` (falls back to a
  hardcoded `localhost` connection string if unset); required by dbmate and set by docker-compose.
- `VITE_API_BASE_URL`: API endpoint for frontend (docker-compose only)

## Certificate Handling
The `INCLUDE_CA=true` build argument in Docker builds copies certificates from `src/certificates/` into the container.
This supports corporate networks with custom root CAs. The `certificates/` directory must exist even if empty.

## Repository Layout
```
work-pulse/           ← git root (NOT the Cargo workspace root)
├── db/migrations/    ← DbMate SQL migrations (bidirectional: up/down sections)
├── db/schema.sql     ← auto-generated snapshot; do not edit by hand
├── doc/              ← documentation
├── doc/architecture/ ← architecture documentation
├── scripts/          ← all task automation scripts (.cmd, .py)
├── sessions/         ← AI coding sessions status is persisted here
│   ├── INDEX.md                ← overview for all AI coding sessions
│   ├── template.md             ← empty template to be used for documenting new session
│   └── YYYY-MM-dd-*.md         ← documentation of a single coding session
├── skills/           ← Agent skills to be used in this repository
├── src/              ← Cargo workspace root (run all `cargo` commands here)
│   ├── Cargo.toml              ← workspace (resolver = "2")
│   ├── Cargo.docker.toml       ← Docker-only workspace (excludes CLI crate)
│   ├── work-pulse-core/        ← domain library (entities, adapters, use cases, infra)
│   ├── work-pulse-service/     ← Axum REST API binary
│   ├── work-pulse-cli/         ← CLI binary (HTTP client only; excluded from Docker)
│   └── work-pulse-app/         ← React frontend (standalone npm package)
└── tasks/            ← tasks and backlog for this repository
    ├── BOARD.md                ← overview for all tasks
    ├── template.md             ← empty template for tasks
    ├── backlog/                ← tasks to be done
    ├── done/                   ← tasks that have been finished
    └── in-progress/            ← tasks that are in progress
```

## Critical: Workspace Roots
- **Frontend**: `src/work-pulse-app/` is a standalone npm package — no pnpm/yarn workspaces.

## Dev Commands

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

### Service Wiring
- Port: always `0.0.0.0:8080`.
- Swagger UI: `/swagger-ui`; OpenAPI JSON: `/api-docs/openapi.json`.
- CORS is fully open (`Any` origin/method/headers).
- The service reads `DATABASE_URL` from the environment if set; otherwise falls back to the hardcoded
  `CONNECTION_STRING` constant (`postgres://workpulse:supersecret@localhost:5432/workpulse`) in
  `src/work-pulse-service/src/main.rs`. docker-compose sets `DATABASE_URL` to point at the `work-pulse-db` container
  hostname — required because `localhost` inside the backend container does not resolve to the separate DB container.

### Non-Standard API Behavior
- `PUT /api/v1/activities` — update an activity. The activity ID goes in the **request body**, not the URL path.

### Schema Gotchas
- `activities.end_time` is nullable — an activity without an end time has duration zero in the domain.
- Deleting a category cascades to delete all its activities (`ON DELETE CASCADE`).
- Migration `20241016000003` seeds 9 default categories. In-memory repos start **empty** — seed data must be created via
  the API when using `--use-in-memory-repositories`.

## Testing

### Frontend
- Test runner is **Jest** (not Vitest), even though Vite is the build tool. There is no `vitest.config.*`.
- Jest uses Babel (`babel-jest`) with manual config in `jest.config.js` — Vite's native ESM pipeline is not used by
  Jest.
- Test environment: `jsdom`. Setup file: `src/setupTests.js` (adds `@testing-library/jest-dom` matchers).
- Path alias `@/` maps to `src/` (configured in `moduleNameMapper`).
- Only one test file currently exists: `src/lib/dateUtils.test.js` (pure unit, no network, no DOM).
- No running service or database needed for any existing frontend test.

## Code Style (Frontend)
Prettier config (`.prettierrc`): single quotes, no semicolons, trailing commas, 100-char line width, 2-space indent.

## Docker
```cmd
.\scripts\build.cmd                        # build container images
docker compose up -d                       # run the full stack
.\scripts\db-migrate-docker.cmd up         # run migrations against the Docker stack
```

- `src/certificates/` **must exist** (even if empty) — the service Dockerfile has an unconditional `COPY` of it.
- `./data/` is the PostgreSQL data volume on the host; delete it with `.\scripts\clean-data-folder.cmd` to wipe the DB.

## CI
- Workflow: `.github/workflows/ci.yml`. Triggers: push/PR to `main` only.
- Steps: `cargo build --workspace --verbose` then `cargo test --workspace --verbose` (from `./src`).
- **Frontend is not tested in CI** — no lint, test, or build step for the React app.
