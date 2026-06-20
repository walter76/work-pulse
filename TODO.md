# TODO

## CLI: pass `comment` through CSV import

The `work-pulse-cli` crate (`src/work-pulse-cli/src/csv_import.rs`) already deserialises the
`Comment` CSV column into `ActivityTableRecord.comment` and prints it during the import preview,
but never passes it to `ActivityService::create_activity()`.

Once the `comment` field is mapped end-to-end per `dev-plans/map-activity-comment.md`, update
the CLI `create_activity()` call to forward `record.comment` so that comments from CSV imports
are persisted when using the CLI.
