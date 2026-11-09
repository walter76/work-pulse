---
started: 2026-11-09
updated: 2026-11-09
status: completed          # in-progress | completed | blocked
task: rework-todays-activities
adr: n.a.
agent: GitHub Copilot
---

## Goal
Rework Today's Activities so users can view and manage activities for any selected date, with per-activity and daily duration information.

## Context
[Task](../../tasks/done/rework-todays-activities.md)
[Feature analysis](../../doc/features/feature-analysis.md)

## Outcome
Today's Activities now uses a local-today default date selector for one-day activity queries and quick entry. Rows show durations, the daily total follows the Daily Report thresholds, empty days show zero, and edit navigation preserves the selected date. Existing date-range API support was sufficient, so no backend change was needed.

Frontend coverage includes selected-date changes, empty days, duration totals/colors, create/edit/delete actions, out-of-order refresh responses, and date utility edge cases. All 47 frontend tests pass and lint exits successfully with two existing hook-dependency warnings in `activityLog.jsx` and `editActivity.jsx`. The production build succeeds; Vite reports its existing chunk-size advisory.

## Log
### 2026-11-09
- Initialized the session and moved the task from backlog to in-progress.
- Recorded the implementation plan; no feature code has been changed yet.
- Implemented the selected-date workflow, duration table/footer, edit return state, and latest-request-wins refresh behavior.
- Confirmed existing `start_date`/`end_date` filtering supports the one-day query; backend unchanged.
- Added seven page/editor workflow tests, a stale-response hook regression test, and duration/date utility tests; all 47 frontend tests pass.
- Full-project lint exits successfully with two existing hook-dependency warnings. Production build succeeds with the existing chunk-size advisory.
