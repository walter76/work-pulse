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
Install dependencies and run frontend checks from `src/work-pulse-app/`:
`npm install`, `npm run lint`, `npm test`, `npm run test:watch`, and `npm run test:coverage`.

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
## Testing
- **React**: Jest with Testing Library, run with `npm test` in `work-pulse-app/`
- **Rust**: see [doc/guidelines/rust-coding-guidelines.md](doc/guidelines/rust-coding-guidelines.md)

## Environment Variables
- `RUST_LOG`: Controls Rust logging level (default: "debug")
- `DATABASE_URL`: PostgreSQL connection string. Optional override for `work-pulse-service` (falls back to a
  hardcoded `localhost` connection string if unset); required by dbmate and set by docker-compose.
- `VITE_API_BASE_URL`: API endpoint for frontend (docker-compose only)

## Certificate Handling
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

The frontend is plain JavaScript and has no `typecheck` script.

### Service Wiring
- The service binds to `0.0.0.0:8080`.
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

## CI
- Workflow: `.github/workflows/ci.yml`. Triggers: push/PR to `main` only.
- Steps: `cargo build --workspace --verbose` then `cargo test --workspace --verbose` (from `./src`).
- **Frontend is not tested in CI** — no lint, test, or build step for the React app.
