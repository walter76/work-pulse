# AGENTS.md

Guidance for AI agents working in this repo. See `CLAUDE.md` for a fuller overview.

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
└── *.cmd             ← all task automation is Windows .cmd scripts at repo root
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
.\work-pulse-db.cmd          # start postgres:16 container on :5432
.\db-migrate.cmd up          # apply pending migrations (local dbmate)
.\db-migrate.cmd down        # rollback one migration
.\db-migrate.cmd status
.\db-migrate.cmd new <name>  # create new migration file
.\db-migrate.cmd reset       # wipe and re-apply all migrations
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
.\build.cmd                        # build both container images
docker compose up -d               # full stack
.\db-migrate-docker.cmd up         # run migrations against the Docker stack
```

- `src/certificates/` **must exist** (even if empty) — the service Dockerfile has an unconditional `COPY` of it.
- Docker build uses `Cargo.docker.toml` (renamed to `Cargo.toml` inside the image) to exclude the CLI crate.
- `./data/` is the PostgreSQL data volume on the host; delete it with `.\clean-data-folder.cmd` to wipe the DB.

## CI

- Workflow: `.github/workflows/ci.yml`. Triggers: push/PR to `main` only.
- Steps: `cargo build --workspace --verbose` then `cargo test --workspace --verbose` (from `./src`).
- **Frontend is not tested in CI** — no lint, test, or build step for the React app.

## Cargo Workspace Quirk

The workspace uses `resolver = "1"` (legacy feature unification), despite all crates being on `edition = "2024"`. Be aware that features from all workspace crates are unified during resolution.
