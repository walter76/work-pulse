# Coding Guidelines for Rust

**MUST** = hard rule, enforced in review. **SHOULD** = default; deviate only with a reason.

## Commands
The Cargo workspace root is `src/`, not the repo root. **MUST** run all `cargo` commands from `src/`.

```cmd
cargo build --workspace                             # build all crates
cd work-pulse-service && cargo run -- --use-in-memory-repositories  # run service, no DB needed
cd work-pulse-service && cargo run                  # run service, requires PostgreSQL on :5432
```

For test commands see [Running tests](#running-tests-from-src).

## Tooling
- All crates use **edition 2024**.
- **MUST** keep `cargo clippy --workspace --all-targets -- -D warnings` clean.
- **MUST** keep `cargo build --workspace` and `cargo test --workspace` green (CI runs both).
- Use default rustfmt settings unless a repository rustfmt configuration is added.

## Style
- **SHOULD** prefer small functions, early returns, and iterator chains over manual index loops when they improve clarity.
- **SHOULD** prefer borrowing (`&str`, `&[T]`) in parameters; take ownership when storing or transforming values.
- **SHOULD** derive `Debug`, `Clone`, and `PartialEq` on data types where appropriate; derive serde traits for values
  that cross the frontend/backend boundary or are read from or written to JSON.
- **SHOULD** document public Rust items when their purpose or constraints are not clear from their signatures.
- Comments must not merely restate what the code does.

## Naming
- Modules and files use `snake_case`; types and traits use `UpperCamelCase`.
- Use descriptive names for modules.
- Use newtypes for identifiers when they have domain meaning or need type-level distinction; do not pass unrelated IDs as interchangeable primitive values.
- Modules/files: `snake_case`. Repositories and use cases: `<entity>_list.rs` (e.g. `activities_list.rs`).
- Service modules: `<feature>_service.rs` in `work-pulse-service/src/services/`.
- IDs are newtypes over `Uuid` (`ActivityId(pub Uuid)`, `AccountingCategoryId(pub Uuid)`). **MUST** use the newtype in signatures, never a bare `Uuid`.

## Architecture (`work-pulse-core`)
Dependency direction is strictly inward:

```
entities  ←  adapters  ←  use_cases  ←  infra / work-pulse-service
```

- **MUST**: `entities` depends on no other module and no framework crate (no `sqlx`, `axum`, `utoipa`).
- **MUST**: `use_cases` depends only on `entities` and `adapters` (traits), never on `infra`.
- **MUST**: every repository trait in `adapters/mod.rs` has two implementations in `infra/repositories/`:
  - `postgres/`: production, using `sqlx`
  - `in_memory/`: for tests and for development without a database
- **MUST**: HTTP concerns (status codes, `Json`, `ToSchema` DTOs) live in `work-pulse-service` only.
- Use cases are generic over their repository (`ActivitiesList<R: ...>`) and hold it as `Arc<Mutex<R>>`.
- The implementation is chosen at startup in `work-pulse-service/src/main.rs` (`--use-in-memory-repositories`).

### Adding a feature
1. Domain entity → `entities/`
2. Repository trait → `adapters/mod.rs`
3. Business logic → `use_cases/`
4. Repository implementations (postgres + in-memory) → `infra/repositories/`; other infra (e.g. importers) → `infra/`
5. Axum routes → `work-pulse-service/src/services/`

## Error Handling
- **MUST** use `thiserror` enums for fallible backend operations. Keep errors typed within backend modules, with variants that describe meaningful I/O, JSON parsing, mapping, or bundle-scan failures.
- **MUST**: library code (`work-pulse-core`) uses `thiserror` enums, one per adapter/use case
  (e.g. `ActivitiesListRepositoryError`).
- **SHOULD**: binaries (`work-pulse-cli`) use `anyhow` with `.context(...)` for readable failures.
- **MUST**: the service maps domain errors to HTTP status codes explicitly (`NotFound` → 404, validation → 400,
  everything else → 500). Don't return a 500 for errors the client caused.
- **MUST NOT** use `unwrap()` in non-test code. `expect("<why this cannot fail>")` only for true invariants
  (e.g. startup config). Propagate with `?` otherwise.
- **MUST NOT** use `panic!` for recoverable errors.

## Async
- Runtime: `tokio`.
- Repository traits use `#[async_trait]`. **SHOULD** keep this for trait objects; native `async fn` in traits is fine
  for traits used only generically.
- **MUST NOT** hold a `Mutex` guard across an `.await` unless it is a `tokio::sync::Mutex`.
- **MUST NOT** block the runtime (no `std::thread::sleep`, no blocking I/O in async fns; use
  `tokio::task::spawn_blocking` if unavoidable).

## Logging
- Use `tracing` macros (`debug!`, `info!`, `warn!`, `error!`), not `println!`, in the service and core.
- Levels: `error` = request failed due to a server fault, `warn` = recoverable anomaly, `info` = lifecycle events,
  `debug` = request-level detail.
- **MUST NOT** log secrets (`DATABASE_URL`, credentials) or full request bodies.

## REST API (`work-pulse-service`)
- **MUST** annotate every handler with `#[utoipa::path(...)]` using a tag constant from the `main.rs` prelude.
- **MUST** register routes under `/api/v1/` via `create_open_api_router()`.
- **MUST NOT** make breaking changes to `/api/v1/` contracts; introduce `/api/v2/` instead.
- Request/response DTOs derive `Serialize`/`Deserialize` + `ToSchema` and live next to their handler.

## Testing
- CI runs `cargo test --workspace` without a database. Unit tests **MUST NOT** require PostgreSQL.

### Unit tests
- Co-located with source in a `#[cfg(test)] mod tests` block.
- Use the in-memory repositories; no mocking framework.
- Async tests use `#[tokio::test]`.
- Naming: `<subject>_should_<expected_behavior>[_when_<condition>]`, e.g.
  `accounting_category_id_parse_str_should_fail_with_invalid_id`.
- **SHOULD** cover the error path, not just the happy path.

### Integration tests (BDD)
- Location: `work-pulse-service/tests/` — `services.rs` (Cucumber runner, `harness = false`) and
  `features/*.feature` (Gherkin scenarios).
- **SHOULD** add a `.feature` scenario for each new REST endpoint.
- Tests build routers directly (no running service). **SHOULD** pass in-memory repositories to avoid database setup.
- Known issue: the "connected database" health-check scenario needs PostgreSQL on `localhost:5432`, and the runner
  uses `World::run`, which does not fail the process on failed scenarios. In CI that scenario fails silently.

### Running tests (from `src/`)
```cmd
cargo test --workspace                              # all tests
cargo test --workspace <test_name>                  # filter by name substring
cargo test -p work-pulse-service --test services    # BDD integration tests only
```

## Dependencies
- **SHOULD** check whether a crate is already in use before adding a new one; reuse the same version.
- **SHOULD** enable only the features you need.
- The Docker build uses `Cargo.docker.toml` (renamed to `Cargo.toml` inside the image), which excludes
  `work-pulse-cli`. Dependencies needed by the service
  **MUST** be declared in the service or core `Cargo.toml`, never only in the CLI.
