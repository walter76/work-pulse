# Work Pulse — Comprehensive Architecture Review

**Repository:** `D:\WS\src\work-pulse`
**Review Date:** 2026-06-22

---

## Table of Contents

1. [Executive Summary](#1-executive-summary)
2. [Tech Stack](#2-tech-stack)
3. [Directory Structure](#3-directory-structure)
4. [Core Systems](#4-core-systems)
   - 4.1 [work-pulse-core — Domain Library](#41-work-pulse-core--domain-library)
   - 4.2 [work-pulse-service — REST API](#42-work-pulse-service--rest-api)
   - 4.3 [work-pulse-cli — CLI Tool](#43-work-pulse-cli--cli-tool)
   - 4.4 [work-pulse-app — React Frontend](#44-work-pulse-app--react-frontend)
5. [Data Flow](#5-data-flow)
6. [Key Data Structures](#6-key-data-structures)
7. [Architectural Patterns](#7-architectural-patterns)
8. [File Index](#8-file-index)
9. [Configuration](#9-configuration)
10. [Build & Deploy](#10-build--deploy)
11. [External Integrations](#11-external-integrations)
12. [Observations & Recommendations](#12-observations--recommendations)

---

## 1. Executive Summary

Work Pulse is a personal work-hours tracking application that allows a user to record daily activities against accounting categories and generate daily and weekly time reports. The system is built as a three-tier web application plus a supplementary CLI tool.

The architecture is deliberately modest in scope but technically careful: the Rust backend enforces Clean Architecture layering with a strict dependency direction from infrastructure to domain. The frontend is a lightweight React SPA using MUI Joy components. PostgreSQL stores data in a two-table schema. All components can run locally in minutes or be deployed together with Docker Compose.

**Key characteristics:**
- **Clean Architecture** in the Rust core library — entities → adapters (traits) → use cases → infrastructure
- **Dual repository backend** — every repository trait has both a PostgreSQL and an in-memory implementation; the service selects at startup via a CLI flag
- **OpenAPI-first service** — utoipa generates and serves Swagger UI from inline annotations
- **CORS fully open** — the service allows any origin, method, and header
- **Hardcoded connection string** — the service binary does not read `DATABASE_URL` from the environment; Docker Compose injects it but the service code never reads it (see Observations)
- **No authentication** — the API has no auth layer
- **Minimal test surface** — Rust unit tests are thorough for the domain; the frontend has one test file covering `dateUtils`

---

## 2. Tech Stack

### Backend

| Concern | Crate / Lib | Version |
|---|---|---|
| Language | Rust (edition 2024) | rustc (latest stable) |
| Async runtime | tokio | 1.48 |
| Web framework | axum | 0.8.4 |
| HTTP server | hyper | 1.6 |
| Middleware | tower / tower-http | 0.5 / 0.6.5 |
| Database driver | sqlx | 0.8.6 (postgres + uuid + chrono) |
| Async trait | async-trait | 0.1.89 |
| Date/time | chrono | 0.4.41 |
| UUIDs | uuid | 1.17 (v4) |
| Serialization | serde | 1.0.219 |
| CSV parsing | csv | 1.3.1 |
| OpenAPI generation | utoipa + utoipa-axum | 5.3 / 0.2 |
| Swagger UI | utoipa-swagger-ui | 9.0.2 |
| CLI argument parsing | clap (derive) | 4.5.50 |
| Tracing | tracing + tracing-subscriber | 0.1 / 0.3 |
| Error types | thiserror | 2.0.12 |

### Frontend

| Concern | Package | Version |
|---|---|---|
| Language | JavaScript (ESM) | — |
| UI framework | React | 19.1 |
| Component library | MUI Joy | 5.0.0-beta.52 |
| Icons | @mui/icons-material | 7.1.1 |
| Styling | @emotion/react + @emotion/styled | 11.14 |
| Font | @fontsource/inter | 5.2.5 |
| Routing | react-router-dom | 7.6.3 |
| HTTP client | axios | 1.9 |
| Build tool | Vite 6 + @vitejs/plugin-react | 6.3.5 / 4.4.1 |
| Test runner | Jest (jsdom) | 30.2 |
| Test transform | babel-jest (preset-env + preset-react) | 30.2 |
| Test utilities | @testing-library/react + jest-dom | 16.3 / 6.9.1 |
| Linting | ESLint 9 (flat config) | 9.25 |
| Code style | Prettier | (config in `.prettierrc`) |

### Database & Tooling

| Concern | Tool | Version |
|---|---|---|
| Database | PostgreSQL | 16 (Docker image) |
| Migrations | DbMate | latest |
| Container runtime | Docker / Docker Compose | v3.9 compose spec |

### CLI

| Concern | Crate | Version |
|---|---|---|
| HTTP client | reqwest (blocking + json) | 0.12.20 |
| CSV | csv | 1.3.1 |
| Encoding | encoding_rs | 0.8.35 |
| Lazy statics | once_cell | 1.20.2 |
| Error handling | anyhow | 1.0.98 |

---

## 3. Directory Structure

```
work-pulse/                       ← git root
│
├── .env                          ← DATABASE_URL for dbmate (local dev)
├── .gitignore
├── .github/
│   └── workflows/
│       └── ci.yml                ← GitHub Actions CI (Rust only)
│
├── build.cmd                     ← Build both Docker images
├── clean-data-folder.cmd         ← Delete ./data/ (wipe DB volume)
├── db-migrate.cmd                ← DbMate wrapper (local install)
├── db-migrate-docker.cmd         ← DbMate wrapper (Docker-based)
├── docker-compose.yml            ← Full stack: db + backend + frontend + migrate
├── work-pulse-db.cmd             ← Start postgres:16 container standalone
├── work-pulse-service.cmd        ← Run pre-built service container
│
├── db/
│   ├── migrations/               ← DbMate SQL migrations (up/down)
│   │   ├── 20241016000001_create_accounting_categories.sql
│   │   ├── 20241016000002_create_activities.sql
│   │   └── 20241016000003_insert_default_categories.sql
│   └── schema.sql                ← Auto-generated snapshot (do not edit)
│
├── AGENTS.md                     ← AI agent guidance
├── CLAUDE.md                     ← Claude-specific guidance
├── README.md
├── TODO.md
└── LICENSE
│
└── src/                          ← Cargo workspace root
    ├── Cargo.toml                ← workspace (resolver="2", all 3 crates)
    ├── Cargo.docker.toml         ← workspace without CLI (for Docker build)
    ├── Cargo.lock
    │
    ├── work-pulse-core/          ← Domain library
    │   ├── Cargo.toml
    │   └── src/
    │       ├── lib.rs
    │       ├── entities/
    │       │   ├── mod.rs
    │       │   ├── activity.rs         ← Activity + ActivityId domain types
    │       │   └── accounting.rs       ← AccountingCategory + AccountingCategoryId
    │       ├── adapters/
    │       │   └── mod.rs              ← Repository + Importer traits
    │       ├── use_cases/
    │       │   ├── mod.rs
    │       │   ├── activities_list.rs  ← CRUD + import orchestration
    │       │   ├── accounting_categories_list.rs
    │       │   ├── daily_report.rs
    │       │   └── weekly_report.rs
    │       └── infra/
    │           ├── mod.rs
    │           ├── importers/
    │           │   ├── mod.rs
    │           │   └── csv_activities_importer.rs
    │           └── repositories/
    │               ├── mod.rs
    │               ├── postgres/
    │               │   ├── mod.rs              ← PsqlConnection (sqlx pool)
    │               │   ├── activities_list.rs
    │               │   └── accounting_categories_list.rs
    │               └── in_memory/
    │                   ├── mod.rs
    │                   ├── activities_list.rs
    │                   └── accounting_categories_list.rs
    │
    ├── work-pulse-service/       ← Axum REST API binary
    │   ├── Cargo.toml
    │   └── src/
    │       ├── main.rs           ← Startup, routing, dependency injection
    │       └── services/
    │           ├── mod.rs
    │           ├── activities_list_service.rs   ← /api/v1/activities
    │           ├── accounting_categories_service.rs ← /api/v1/accounting-categories
    │           ├── daily_report_service.rs       ← /api/v1/daily-report
    │           └── weekly_report_service.rs      ← /api/v1/weekly-report
    │
    ├── work-pulse-cli/           ← CLI binary (HTTP client only)
    │   ├── Cargo.toml
    │   └── src/
    │       ├── main.rs           ← clap CLI entrypoint
    │       ├── activity_service.rs   ← Thin HTTP wrapper for /api/v1/activities
    │       ├── category_service.rs   ← Thin HTTP wrapper for /api/v1/accounting-categories
    │       ├── category_mapper.rs    ← Static name-normalization table
    │       ├── csv_import.rs         ← CSV read + API-driven import
    │       └── csv_export.rs         ← Stub (not implemented)
    │
    ├── work-pulse-app/           ← React frontend (standalone npm package)
    │   ├── package.json
    │   ├── vite.config.js
    │   ├── jest.config.js
    │   ├── eslint.config.js
    │   ├── .prettierrc
    │   ├── .env                  ← VITE_API_BASE_URL (optional override)
    │   ├── index.html            ← Vite entry point
    │   ├── public/
    │   └── src/
    │       ├── main.jsx          ← React root, providers
    │       ├── setupTests.js     ← @testing-library/jest-dom matchers
    │       ├── app/
    │       │   ├── App.jsx       ← Layout + Routes
    │       │   └── Navigation.jsx
    │       ├── components/
    │       │   ├── activitiesTable.jsx
    │       │   └── errorMessage.jsx
    │       ├── config/
    │       │   └── api.js        ← API_BASE_URL constant
    │       ├── hooks/
    │       │   ├── useActivities.jsx
    │       │   └── useCategories.jsx
    │       ├── lib/
    │       │   ├── dateUtils.js
    │       │   └── dateUtils.test.js
    │       └── pages/
    │           ├── todaysActivities.jsx
    │           ├── activityLog.jsx
    │           ├── editActivity.jsx
    │           ├── dailyReport.jsx
    │           ├── weeklyReport.jsx
    │           ├── categoriesConfiguration.jsx
    │           └── importActivities.jsx
    │
    ├── work-pulse-service.Dockerfile
    └── work-pulse-app.Dockerfile
```

---

## 4. Core Systems

### 4.1 work-pulse-core — Domain Library

**Purpose:** Contains all business logic and domain types. Has zero dependency on Axum, HTTP, or any web concern. Designed to be consumed by both the service and, in principle, any other host.

**Clean Architecture layer map:**

```
entities/        → pure value types; no I/O, no traits
adapters/        → async_trait repository/importer contracts (depend only on entities)
use_cases/       → orchestrate entities through adapter traits
infra/           → concrete implementations of adapter traits
```

#### 4.1.1 Entities

**`Activity`** (`entities/activity.rs`)

The central domain object. Fields:

| Field | Type | Notes |
|---|---|---|
| `id` | `ActivityId(Uuid)` | newtype wrapper; generated with `Uuid::new_v4()` |
| `date` | `NaiveDate` | calendar date of work |
| `start_time` | `NaiveTime` | wall-clock start |
| `end_time` | `Option<NaiveTime>` | `None` for in-progress; duration is zero when absent |
| `accounting_category_id` | `AccountingCategoryId(Uuid)` | FK to category |
| `task` | `String` | free-form description |
| `comment` | `Option<String>` | present in DB and entity; not surfaced in daily/weekly report DTOs |

`duration()` computes `end_time - start_time`; returns `Duration::zero()` if `end_time` is `None`.

```rust
pub fn duration(&self) -> Duration {
    if let Some(end_time) = self.end_time() {
        *end_time - *self.start_time()
    } else {
        Duration::zero()
    }
}
```

**`AccountingCategory`** (`entities/accounting.rs`)

| Field | Type |
|---|---|
| `id` | `AccountingCategoryId(Uuid)` |
| `name` | `String` |

Both entity IDs are UUID newtypes with `parse_str` factory methods that return typed errors (`ActivityIdError`, `AccountingCategoryIdError`).

#### 4.1.2 Adapters (Trait Contracts)

`adapters/mod.rs` defines three traits:

**`ActivitiesListRepository`** — CRUD over activities:
- `get_all()`, `get_by_date()`, `get_by_date_range()`
- `add()`, `update()`, `delete()`, `delete_all()`, `delete_by_date_range()`

**`AccountingCategoriesListRepository`** — CRUD over categories plus:
- `get_or_create_by_name()` — used by the CSV importer to auto-provision categories

**`ActivitiesImporter`** — single method:
```rust
async fn import<R: Read + Send>(
    &mut self,
    reader: R,
    year: u16,
) -> Result<Vec<Activity>, ActivitiesImporterError>
```

All traits are `async_trait` + `Send + Sync`.

Error variants use `thiserror`:
- `ActivitiesListRepositoryError::NotFound(ActivityId)` / `::DatabaseError(String)`
- `AccountingCategoriesListRepositoryError::NotFound(AccountingCategoryId)` / `::DatabaseError(String)`
- `ActivitiesImporterError::ParseError` / `::RepositoryError(String)` / `::NoActivitiesToImport`

#### 4.1.3 Use Cases

**`ActivitiesList<R: ActivitiesListRepository>`**

Generic struct wrapping `Arc<Mutex<R>>`. Methods:
- `record(date, start_time, end_time, category_id, task, comment)` — creates and persists
- `activities()` — returns all
- `get_by_id(id)` — linear scan via `get_all()` (see Observations)
- `update(activity)`, `delete(id)`
- `delete_by_date_range(start, end)`
- `import<I, D>(importer, reader, year, replace_mode)` — orchestrates parse + optional pre-delete + batch insert, with structured tracing logs

`ReplaceMode` enum controls import deduplication:
```rust
pub enum ReplaceMode { None, All, ImportDateRange }
```

**`AccountingCategoriesList<R>`** — similar structure; enforces name-uniqueness at the use-case level (not DB-enforced in the in-memory repo; DB has a UNIQUE constraint).

**`DailyReport`** — value object built by fetching `repository.get_by_date(date)` and summing durations. Exposes `activities()` and `total_duration()`.

**`WeeklyReport`** — fetches a 7-day window (`week_start` to `week_start + 7 days`). Computes:
- `total_duration`
- `duration_per_category`: `Vec<(AccountingCategoryId, Duration)>` — aggregated totals
- `daily_durations_per_category`: `Vec<(NaiveDate, Vec<(AccountingCategoryId, Duration)>)>` — per-day breakdowns

**Note:** `WeeklyReport` uses a `HashMap` internally, so ordering in `duration_per_category` is non-deterministic. The service serializes to a JSON `HashMap<String, String>` so the client receives an unordered object.

#### 4.1.4 Infrastructure — Postgres Repositories

`PsqlConnection` wraps a `sqlx::PgPool` (cloneable) and is passed to both concrete repository types at construction time.

```rust
pub struct PsqlConnection {
    pool: PgPool,
}
impl PsqlConnection {
    pub async fn with_database_url(database_url: &str) -> Self {
        let pool = PgPool::connect(database_url).await.unwrap();
        Self::new(pool)
    }
}
```

Both `PsqlActivitiesListRepository` and `PsqlAccountingCategoriesListRepository` use raw `sqlx::query()` (not the `query!` macro, so no compile-time SQL verification). Rows are manually projected to entities via column-name indexing.

The `add_range` / `add_batch` path in `PsqlActivitiesListRepository` uses `sqlx::QueryBuilder` to build bulk INSERT statements in chunks of 100, used during CSV import.

Error handling is incomplete: `get_all`, `get_by_date`, `get_by_date_range`, and `add` all call `.unwrap()` on the sqlx result. A DB error in a read path will **panic the async task**, not return a clean error response.

#### 4.1.5 Infrastructure — In-Memory Repositories

Both use `Vec<Record>` where `Record` is an internal struct storing raw primitive types (not entity references), converting to/from domain entities on every access. This avoids `Clone` trait requirements on entities inside the `Arc<Mutex<_>>`.

```rust
struct ActivityRecord {
    id: Uuid,
    date: NaiveDate,
    start_time: NaiveTime,
    end_time: Option<NaiveTime>,
    accounting_category_id: AccountingCategoryId,
    task: String,
    comment: Option<String>,
}
```

`get_all()` clones and converts every record on every call. Fine for personal use; would be expensive at scale.

#### 4.1.6 Infrastructure — CSV Importer

`CsvActivitiesImporter` implements `ActivitiesImporter`. Expected CSV columns:

| Column | Format |
|---|---|
| `CW` | integer (calendar week, informational only) |
| `Date` | `dd.mm.` (year supplied separately as `year: u16`) |
| `Check In` | `HH:MM` |
| `Check Out` | `HH:MM` |
| `PAM Category` | string — looked up / auto-created in category repo |
| `Topic` | mapped to `task` |
| `Comment` | optional text |

Date conversion: `"15.03." + "2023"` → `"2023-03-15"` via `NaiveDate::parse_from_str("%d.%m.%Y")`.

The importer maintains an in-memory cache of categories fetched at the start of the import to minimize repository calls. New category names are auto-created via `get_or_create_by_name`.

---

### 4.2 work-pulse-service — REST API

**Purpose:** An Axum web server that exposes the work-pulse-core use cases over a JSON REST API with integrated OpenAPI/Swagger documentation.

**Startup flow in `main.rs`:**

1. Parse `RUST_LOG` and initialize `tracing_subscriber`
2. Parse `--use-in-memory-repositories` CLI flag via `clap`
3. Create the appropriate pair of repository `Arc<Mutex<_>>` instances
4. Build the `OpenApiRouter` using `utoipa_axum`
5. Merge Swagger UI at `/swagger-ui`
6. Apply CORS (`Any` origin/methods/headers) and `TraceLayer`
7. Bind `0.0.0.0:8080` and start `axum::serve`

**Connection string is hardcoded:**
```rust
const CONNECTION_STRING: &str =
    "postgres://workpulse:supersecret@localhost:5432/workpulse";
```
The service does **not** read `DATABASE_URL` from the environment. Docker Compose injects it but the service ignores it (see Observations).

**Route table:**

| Method | Path | Handler | Description |
|---|---|---|---|
| `GET` | `/api/v1/activities` | `list_activities` | All activities; optional `?start_date=&end_date=` |
| `POST` | `/api/v1/activities` | `create_activity` | Create new activity |
| `GET` | `/api/v1/activities/{id}` | `get_activity_by_id` | Fetch single |
| `PUT` | `/api/v1/activities` | `update_activity` | **ID in body**, not URL |
| `DELETE` | `/api/v1/activities/{id}` | `delete_activity` | Delete by ID |
| `PUT` | `/api/v1/activities/upload-csv` | `upload_activities_csv_raw` | Import CSV as raw body |
| `POST` | `/api/v1/activities/upload-csv` | `upload_activities_csv_multipart` | Import CSV as multipart |
| `GET` | `/api/v1/accounting-categories` | `list_accounting_categories` | All categories |
| `POST` | `/api/v1/accounting-categories` | `create_accounting_category` | Create |
| `PUT` | `/api/v1/accounting-categories` | `update_accounting_category` | **ID in body** |
| `DELETE` | `/api/v1/accounting-categories/{id}` | `delete_accounting_category` | Delete (cascades to activities in DB) |
| `GET` | `/api/v1/daily-report` | `generate_daily_report` | `?report_date=YYYY-MM-DD` |
| `GET` | `/api/v1/weekly-report` | `generate_weekly_report` | `?week_start_date=YYYY-MM-DD` |

**Service state threading model:**

Each service module receives `Arc<Mutex<R>>` repository references. For `activities_list_service`, there is a two-repo state struct:
```rust
struct ActivitiesServiceState<R, T> {
    activities_list_repository: Arc<Mutex<R>>,
    accounting_categories_repository: Arc<Mutex<T>>,
}
```
This state is itself wrapped in `Arc<Mutex<_>>`, meaning the entire activities service state is locked for the duration of each request handler. Concurrent requests serialize completely (see Observations).

**DTO layer:**

Each service module defines its own DTO structs with `from_entity`/`to_entity` converters. These are `Serialize + Deserialize + ToSchema` for JSON and OpenAPI generation.

The `Activity` DTO uses `Option<String>` for `id` — `None` in POST (create) requests, `Some(uuid_string)` in PUT (update) and GET responses. All timestamps are plain strings, not typed.

**Filter behavior (`list_activities`):**

The date-range filter on `GET /api/v1/activities` is applied in-process after fetching all records from the repository. The repo always returns everything; filtering happens at the service layer via string comparison on `activity.date`. Correct (ISO 8601 dates sort lexicographically) but inefficient.

**OpenAPI:** Tags are defined in `main.rs` `prelude` module. Spec at `/api-docs/openapi.json`, Swagger UI at `/swagger-ui`.

---

### 4.3 work-pulse-cli — CLI Tool

**Purpose:** A command-line utility for importing activities from a CSV file by calling the running REST API. Export is stubbed.

**Commands:**
```
work-pulse-cli csv-import --file <path>   # Reads CSV, creates categories + activities via API
work-pulse-cli csv-export --file <path>   # Stub; prints filename and returns Ok
```

**Import flow (`csv_import::import`):**

1. Read file bytes, decode from **Latin-1** (using `encoding_rs`) to handle Windows-encoded CSVs
2. Parse rows into `ActivityTableRecord` via `csv::Reader::deserialize`
3. Enumerate unique PAM categories in the CSV
4. Map each through the static `CATEGORY_MAP` (e.g., `"CurrentVersion"` → `"Current Version"`)
5. Check against `GET /api/v1/accounting-categories`; create any missing via `POST`
6. Re-fetch categories (now complete)
7. For each CSV row: convert date, resolve `pam_category_id`, call `POST /api/v1/activities`

**Design notes:**
- Year is **hardcoded** to `"2025"` (`const ACTIVITIES_YEAR: &str = "2025"`), while the service's importer accepts a configurable `year: u16`
- The CLI does not use `work-pulse-core` at all — it re-implements its own HTTP-client-based DTOs via `reqwest::blocking`
- Service URL is hardcoded: `http://localhost:8080/api/v1/activities`
- Each activity is created with a separate HTTP round-trip; no batching
- `category_mapper.rs` has 5 Siemens Healthineers-specific entries left in the mapping table

---

### 4.4 work-pulse-app — React Frontend

**Purpose:** A browser-based SPA for interacting with the work-pulse REST API. Provides forms to record activities, browse the log, configure categories, import CSV files, and view daily/weekly reports.

#### 4.4.1 Entry Point and Providers

`src/main.jsx`:
```jsx
createRoot(document.getElementById('root')).render(
  <StrictMode>
    <CssVarsProvider>       {/* MUI Joy theme + CSS variables */}
      <CssBaseline />
      <Router>              {/* react-router-dom BrowserRouter */}
        <App />
      </Router>
    </CssVarsProvider>
  </StrictMode>,
)
```

No global state management library (no Redux, Zustand, Context); state is colocated in pages via hooks.

#### 4.4.2 Layout

`App.jsx` defines a fixed two-panel layout:
- Left sidebar (300px): `Navigation.jsx` — vertical stack of route links, active route shown with `variant="solid"`
- Right content area: React Router `<Routes>` rendering the active page

#### 4.4.3 Routing

| Path | Component | Purpose |
|---|---|---|
| `/` | `TodaysActivities` | Default — today's activity list + create form |
| `/activities` | `TodaysActivities` | Same |
| `/activities/edit/:id` | `EditActivity` | Edit an existing activity |
| `/activities/log` | `ActivityLog` | Date-range browseable log, grouped by week |
| `/categories` | `CategoriesConfiguration` | CRUD for accounting categories |
| `/import` | `ImportActivities` | Upload a CSV file for import |
| `/daily-report` | `DailyReport` | Daily time summary table |
| `/weekly-report` | `WeeklyReport` | Weekly time summary matrix by category |

A "Yearly Report" button is visible in the navigation but has no route wired up — it is non-functional.

#### 4.4.4 Data Hooks

**`useActivities`** (`hooks/useActivities.jsx`):
- State: `activities[]`, `loading`, `error`
- `refreshActivities(startDate, endDate)` — `GET /api/v1/activities?start_date=&end_date=`
- `createActivity(data)` — `POST /api/v1/activities`
- `updateActivity(id, data)` — `PUT /api/v1/activities` (id in body via data)
- `deleteActivity(id)` — optimistic removal from local state, then `DELETE /api/v1/activities/:id`, reverts on failure

**`useCategories`** (`hooks/useCategories.jsx`):
- State: `categories[]`, `loading`, `error`
- `refreshCategories()` — `GET /api/v1/accounting-categories`
- Read-only; mutations are done directly with `axios` in `categoriesConfiguration.jsx`

#### 4.4.5 API Configuration

`src/config/api.js`:
```js
const API_BASE_URL = import.meta.env.VITE_API_BASE_URL || 'http://localhost:8080'
```

In development, `VITE_API_BASE_URL` is unset, so requests go to `localhost:8080`. In Docker, Vite bakes this value into the bundle at build time — the Docker Compose `VITE_API_BASE_URL` env var is a run-time env var and has no effect after the build (see Observations).

#### 4.4.6 Pages

**TodaysActivities** — inline activity creation form (date, start/end time, category select, task, comment) + `ActivitiesTable`. Default date is today. Activities sorted descending by `start_time`.

**ActivityLog** — date-range picker defaulting to the current month. Activities grouped by ISO week using `groupActivitiesByWeek()`. Sorted by date then start_time within each week.

**EditActivity** — fetches the activity by ID on mount, pre-populates form fields, saves via `PUT /api/v1/activities` (ID in body), navigates back to `/activities` on success.

**DailyReport** — date picker, calls `GET /api/v1/daily-report?report_date=`. Renders a table of activities sorted by start_time. Total duration colored by threshold: green 8–10h, orange < 8h, red > 10h.

**WeeklyReport** — HTML `<input type="week">` defaulting to current ISO week. Calls `GET /api/v1/weekly-report?week_start_date=`. Renders a category × day matrix. Totals colored: green 40–50h, orange < 40h, red > 50h.

**ImportActivities** — file picker + year input. Submits `multipart/form-data` to `POST /api/v1/activities/upload-csv?activities_year=<year>`. Does not support `replace_mode` — always uses default (none).

**CategoriesConfiguration** — full CRUD inline table with inline editing. Uses `PUT /api/v1/accounting-categories` with `{ id, name }` in the body.

#### 4.4.7 Components

**`ActivitiesTable`** — reusable table rendering an array of activities. Accepts `onEditActivity` and `onDeleteActivity` callbacks. Formats date via `formatDateForDisplay`.

**`ErrorMessage`** — renders a `Typography` with `color="danger"` if `message` is truthy; renders nothing otherwise.

#### 4.4.8 dateUtils Library

`src/lib/dateUtils.js` contains pure date/time utility functions with full JSDoc:

| Function | Purpose |
|---|---|
| `getWeekNumber(date)` | ISO 8601 week number (1–53) |
| `getWeekRange(date)` | Monday/Sunday dates for the week of `date` |
| `groupActivitiesByWeek(activities)` | Groups activity array by `YYYY-Www` key |
| `getCurrentMonthRange()` | `{start, end}` for the current month |
| `formatDuration(isoDuration)` | `"PT10800S"` → `"03:00"` |
| `formatDateForDisplay(dateString)` | `"2025-10-15"` → locale-aware long form |
| `durationToMinutes(duration)` | `"01:30"` → `90` |
| `getCurrentWeek()` | `"YYYY-Www"` for current week |
| `formatWeekForDisplay(weekString)` | `"2025-W42"` → human-readable range string |
| `getWeekStartDate(weekString)` | Returns `Date` object for Monday of given week |

This is the only frontend module with tests (`dateUtils.test.js`), which are thorough (30+ cases covering ISO week boundaries, leap years, cross-year spanning).

---

## 5. Data Flow

### 5.1 Recording a New Activity

```
Browser (TodaysActivities)
  │  POST /api/v1/activities
  │  { date, start_time, end_time?, accounting_category_id, task, comment? }
  ▼
work-pulse-service: create_activity handler
  │  locks Arc<Mutex<ActivitiesServiceState>>
  │  builds ActivitiesList use case
  │  calls activities_list.record(...)
  ▼
ActivitiesList::record()
  │  locks Arc<Mutex<R>> (repository)
  │  creates Activity entity (new random UUID)
  │  calls repository.add(activity)
  ▼
PsqlActivitiesListRepository::add()  OR  InMemoryActivitiesListRepository::add()
  │  INSERT INTO activities (id, date, start_time, end_time, category_id, task, comment)
  ▼
PostgreSQL / in-memory Vec
  │
  ◄ Activity entity returned through call stack
  │
work-pulse-service: serializes to Activity DTO
  │  returns HTTP 201 { id, date, start_time, end_time, accounting_category_id, task, comment }
  ▼
Browser: useActivities adds to local state array
```

### 5.2 CSV Import via Web UI

```
Browser (ImportActivities)
  │  POST /api/v1/activities/upload-csv?activities_year=2025
  │  multipart/form-data { file: <csv bytes> }
  ▼
work-pulse-service: upload_activities_csv_multipart
  │  extracts "file" field from multipart
  │  creates CsvActivitiesImporter (with accounting_categories_repository)
  │  calls activities_list.import(importer, csv_bytes, year, ReplaceMode::None)
  ▼
ActivitiesList::import()
  │  calls importer.import(reader, year) → Vec<Activity>
  ▼
CsvActivitiesImporter::import()
  │  parses CSV rows
  │  fetches all categories from category repo
  │  for each row: get_or_create category, build Activity
  │  returns Vec<Activity>
  ▼
ActivitiesList::import() continued
  │  (no pre-delete for ReplaceMode::None)
  │  for each activity: repository.add(activity)
  ▼
DB / memory
```

### 5.3 Weekly Report Generation

```
Browser (WeeklyReport)
  │  GET /api/v1/weekly-report?week_start_date=2025-10-13
  ▼
work-pulse-service: generate_weekly_report
  │  locks repository
  │  calls WeeklyReport::new(week_start_date, &*repository)
  ▼
WeeklyReport::new()
  │  repository.get_by_date_range(week_start, week_start + 7 days)
  │  sums total_duration
  │  builds duration_per_category HashMap
  │  builds daily_durations_per_category per day
  ▼
work-pulse-service: maps to WeeklyReport DTO
  │  converts AccountingCategoryId → string keys
  │  converts Duration → ISO 8601 string (e.g., "PT10800S")
  │  returns HTTP 200 JSON
  ▼
Browser: renders category × day matrix
  │  resolves category IDs to names via useCategories hook
  │  formats durations via formatDuration()
```

### 5.4 CLI CSV Import

```
CLI: csv-import --file activities.csv
  │  reads file, decodes Latin-1
  │  parses CSV rows
  │  checks categories against GET /api/v1/accounting-categories
  │  creates missing categories via POST /api/v1/accounting-categories
  │  for each row: POST /api/v1/activities (1 request per row)
  │  (hardcoded year 2025, no replace mode)
```

---

## 6. Key Data Structures

### 6.1 Domain Entities (Rust)

```rust
// Entity — core work item
pub struct Activity {
    id: ActivityId,                           // newtype UUID
    date: NaiveDate,
    start_time: NaiveTime,
    end_time: Option<NaiveTime>,              // None = in-progress
    accounting_category_id: AccountingCategoryId,
    task: String,
    comment: Option<String>,
}

// Category — time-booking bucket
pub struct AccountingCategory {
    id: AccountingCategoryId,                 // newtype UUID
    name: String,
}

// ID newtypes (pattern repeated for both entity types)
pub struct ActivityId(pub Uuid);
pub struct AccountingCategoryId(pub Uuid);
```

### 6.2 Repository Traits (Rust)

```rust
#[async_trait]
pub trait ActivitiesListRepository: Send + Sync {
    async fn get_all(&self) -> Vec<Activity>;
    async fn get_by_date(&self, date: NaiveDate) -> Vec<Activity>;
    async fn get_by_date_range(&self, start: NaiveDate, end: NaiveDate) -> Vec<Activity>;
    async fn add(&mut self, activity: Activity);
    async fn update(&mut self, activity: Activity) -> Result<(), ActivitiesListRepositoryError>;
    async fn delete(&mut self, id: ActivityId) -> Result<(), ActivitiesListRepositoryError>;
    async fn delete_all(&mut self) -> Result<(), ActivitiesListRepositoryError>;
    async fn delete_by_date_range(&mut self, start: NaiveDate, end: NaiveDate)
        -> Result<usize, ActivitiesListRepositoryError>;
}
```

### 6.3 Service DTOs (Rust)

```rust
// activities_list_service.rs
struct Activity {
    id: Option<String>,       // None on POST, Some on PUT/GET
    date: String,             // "YYYY-MM-DD"
    start_time: String,       // "HH:MM:SS"
    end_time: Option<String>, // "HH:MM:SS"
    accounting_category_id: String,
    task: String,
    comment: Option<String>,
}

// weekly_report_service.rs
struct WeeklyReport {
    week_start: String,
    week_end: String,
    total_duration: String,                              // ISO 8601, e.g. "PT10800S"
    duration_per_category: HashMap<String, String>,     // uuid → duration
    daily_durations_per_category: HashMap<String, HashMap<String, String>>, // date → uuid → duration
}
```

### 6.4 Database Schema (PostgreSQL)

```sql
CREATE TABLE accounting_categories (
    id         UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name       VARCHAR(255) NOT NULL UNIQUE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE activities (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    date        DATE NOT NULL,
    start_time  TIME NOT NULL,
    end_time    TIME,                       -- nullable: activity may be ongoing
    category_id UUID NOT NULL
              REFERENCES accounting_categories(id) ON DELETE CASCADE,
    task        TEXT NOT NULL,
    comment     TEXT,
    created_at  TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at  TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Indexes
idx_accounting_categories_name  ON accounting_categories(name)
idx_activities_date             ON activities(date)
idx_activities_category_id      ON activities(category_id)
idx_activities_date_category    ON activities(date, category_id)
```

### 6.5 Frontend Activity Object (JS)

```js
// As received from GET /api/v1/activities
{
  id: "550e8400-e29b-41d4-a716-446655440000",
  date: "2025-10-15",
  start_time: "09:00:00",
  end_time: "10:30:00",      // null if ongoing
  accounting_category_id: "...",
  task: "Code Review",
  comment: null              // or string
}
```

### 6.6 CSV Import Format

Expected columns (both CLI and web upload):

```
CW | Date   | Check In | Check Out | PAM Category | Topic        | Comment
11 | 15.03. | 09:00    | 17:00     | Development  | Coding       | Worked on project X
```

- `Date` is `dd.mm.` — year supplied externally
- The `Duration` column shown in the frontend's file format docs does not exist in the actual `ActivityTableRecord` deserialization struct and is silently ignored if present

---

## 7. Architectural Patterns

### 7.1 Clean Architecture (Dependency Rule)

The `work-pulse-core` crate strictly enforces the Clean Architecture dependency rule:

```
entities (no deps on outer layers)
    ↑
adapters (depends on entities only)
    ↑
use_cases (depends on entities + adapters)
    ↑
infra (depends on everything; implements adapter traits)
```

The service (`work-pulse-service`) sits outside the core and depends only on `work-pulse-core`. It only touches `infra` at startup in `main.rs` to instantiate concrete types, then injects them via generics.

### 7.2 Repository Pattern with Dual Implementation

Every persistence trait has two implementations:

| Trait | Production | Test/Dev |
|---|---|---|
| `ActivitiesListRepository` | `PsqlActivitiesListRepository` | `InMemoryActivitiesListRepository` |
| `AccountingCategoriesListRepository` | `PsqlAccountingCategoriesListRepository` | `InMemoryAccountingCategoriesListRepository` |

Selection happens at process startup via the `--use-in-memory-repositories` CLI flag. All unit tests use the in-memory implementations.

### 7.3 Generic Use Cases via `Arc<Mutex<R>>`

Use case structs are generic over the repository type:

```rust
pub struct ActivitiesList<R> {
    repository: Arc<Mutex<R>>,
}

impl<R: ActivitiesListRepository> ActivitiesList<R> { ... }
```

This avoids dynamic dispatch (`Box<dyn Trait>`) in the use case layer. The service layer wires them up generically:

```rust
fn create_open_api_router<R, T>(
    accounting_categories_repository: Arc<Mutex<R>>,
    activities_list_repository: Arc<Mutex<T>>,
) -> OpenApiRouter
where
    R: AccountingCategoriesListRepository + Send + Sync + 'static,
    T: ActivitiesListRepository + Send + Sync + 'static,
```

### 7.4 OpenAPI-First with utoipa

Every handler has `#[utoipa::path(...)]` annotation. The `OpenApiDoc` is assembled at startup and exposed at `/api-docs/openapi.json`. Swagger UI is served at `/swagger-ui`.

### 7.5 Optimistic UI Updates

`useActivities.deleteActivity` implements optimistic UI: immediately removes from local state, then calls DELETE, reverts on failure:

```js
const originalActivities = [...activities]
setActivities((prev) => prev.filter((act) => act.id !== activityId))
try {
    await axios.delete(...)
} catch (error) {
    setActivities(originalActivities) // Revert
}
```

### 7.6 ISO 8601 Duration Strings

`chrono::Duration::to_string()` produces ISO 8601 duration format (e.g., `PT10800S`). The frontend's `formatDuration` parses this to render `HH:MM`:

```js
const regex = /PT(?:(\d+)H)?(?:(\d+)M)?(?:(\d+)S)?/
```

This is a coupling between Rust's chrono formatting and a frontend regex. The regex handles all valid ISO 8601 duration formats correctly.

### 7.7 Colocation of Tests

All Rust unit tests are `#[cfg(test)]` modules inside the source files they test. No separate `tests/` directory — idiomatic Rust with white-box test access.

---

## 8. File Index

### Backend — Key Files

| File | Role |
|---|---|
| `src/Cargo.toml` | Workspace manifest (all 3 crates) |
| `src/Cargo.docker.toml` | Docker workspace (core + service only) |
| `src/work-pulse-core/src/lib.rs` | Crate root, re-exports 4 modules |
| `src/work-pulse-core/src/entities/activity.rs` | `Activity` + `ActivityId` domain types |
| `src/work-pulse-core/src/entities/accounting.rs` | `AccountingCategory` + `AccountingCategoryId` |
| `src/work-pulse-core/src/adapters/mod.rs` | All repository and importer traits |
| `src/work-pulse-core/src/use_cases/activities_list.rs` | `ActivitiesList`, `ReplaceMode` |
| `src/work-pulse-core/src/use_cases/accounting_categories_list.rs` | `AccountingCategoriesList` |
| `src/work-pulse-core/src/use_cases/daily_report.rs` | `DailyReport` |
| `src/work-pulse-core/src/use_cases/weekly_report.rs` | `WeeklyReport` |
| `src/work-pulse-core/src/infra/repositories/postgres/mod.rs` | `PsqlConnection` |
| `src/work-pulse-core/src/infra/repositories/postgres/activities_list.rs` | Postgres activities repo |
| `src/work-pulse-core/src/infra/repositories/postgres/accounting_categories_list.rs` | Postgres categories repo |
| `src/work-pulse-core/src/infra/repositories/in_memory/activities_list.rs` | In-memory activities repo |
| `src/work-pulse-core/src/infra/repositories/in_memory/accounting_categories_list.rs` | In-memory categories repo |
| `src/work-pulse-core/src/infra/importers/csv_activities_importer.rs` | CSV importer |
| `src/work-pulse-service/src/main.rs` | Startup, DI, routing, CORS |
| `src/work-pulse-service/src/services/activities_list_service.rs` | Activities CRUD + CSV upload endpoints |
| `src/work-pulse-service/src/services/accounting_categories_service.rs` | Category CRUD endpoints |
| `src/work-pulse-service/src/services/daily_report_service.rs` | Daily report endpoint |
| `src/work-pulse-service/src/services/weekly_report_service.rs` | Weekly report endpoint |
| `src/work-pulse-cli/src/main.rs` | CLI entry point |
| `src/work-pulse-cli/src/csv_import.rs` | CSV import logic |
| `src/work-pulse-cli/src/category_mapper.rs` | Name normalization table |
| `src/work-pulse-cli/src/activity_service.rs` | HTTP client for activities |
| `src/work-pulse-cli/src/category_service.rs` | HTTP client for categories |
| `src/work-pulse-cli/src/csv_export.rs` | Stub |

### Database

| File | Role |
|---|---|
| `db/migrations/20241016000001_create_accounting_categories.sql` | Creates `accounting_categories` table |
| `db/migrations/20241016000002_create_activities.sql` | Creates `activities` table with FK + indexes |
| `db/migrations/20241016000003_insert_default_categories.sql` | Seeds 9 default categories |
| `db/schema.sql` | Auto-generated snapshot from DbMate |

### Frontend — Key Files

| File | Role |
|---|---|
| `src/work-pulse-app/src/main.jsx` | Root render, providers (CssVarsProvider, Router) |
| `src/work-pulse-app/src/app/App.jsx` | Layout shell + all Routes |
| `src/work-pulse-app/src/app/Navigation.jsx` | Sidebar navigation |
| `src/work-pulse-app/src/config/api.js` | `API_BASE_URL` constant |
| `src/work-pulse-app/src/hooks/useActivities.jsx` | Activity CRUD state + API calls |
| `src/work-pulse-app/src/hooks/useCategories.jsx` | Category fetch state + API call |
| `src/work-pulse-app/src/lib/dateUtils.js` | Date utility functions |
| `src/work-pulse-app/src/lib/dateUtils.test.js` | Tests for dateUtils |
| `src/work-pulse-app/src/pages/todaysActivities.jsx` | Today's activity create + list |
| `src/work-pulse-app/src/pages/activityLog.jsx` | Date-range log grouped by week |
| `src/work-pulse-app/src/pages/editActivity.jsx` | Edit single activity |
| `src/work-pulse-app/src/pages/dailyReport.jsx` | Daily time report |
| `src/work-pulse-app/src/pages/weeklyReport.jsx` | Weekly category × day matrix |
| `src/work-pulse-app/src/pages/categoriesConfiguration.jsx` | Category CRUD UI |
| `src/work-pulse-app/src/pages/importActivities.jsx` | CSV upload UI |
| `src/work-pulse-app/src/components/activitiesTable.jsx` | Reusable activity table |
| `src/work-pulse-app/src/components/errorMessage.jsx` | Conditional error display |
| `src/work-pulse-app/vite.config.js` | Vite config (minimal) |
| `src/work-pulse-app/jest.config.js` | Jest config with babel-jest transform |
| `src/work-pulse-app/.prettierrc` | Code style settings |
| `src/work-pulse-app/eslint.config.js` | ESLint 9 flat config |

### Infrastructure

| File | Role |
|---|---|
| `docker-compose.yml` | Full stack: postgres + backend + frontend + migrate |
| `src/work-pulse-service.Dockerfile` | Multi-stage Rust build → debian:trixie-slim |
| `src/work-pulse-app.Dockerfile` | Multi-stage Node build → nginx:alpine |
| `.github/workflows/ci.yml` | GitHub Actions: build + test (Rust only) |
| `.env` | `DATABASE_URL` for DbMate (local dev) |
| `src/work-pulse-app/.env` | `VITE_API_BASE_URL` (optional) |
| `build.cmd` | Builds both Docker images |
| `db-migrate.cmd` | DbMate CLI wrapper |
| `db-migrate-docker.cmd` | DbMate Docker wrapper |
| `work-pulse-db.cmd` | Starts postgres:16 standalone container |
| `work-pulse-service.cmd` | Runs pre-built service container |
| `clean-data-folder.cmd` | Deletes `./data/` (DB volume) |

---

## 9. Configuration

### 9.1 Runtime Configuration

| Variable | Where consumed | Effect |
|---|---|---|
| `RUST_LOG` | `work-pulse-service` (env var at runtime) | Logging level; default `"debug"` |
| `--use-in-memory-repositories` | `work-pulse-service` (CLI flag) | Switches from Postgres to in-memory repos |
| `CONNECTION_STRING` (hardcoded) | `work-pulse-service/src/main.rs` | Always `postgres://workpulse:supersecret@localhost:5432/workpulse` |
| `DATABASE_URL` | DbMate only (`.env` + Docker Compose) | Used only for migrations; the service does not read this |
| `VITE_API_BASE_URL` | Vite (build-time env) | Frontend API base URL; default `http://localhost:8080` |

### 9.2 Prettier (Frontend)

```json
{
  "singleQuote": true,
  "trailingComma": "all",
  "printWidth": 100,
  "tabWidth": 2,
  "semi": false
}
```

### 9.3 Jest

- Test environment: `jsdom`
- Setup: `src/setupTests.js` → `@testing-library/jest-dom`
- Transform: `babel-jest` with `@babel/preset-env` (targeting current Node) + `@babel/preset-react` (automatic runtime)
- Path alias: `@/` → `src/`
- Coverage collected from `src/**/*.{js,jsx}` excluding `main.jsx`

### 9.4 ESLint

ESLint 9 flat config targeting `**/*.{js,jsx}`. Plugins: `eslint-plugin-react-hooks`, `eslint-plugin-react-refresh`. `no-unused-vars` ignores names matching `^[A-Z_]`.

---

## 10. Build & Deploy

### 10.1 Local Development (No Docker)

```cmd
# 1. Start Postgres
.\work-pulse-db.cmd

# 2. Apply migrations
.\db-migrate.cmd up

# 3. Start backend (in-memory for no-DB dev)
cd src\work-pulse-service
cargo run -- --use-in-memory-repositories

# 4. Start frontend
cd src\work-pulse-app
npm run dev          # Vite on http://localhost:5173
```

### 10.2 Docker Compose (Full Stack)

```cmd
.\build.cmd
docker compose up -d
.\db-migrate-docker.cmd up
```

Docker Compose services:

| Service | Image | Port | Notes |
|---|---|---|---|
| `work-pulse-db` | `postgres:16` | 5432 | Health-checked; volume at `./data/` |
| `work-pulse-backend` | `work-pulse-service:latest` | 8080 | Depends on healthy DB; `DATABASE_URL` injected but not read by service |
| `work-pulse-frontend` | `work-pulse-app:latest` | 3000 | nginx serving Vite build; `VITE_API_BASE_URL` injected but baked at build time |
| `work-pulse-migrate` | `amacneil/dbmate` | — | Profile `migration`; shares `./db/` volume |

### 10.3 Rust Build Details

- Dockerfile copies `Cargo.docker.toml` as `Cargo.toml`, excluding the CLI crate
- Builds with `cargo build --package work-pulse-service --release`
- Optional `INCLUDE_CA=true` build arg copies certificates from `src/certificates/` into the image
- Final image is `debian:trixie-slim`; only the compiled binary is copied

### 10.4 Frontend Build Details

- Node 18 Alpine: `npm ci` then `npm run build` (Vite)
- Output placed at `/app/dist`, served by `nginx:alpine`
- No custom nginx config

### 10.5 CI (GitHub Actions)

```yaml
on:
  push:
    branches: ["main"]
  pull_request:
    branches: ["main"]

steps:
  - checkout
  - cargo build --workspace --verbose    # from ./src
  - cargo test --workspace --verbose     # from ./src
```

The CI pipeline covers only the Rust backend. The frontend has no CI steps.

---

## 11. External Integrations

| Integration | Mechanism | Details |
|---|---|---|
| PostgreSQL | `sqlx` async pool | Connection string hardcoded; TLS via `runtime-tokio-rustls` feature |
| Swagger UI | `utoipa-swagger-ui` | Bundled static assets at `/swagger-ui`; spec at `/api-docs/openapi.json` |
| DbMate | External CLI / Docker image | Used only for schema migrations; reads `DATABASE_URL` |
| nginx | Docker image | Serves the built React SPA; no custom config |

No external SaaS services, no OAuth, no email, no message queues.

---

## 12. Observations & Recommendations

### Critical Issues

**1. Hardcoded connection string vs. Docker Compose `DATABASE_URL`**
The service reads `CONNECTION_STRING` from a Rust `const` pointing to `localhost:5432`. Running the pre-built Docker image in the compose stack will attempt to connect to `localhost:5432` *inside the container*, which fails. The service should read `std::env::var("DATABASE_URL")` and fall back to the hardcoded default.

**2. Panics on DB errors in read paths**
`PsqlActivitiesListRepository::get_all`, `get_by_date`, `get_by_date_range`, and `add` all call `.unwrap()` on the sqlx future. Any database error panics the Tokio task and returns a 500 with no useful body. All repository methods should return `Result<_, ActivitiesListRepositoryError>`.

**3. `VITE_API_BASE_URL` baked at build time, not runtime**
Vite replaces `import.meta.env.VITE_*` values at build time. The Docker Compose `VITE_API_BASE_URL` is set at container run time, after the build — so it has no effect. Additionally, the browser makes requests from the user's machine, not inside the Docker network, so `work-pulse-backend` is not resolvable from the browser anyway. A build arg needs to be passed at `docker build` time, or a runtime config injection technique (e.g., writing a `window.ENV` script via nginx) should be used.

**4. Entire activities service state locked per request**
`ActivitiesServiceState<R, T>` is wrapped in `Arc<Mutex<_>>`. This single lock gates all concurrent requests to every activities endpoint. The repositories themselves are already `Arc<Mutex<_>>`; the outer state lock is unnecessary and should be removed.

### Design Issues

**5. `get_by_id` linear scan via `get_all`**
`ActivitiesList::get_by_id` fetches all activities then filters in-process. For Postgres this is a full table scan for every single-item lookup (e.g., the edit page). A `get_by_id` method should be added to the trait and implemented with `WHERE id = $1`.

**6. `list_activities` in-process date filtering**
`GET /api/v1/activities?start_date=&end_date=` fetches all activities from the DB and filters in Rust. The repository trait already has `get_by_date_range`; the service handler should call it instead.

**7. CLI year hardcoded to 2025**
`csv_import.rs` has `const ACTIVITIES_YEAR: &str = "2025"`. This should be a `--year` CLI argument, mirroring the service's `activities_year` query parameter.

**8. `csv_export` is a complete stub**
`csv_export.rs` prints a message and returns `Ok(())` with no implementation. Users who try `work-pulse-cli csv-export` will get no output and no error.

**9. Non-functional "Yearly Report" navigation button**
`Navigation.jsx` renders a "Yearly Report" `Button` with no `component={Link}` and no `to=` prop. Clicking it does nothing.

**10. Inconsistent error handling in service handlers**
Several parsing calls use `.expect("Invalid ... format")` instead of returning HTTP 400 responses. Malformed UUID or date strings in request bodies will panic the handler.

### Minor / Quality Issues

**11. No authentication or authorization**
Fully open CORS and no auth layer. Acceptable for a local personal tool; needs addressing before any network exposure.

**12. `comment` field note in AGENTS.md is inaccurate**
AGENTS.md states `activities.comment` is "not mapped in the Rust `Activity` entity." The field is fully mapped in both the entity and the Postgres repository. It is only absent from the `DailyReportActivity` DTO.

**13. Frontend CI gap**
The CI pipeline has no frontend steps. Adding `npm run lint` and `npm test` would catch regressions.

**14. `WeeklyReport` week_end boundary**
The weekly report fetches `week_start` to `week_start + 7 days` inclusive. Since the DB range uses `BETWEEN`, the 8th day (same weekday as `week_start`) is included, meaning a Monday-started week inadvertently includes the following Monday.

**15. Import page documents a `Duration` column that does not exist**
`importActivities.jsx` lists `Duration (format: HH:MM:SS)` as a required CSV column, but `ActivityTableRecord` has no `Duration` field — it is silently ignored. The documentation should be corrected.

**16. Workspace resolver discrepancy in AGENTS.md**
AGENTS.md says `resolver = "1"` but `src/Cargo.toml` uses `resolver = "2"`.

### Positive Observations

- The Clean Architecture layering is well-executed. The dependency rule is never violated, and the dual-repo pattern makes all domain logic unit-testable without a database.
- The in-memory repositories are thorough implementations, enabling realistic use-case tests.
- Unit tests are comprehensive for all domain entities, use cases, and the CSV importer.
- `dateUtils.js` is exceptionally well-tested — 30+ test cases covering ISO week boundaries, leap years, and cross-year spanning.
- The `ReplaceMode` enum for CSV import is a clean design expressing three distinct semantics without boolean flags.
- Tracing instrumentation in `ActivitiesList::import` logs timing for parse and DB phases.
- The Docker Compose `healthcheck` on Postgres ensures the backend only starts after the DB is ready.
