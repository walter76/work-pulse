---
started: 2026-06-20
updated: 2026-06-20
status: completed
task: Map `activities.comment` End-to-End
adr: n.a.
agent: claude-code
---

## Goal
Add `comment: Option<String>` to the `Activity` entity and wire it through every layer so it is stored, retrieved,
exposed in the API, and editable in the UI.

## Context
- Migration `20241016000002` — introduced `comment TEXT` column (nullable, no constraints)
- `TODO.md` — CLI noted as out of scope

## Outcome
`comment` field mapped end-to-end across all layers. The `activities.comment` column (previously invisible to the
application stack) is now fully wired through the entity, repositories, CSV importer, use case, API DTO, and frontend
(create form, edit form, table). Backwards-compatible API change — `comment` is nullable JSON; existing clients that
omit it continue to work.

**Risks addressed:**
- UPDATE now correctly overwrites `comment` (including NULL) instead of silently preserving it
- CSV empty strings treated as `None` to avoid storing blank values
- `record()` signature change (internal breaking) coordinated with all call sites

**Left over:** CLI support for `comment` is out of scope (tracked in `TODO.md`).

## Current state

## Log
### 2026-06-20
- Analyzed existing state: `comment` column exists in DB since migration `20241016000002` but unmapped in all
  application layers
- Updated entity (`activity.rs`): added `comment: Option<String>` field, `comment()` getter, `set_comment()` setter;
  constructors unchanged (init `comment: None`)
- Updated in-memory repository (`in_memory/activities_list.rs`): added `comment` to `ActivityRecord`, updated
  `from_entity()`/`to_entity()` conversions
- Updated Postgres repository (`postgres/activities_list.rs`): added `comment` to all three SELECT queries, INSERT, bulk
  INSERT, and UPDATE statements; parameter binding adjusted (`$6` for comment, `$7` for id)
- Updated CSV importer (`csv_activities_importer.rs`): maps comment after construction, treats empty strings as `None`
- Updated use case (`activities_list.rs`): `record()` gains `comment: Option<String>` parameter; all test call sites
  updated to pass `None`
- Updated API DTO (`activities_list_service.rs`): added `comment` to DTO struct, updated `from_entity()`/`to_entity()`,
  updated `create_activity` handler and unit tests
- Frontend hook (`useActivities.jsx`): no changes needed (forwards entire payload as-is)
- Frontend create form (`todaysActivities.jsx`): added comment state, optional Input field, included in `createActivity`
  call
- Frontend edit form (`editActivity.jsx`): added comment state, populated from `fetchActivity()`, optional Input field,
  included in `updateActivity` call
- Frontend table (`activitiesTable.jsx`): added Comment column header and data cells
