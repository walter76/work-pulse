---
id: csv-encoding-detection
status: backlog
priority: low
created: 2026-07-17
owner: Walter Stocker
---

## Description
CSV import in `src/work-pulse-cli/src/csv_import.rs` may fail on Linux when CSV files use non-UTF-8 encodings. Add
encoding detection (e.g. via `encoding_rs` or `chardetng`) to handle Windows-1252, UTF-8 with BOM, and similar encodings.

## Acceptance criteria
- [ ] Detect CSV file encoding before parsing
- [ ] Handle common encodings: UTF-8, UTF-8 with BOM, Windows-1252, ISO-8859-1
- [ ] Graceful fallback or clear error for unsupported encodings

## Related
- `src/work-pulse-cli/src/csv_import.rs:115`
