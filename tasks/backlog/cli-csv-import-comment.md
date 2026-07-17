---
id: cli-csv-import-comment
status: backlog
priority: low
created: 2026-07-17
owner: Walter Stocker
---

## Description
`work-pulse-cli` deserializes the `Comment` CSV column into `ActivityTableRecord.comment` and prints it during import
preview, but never passes it to `ActivityService::create_activity()`.

Per `dev-plans/map-activity-comment.md`, update the CLI `create_activity()` call to forward `record.comment` so comments
from CSV imports are persisted.

## Acceptance criteria
- [ ] `create_activity()` in `src/work-pulse-cli/src/csv_import.rs` forwards `record.comment` to
  `ActivityService::create_activity()`
- [ ] CSV import via CLI persists comments for imported activities

## Related
- `dev-plans/map-activity-comment.md`
