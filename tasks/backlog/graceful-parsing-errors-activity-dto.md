---
id: graceful-parsing-errors-activity-dto
status: backlog
priority: medium
created: 2026-07-17
owner: Walter Stocker
---

## Description
`to_entity()` in `src/work-pulse-service/src/services/activities_list_service.rs` uses `.expect()` for parsing ID, date,
start time, and accounting category ID. These panics should be replaced with graceful error handling that returns
meaningful errors to the API caller.

## Acceptance criteria
- [ ] Replace all `.expect()` calls with proper error propagation
- [ ] Return descriptive error responses for invalid activity ID, date, start time, and accounting category ID
- [ ] Method signature updated to return `Result<Activity, Error>`

## Related
- `src/work-pulse-service/src/services/activities_list_service.rs:96`
