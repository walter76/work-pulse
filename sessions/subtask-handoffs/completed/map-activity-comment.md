# Map `activities.comment` End-to-End

## Background

The `activities` table has a `comment TEXT` column (nullable, no constraints) introduced in
migration `20241016000002`. It has never been mapped in the Rust domain layer or exposed through
the API or UI. The column exists in the DB but is invisible to the entire application stack.

See investigation findings for full details on the current state of each layer.

## Goal

Add `comment: Option<String>` to the `Activity` entity and wire it through every layer so it is
stored, retrieved, exposed in the API, and editable in the UI.

## Steps

### 1. Entity — `src/work-pulse-core/src/entities/activity.rs`

- Add `comment: Option<String>` field to the `Activity` struct.
- Add `comment()` getter returning `Option<&str>`.
- Add `set_comment()` setter accepting `Option<String>`.
- `Activity::new()` and `Activity::with_id()` do **not** gain a `comment` parameter — both
  initialise `comment: None`. Comment is set after construction via `set_comment()`, consistent
  with how `end_time` is handled.
- Update unit tests: existing test signatures are unchanged (constructors are unchanged); add
  targeted tests for `set_comment` / `comment` getter.

### 2. In-memory repository — `src/work-pulse-core/src/infra/repositories/in_memory/activities_list.rs`

- Add `comment: Option<String>` to `ActivityRecord`.
- Update `from_entity()` to copy `activity.comment().map(str::to_owned)`.
- Update `to_entity()` to call `activity.set_comment(self.comment.clone())` after constructing
  the entity.

### 3. Postgres repository — `src/work-pulse-core/src/infra/repositories/postgres/activities_list.rs`

- **All three `SELECT` queries** (`get_all`, `get_by_date`, `get_by_date_range`): add `comment`
  to the column list; call `row.get::<_, Option<String>>("comment")`; call
  `activity.set_comment(comment)`.
- **Single `add()`**: add `comment` to the `INSERT` column list and `.bind(activity.comment())`.
- **Bulk `add_batch()`** (`add_range` helper): add `comment` to the `QueryBuilder` column list
  and `.push_bind(activity.comment())`.
- **`update()`**: add `comment = $N` to the `SET` clause (becomes `$6`, shifting `id` to `$7`)
  and bind `activity.comment()`.
- Delete operations need no changes.

### 4. CSV importer — `src/work-pulse-core/src/infra/importers/csv_activities_importer.rs`

- After constructing the activity, call
  `activity.set_comment(Some(activity_record.comment).filter(|s| !s.is_empty()))` — treating an
  empty string from the CSV as `None`, since the column is often blank.
- Update the importer test: assert `activities[0].comment()` equals the expected value from the
  test CSV fixture.

### 5. Use case — `src/work-pulse-core/src/use_cases/activities_list.rs`

- `record()` gains a `comment: Option<String>` parameter. After creating the activity it calls
  `activity.set_comment(comment)`.
- No other use-case methods need changes (`update()` and `import()` pass through the full
  `Activity` entity unchanged).
- Update all `record(...)` call sites in the test module to pass the new argument (all existing
  calls pass `None`).

### 6. API DTO — `src/work-pulse-service/src/services/activities_list_service.rs`

- Add `comment: Option<String>` to the `Activity` DTO struct (with `#[schema(example = "...")]`).
- Update `from_entity()` to set `comment: entity.comment().map(str::to_owned)`.
- Update `to_entity()` to call `activity.set_comment(self.comment.clone())`.
- Update `create_activity` handler: pass `new_activity.comment.clone()` into
  `activities_list.record(...)`.
- Update the two unit tests (`activity_from_entity_should_convert_correctly`,
  `activity_to_entity_should_convert_correctly`) to include `comment`.

### 7. Frontend hook — `src/work-pulse-app/src/hooks/useActivities.jsx`

No changes needed. `createActivity` and `updateActivity` forward the entire payload object as-is
to the API. Adding `comment` to the payload objects in the page components is sufficient.

### 8. Frontend create form — `src/work-pulse-app/src/pages/todaysActivities.jsx`

- Add `const [comment, setComment] = useState('')`.
- Add an `<Input>` for comment (optional field, no `required`).
- Include `comment: comment || null` in the `createActivity({...})` call.
- Reset `comment` to `''` after successful creation.

### 9. Frontend edit form — `src/work-pulse-app/src/pages/editActivity.jsx`

- Add `const [comment, setComment] = useState('')`.
- In `fetchActivity()`, set `setComment(activityData.comment ?? '')`.
- Add an `<Input>` for comment (optional, no `required`).
- Include `comment: comment || null` in the `updateActivity(activityId, {...})` call.

### 10. Frontend table — `src/work-pulse-app/src/components/activitiesTable.jsx`

- Add `<th>Comment</th>` to the header row.
- Add `<td>{activity.comment}</td>` to each data row.

## Risks and Notes

- **No migration needed.** The `comment` column already exists and is nullable.
- **Backwards-compatible API change.** `comment` is `Option` / nullable JSON; existing clients
  that omit it continue to work (serde treats missing JSON fields as `None` for `Option` types).
- **UPDATE behaviour change.** Today `UPDATE` silently preserves any existing `comment` value
  because the column is absent from the `SET` clause. After step 3, `UPDATE` correctly overwrites
  it — including setting it to `NULL` when the client sends `null`. Any rows with non-NULL
  `comment` from an external source will become visible for the first time.
- **CSV empty-string handling** (step 4): the CSV `Comment` header is required by serde for
  deserialization to succeed, but the value is often blank. Treating `""` as `None` avoids
  storing empty strings.
- **`record()` signature change** (step 5) is an internal breaking change. The only callers are
  in `activities_list_service.rs` (step 6) and the test module within `activities_list.rs` — both
  are updated in their respective steps.
- **CLI is out of scope for this plan.** See `TODO.md`.
