# Work Pulse — Comprehensive Feature Analysis

## 1. Core Domain Entities

### `Activity` (`work-pulse-core/src/entities/activity.rs`)

The central domain object representing a unit of work in a working day.

| Field | Type | Notes |
|---|---|---|
| `id` | `ActivityId` (UUID newtype) | Auto-generated v4 UUID |
| `date` | `NaiveDate` | ISO 8601 calendar date |
| `start_time` | `NaiveTime` | Required; 24 h clock |
| `end_time` | `Option<NaiveTime>` | Nullable — an activity without an end time has zero duration |
| `accounting_category_id` | `AccountingCategoryId` | Foreign key to a category |
| `task` | `String` | Free-text task description |
| `comment` | `Option<String>` | Optional note — exists in the entity and is fully wired end-to-end |

Key behaviour: `duration()` returns `end_time - start_time`; zero if `end_time` is `None`.

### `AccountingCategory` (`work-pulse-core/src/entities/accounting.rs`)

A label used to group activities for reporting purposes (analogous to a project or work-package code).

| Field | Type | Notes |
|---|---|---|
| `id` | `AccountingCategoryId` (UUID newtype) | Auto-generated v4 UUID |
| `name` | `String` | Unique per the DB constraint |

---

## 2. Backend REST API Endpoints

Base prefix: `/api/v1/`. Server always binds on `0.0.0.0:8080`. CORS is fully open. OpenAPI spec
served at `/api-docs/openapi.json`; Swagger UI at `/swagger-ui`.

### Activities (`/api/v1/activities`)

| Method | Path | Description |
|---|---|---|
| `GET` | `/api/v1/activities` | List all activities. Optional query params `start_date` and `end_date` (both must be provided together; client-side string-compare filter applied after fetching all rows) |
| `GET` | `/api/v1/activities/{id}` | Fetch a single activity by UUID |
| `POST` | `/api/v1/activities` | Create a new activity. Body: `{ date, start_time, end_time?, accounting_category_id, task, comment? }`. Returns 201 with the created object. |
| `PUT` | `/api/v1/activities` | Update an existing activity. **ID is in the request body**, not the path. Returns 200. |
| `DELETE` | `/api/v1/activities/{id}` | Delete activity by UUID. Returns 204. |
| `POST` | `/api/v1/activities/upload-csv` | Import activities from a CSV file via `multipart/form-data` (field name `file`). Query params: `activities_year` (required), `replace_mode` (`none`/`all`/`import_date_range`). |
| `PUT` | `/api/v1/activities/upload-csv` | Same import, but CSV content is the raw request body with `Content-Type: text/csv`. |

### Accounting Categories (`/api/v1/accounting-categories`)

| Method | Path | Description |
|---|---|---|
| `GET` | `/api/v1/accounting-categories` | List all categories |
| `POST` | `/api/v1/accounting-categories` | Create a new category. Body: `{ name }`. Returns 201. Rejects duplicates with 500. |
| `PUT` | `/api/v1/accounting-categories` | Rename a category. **ID in request body**. Returns 200. |
| `DELETE` | `/api/v1/accounting-categories/{id}` | Delete category by UUID. Returns 204. Cascades to delete all associated activities (DB constraint). |

### Daily Report (`/api/v1/daily-report`)

| Method | Path | Description |
|---|---|---|
| `GET` | `/api/v1/daily-report?report_date=YYYY-MM-DD` | Returns all activities for the given date, each with its duration, plus a `total_duration`. Durations are ISO 8601 strings (e.g. `PT5400S`). |

### Weekly Report (`/api/v1/weekly-report`)

| Method | Path | Description |
|---|---|---|
| `GET` | `/api/v1/weekly-report?week_start_date=YYYY-MM-DD` | Returns all activities for the 7-day window starting on the given date, `total_duration`, `duration_per_category` (map of category-id → duration), and `daily_durations_per_category` (nested map date → category-id → duration). |

---

## 3. Frontend Pages / Views

UI built with React + MUI Joy. SPA with client-side routing via React Router. All API calls use
axios. Base URL configurable via `VITE_API_BASE_URL` env var (default `http://localhost:8080`).

### Navigation sidebar links

- Today's Activities
- Activity Log
- Daily Report
- Weekly Report
- Yearly Report *(button present but no route — stub/placeholder only)*
- Categories Configuration
- Import Activities

### Today's Activities (`/activities`)

- Inline form to create a new activity: date (defaults to today), start time, end time (optional),
  category dropdown, task text, comment text. Keyboard shortcut: Enter in the task field submits.
- Displays activities for today in a table sorted by start time (descending).
- Per-row Edit (navigates to edit page) and Delete buttons.
- Optimistic delete with record count display and a manual Refresh button.

### Activity Log (`/activities/log`)

- Date-range picker (defaults to current calendar month).
- Fetches and displays activities grouped by ISO week, sorted by date then start time within each
  week.
- Same per-row Edit / Delete actions as Today's Activities.

### Edit Activity (`/activities/edit/:id`)

- Fetches existing activity by ID on mount.
- Same form fields as the create form, pre-populated.
- Validates end time > start time.
- Saves via `PUT /api/v1/activities`, then navigates back to `/activities`.

### Daily Report (`/daily-report`)

- Date picker (defaults to today).
- Table columns: Start Time, End Time, Duration (HH:MM), Category (name resolved client-side),
  Activity (task).
- Footer row showing total duration, colour-coded:
  - Orange: < 8 hours
  - Green: 8–10 hours
  - Red: > 10 hours

### Weekly Report (`/weekly-report`)

- Week picker using the HTML `<input type="week">` control (defaults to current ISO week).
- Pivot/matrix table: rows = days of the week, columns = each accounting category that appears
  that week + a daily total. Footer row = per-category weekly totals + overall weekly total.
- Total duration colour-coded:
  - Orange: < 40 hours
  - Green: 40–50 hours
  - Red: > 50 hours

### Categories Configuration (`/categories`)

- Table listing all categories.
- Inline rename editing (click Edit icon → text input appears in the row; Enter saves, Escape
  cancels).
- Delete button (immediately deletes; cascades to associated activities server-side).
- Add-category form at the top.

### Import Activities (`/import`)

- Year selector (number input, default current year).
- File picker (`.csv` only).
- Uploads via `POST /api/v1/activities/upload-csv` with `multipart/form-data`.
- Displays success/error feedback.
- Includes a static documentation section describing the expected CSV column format (CW, Date,
  Check In, Check Out, Duration, PAM Category, Topic, Comment).
- Note: the import page always uses the default `replace_mode=none`; the `replace_mode` parameter
  is only accessible via the raw API.

---

## 4. Business Logic / Use Cases

### `ActivitiesList` (`use_cases/activities_list.rs`)

| Operation | Description |
|---|---|
| `record()` | Create and persist an activity with optional end time and comment |
| `activities()` | Retrieve all activities (unfiltered) |
| `get_by_id()` | Retrieve a single activity by ID (linear scan over `get_all`) |
| `update()` | Update all mutable fields of an existing activity |
| `delete()` | Delete a single activity by ID |
| `delete_by_date_range()` | Delete all activities within an inclusive date range; returns count deleted |
| `import()` | Bulk import from any `ActivitiesImporter`. Three replace modes: `None` (append), `All` (wipe all first), `ImportDateRange` (delete only records within the imported data's own date range before inserting) |

### `AccountingCategoriesList` (`use_cases/accounting_categories_list.rs`)

| Operation | Description |
|---|---|
| `create()` | Create a category; rejects duplicates by name check before insertion |
| `categories()` | List all categories |
| `update()` | Rename a category |
| `delete()` | Delete a category (cascades to activities via DB FK) |

### `DailyReport` (`use_cases/daily_report.rs`)

Aggregates activities for a single date and sums their durations into a `total_duration`.

### `WeeklyReport` (`use_cases/weekly_report.rs`)

Aggregates activities for a 7-day window. Computes:
- Total duration across all activities
- Duration per accounting category (week-level)
- Duration per accounting category per day of the week (for the pivot table)

---

## 5. Infrastructure

### Persistence — PostgreSQL

Managed by **dbmate** migrations. Two tables:

**`accounting_categories`**
- `id` UUID PK
- `name` VARCHAR(255) UNIQUE NOT NULL
- `created_at`, `updated_at` timestamps
- Index on `name`

**`activities`**
- `id` UUID PK
- `date` DATE NOT NULL
- `start_time` TIME NOT NULL
- `end_time` TIME (nullable)
- `category_id` UUID FK → `accounting_categories(id)` ON DELETE CASCADE
- `task` TEXT NOT NULL
- `comment` TEXT (nullable)
- `created_at`, `updated_at` timestamps
- Indexes on `date`, `category_id`, and `(date, category_id)`

9 default categories seeded by migration `20241016000003`: Development, Meetings, Documentation,
Testing, Code Review, Planning, Research, Support, Administration.

Connection string is hardcoded in `work-pulse-service/src/main.rs` as
`postgres://workpulse:supersecret@localhost:5432/workpulse`.

### Persistence — In-Memory

A complete parallel implementation of both repository traits backed by `Vec<_>`. Used when the
service is started with `--use-in-memory-repositories`. Starts empty (no seeded categories). Used
for all unit tests.

### Repository Abstraction

Both implementations satisfy the same `async_trait` traits defined in `adapters/mod.rs`:

- `AccountingCategoriesListRepository` — `get_all`, `get_by_id`, `add`, `update`, `delete`,
  `get_or_create_by_name`
- `ActivitiesListRepository` — `get_all`, `get_by_date`, `get_by_date_range`, `add`, `update`,
  `delete`, `delete_all`, `delete_by_date_range`

The service uses `Arc<Mutex<R>>` to share repositories across Axum handlers.

### CSV Importer (`infra/importers/csv_activities_importer.rs`)

Implements the `ActivitiesImporter` trait. Parses a CSV with columns: `CW`, `Date` (format
`DD.MM.`), `Check In` (HH:MM), `Check Out` (HH:MM), `PAM Category`, `Topic`, `Comment`. Year is
supplied separately (not embedded in the CSV). Auto-creates missing accounting categories via
`get_or_create_by_name`. Caches categories in-memory during a single import pass to minimise DB
round-trips. Converts `Comment = ""` to `None`.

### Bulk Insert Optimisation

The Postgres activities repository includes an `add_range` / `add_batch` path that uses
`sqlx::QueryBuilder` to bulk-insert in chunks of 100 rows, used during CSV import to avoid N
individual `INSERT` statements.

### Tech Stack

| Layer | Technology |
|---|---|
| Async runtime | Tokio |
| HTTP framework | Axum |
| DB client | sqlx |
| OpenAPI | utoipa + utoipa-axum + utoipa-swagger-ui |
| CORS | tower-http (fully open: any origin/method/header) |
| Tracing | tracing + tracing-subscriber (level via `RUST_LOG`) |
| Frontend build | Vite |
| Frontend UI | React + MUI Joy |
| Frontend routing | React Router |
| Frontend HTTP | axios |
| Frontend tests | Jest + Babel + jsdom + Testing Library |
| CLI HTTP client | reqwest (blocking) |
| Migrations | dbmate (bidirectional up/down SQL files) |
| Container | Docker Compose (postgres:16 + two app images) |

---

## 6. CLI Features (`work-pulse-cli`)

The CLI binary is a pure HTTP client with no direct database access. It communicates with a
running `work-pulse-service` at `http://localhost:8080`.

### Commands

#### `csv-import --file <path>`

Full import pipeline:

1. Reads the CSV file with **Latin-1 (ISO-8859-1) encoding** (handles Windows-generated files
   with special characters; the service-side importer assumes UTF-8).
2. Prints all parsed records to stdout for preview.
3. Checks every unique PAM category in the CSV against categories in the service via
   `GET /api/v1/accounting-categories`. Any missing category is created via
   `POST /api/v1/accounting-categories`.
4. Applies the static **category name mapping table** (see below) to normalise names before
   creating/matching them.
5. Creates each activity via `POST /api/v1/activities`. Year is hardcoded to `2025` for date
   conversion.
6. Prints each created activity's ID, date, times, category, and task to stdout.

**Known limitation (tracked in `TODO.md`):** The `Comment` column is parsed and shown in the
preview but is **not** forwarded to `create_activity()` — comments are silently dropped during
CLI import.

#### `csv-export --file <path>`

Stub only — prints a single message and returns. Not implemented.

### Category Mapper (`category_mapper.rs`)

Static lookup table translating internal CSV category identifiers to the display names used by
the service:

| CSV name | Mapped name |
|---|---|
| `CurrentVersion` | `Current Version` |
| `NextVersion` | `Next Version` |
| `SWATrainer` | `SWA Trainer` |
| `Sonstiges` | `Other` |
| `TechnoCluster` | `TC: SW-Defined Innovation` |

If no mapping exists the original name is used as-is.

### Internal HTTP Client Modules

- `ActivityService` — wraps `POST /api/v1/activities`. Base URL configurable; default
  `http://localhost:8080/api/v1/activities`.
- `CategoryService` — wraps `GET` and `POST /api/v1/accounting-categories`. Same default base
  URL.

---

## 7. Known Gaps / Stubs

| Item | Status |
|---|---|
| **Yearly Report** | Navigation button in the UI has no route and no page component |
| **CSV Export (CLI)** | `csv-export` command is a stub — prints a message and exits |
| **CLI comment forwarding** | `csv_import.rs` parses `Comment` column but does not pass it to `create_activity()` (tracked in `TODO.md`) |
| **Health check endpoint** | `// TODO Implement health check service` comment in `work-pulse-service/src/main.rs` |
| **Activities list filter** | `GET /api/v1/activities` date filtering is done in-process after fetching all rows from the DB, not via a SQL `WHERE` predicate |
| **Error handling** | Several Postgres repository methods use `.unwrap()` directly instead of propagating `Result` (noted via inline TODO comments) |
