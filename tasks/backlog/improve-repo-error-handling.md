---
id: improve-repo-error-handling
status: backlog
priority: low
created: 2026-07-17
owner: Walter Stocker
---

## Description
`PsqlActivitiesListRepository` in `src/work-pulse-core/src/infra/repositories/postgres/activities_list.rs` lacks proper
error handling. Repository access methods should return `Result` types instead of panicking.

## Acceptance criteria
- [ ] Repository methods return `Result<T, ActivitiesListRepositoryError>`
- [ ] Errors from database operations propagate correctly

## Related
- `src/work-pulse-core/src/infra/repositories/postgres/activities_list.rs:15`
