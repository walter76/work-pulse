# Work Pulse — Comprehensive Feature Analysis

This document focuses on product behavior and feature gaps. System boundaries, persistence, deployment, tests, and cross-cutting implementation constraints are maintained in the [architecture review](../architecture/work-pulse-architecture.md).

## 1. Core Domain Entities
See the [domain model](work-pulse-domain-model.md) for entity fields and behavior.

---

## 2. Backend REST API Endpoints
See the [architecture review](../architecture/work-pulse-architecture.md) for the API route table, request/response behavior, and health endpoint.

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

- Date selector defaults to the local current date and controls both the displayed activities and
  the date assigned to quick-entry activities. The form includes start time, optional end time,
  category, task, and comment. Enter in the task field submits.
- Displays the selected day's activities in descending start-time order, with per-activity durations
  and a daily total. The total is orange below 8 hours, green from 8 to 10 hours, and red above 10.
- Per-row Edit (returns to the same selected date) and Delete buttons, record count, and manual
  Refresh button. Empty dates show a zero total and an empty-state message.

### Activity Log (`/activities/log`)

- Date-range picker (defaults to current calendar month).
- Fetches and displays activities grouped by ISO week, sorted by date then start time within each
  week.
- Same per-row Edit / Delete actions as Today's Activities.

### Edit Activity (`/activities/edit/:id`)

- Fetches existing activity by ID on mount.
- Same form fields as the create form, pre-populated.
- Validates end time > start time.
- Saves via `PUT /api/v1/activities`, then navigates back to `/activities` while retaining the
  selected date when launched from Today's Activities.

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
Use-case responsibilities and repository interactions are described in [architecture](../architecture/work-pulse-architecture.md#41-work-pulse-core--domain-library).

---

## 5. Architecture Reference

The [architecture review](../architecture/work-pulse-architecture.md) is the source of truth for runtime configuration, API/service structure, persistence, repository implementations, deployment, and test/CI coverage. The [domain model](work-pulse-domain-model.md) documents entity attributes and domain rules.

---

## 6. CLI Features (`work-pulse-cli`)
CLI commands, import behavior, and current limitations are covered in [architecture](../architecture/work-pulse-architecture.md#43-work-pulse-cli--cli-tool).

---

## 7. Known Gaps / Stubs

| Item | Status |
|---|---|
| **Yearly Report** | Navigation button in the UI has no route and no page component |
| **CSV Export (CLI)** | `csv-export` command is a stub — prints a message and exits |
| **CLI comment forwarding** | `csv_import.rs` parses `Comment` column but does not pass it to `create_activity()` (tracked in `TODO.md`) |
| **CSV format guidance** | The import page lists a `Duration` column, but the service-side CSV record does not consume it |

Implementation-wide limitations, including in-memory date filtering, error propagation, and database health behavior, are documented in the [architecture review](../architecture/work-pulse-architecture.md#12-current-constraints).
