---
started: 2026-07-28
updated: 2026-07-28
status: completed            # in-progress | completed | blocked
task: update backend acceptance test skill
adr: n.a.
agent: GitHub Copilot
---

## Goal
Add missing frontmatter to the backend acceptance test skill and tighten the skill guidance.

## Context
- skills/backend-acceptance-tests/SKILL.md

## Outcome
The backend acceptance test skill now has valid YAML frontmatter and clearer guidance on when to use it, what not to change, and how to author feature-only acceptance test requests. Validation found no file errors.

## Log
### 2026-07-28
- Created session log for this task.
- Verified the repo paths referenced by the skill exist: `src/work-pulse-service/tests/services.rs` and `src/work-pulse-service/tests/features/health_check_service.feature`.
- Added YAML frontmatter to the skill and rewrote the body to include explicit use, non-use, and authoring procedure guidance.
