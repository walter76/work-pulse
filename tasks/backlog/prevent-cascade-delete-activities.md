---
id: prevent-cascade-delete-activities
status: backlog
priority: medium
created: 2026-09-01
owner: Walter Stocker
---

## Description
Currently, the database constraint `ON DELETE CASCADE` on `activities.category_id` deletes all activities when a
category is deleted. This is a data loss risk. Instead, we should prevent category deletion if activities exist, or
allow category archiving/reassignment to preserve activity history.

## Acceptance criteria
- [ ] Evaluate approach: soft-delete categories or prevent deletion with activities
- [ ] Update database schema constraint from `ON DELETE CASCADE` to `ON DELETE RESTRICT`
- [ ] Implement API response handling for constraint violation (409 Conflict)
- [ ] Add category archival or reassignment endpoint if soft-delete approach chosen
- [ ] Document the category deletion behavior in CLAUDE.md

## Related
- Schema gotcha documented in [CLAUDE.md](../../CLAUDE.md#schema-gotchas)
