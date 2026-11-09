---
started: 2026-11-09
updated: 2026-11-09
status: in-progress          # in-progress | completed | blocked
task: rework-todays-activities
adr: n.a.
agent: GitHub Copilot
---

## Goal
Rework Today's Activities so users can view and manage activities for any selected date, with per-activity and daily duration information.

## Context
[Task](../tasks/in-progress/rework-todays-activities.md)
[Feature analysis](../doc/features/feature-analysis.md)

## Current state
### open questions / risks
- Determine whether the existing activities API supports an efficient single-day query; add backend support only if needed.
- Preserve the selected date when navigating to and returning from activity editing.
### next steps
1. Inspect the `/activities` page, activity data hooks/API, date utilities, Daily Report duration formatting/highlighting, and edit navigation.
2. Decide the smallest API change needed for date-filtered activities, then implement it only if the existing API cannot support the workflow.
3. Update Today's Activities with a date selector defaulting to today, selected-date quick entry, per-row durations, a color-coded daily total, and correct empty-state behavior.
4. Ensure create, edit, and delete keep the selected date and refresh the list and total consistently.
5. Add focused frontend coverage for date changes, empty dates, duration totals/colors, and create/edit/delete flows; run frontend checks and backend checks if the API changes.

## Log
### 2026-11-09
- Initialized the session and moved the task from backlog to in-progress.
- Recorded the implementation plan; no feature code has been changed yet.
