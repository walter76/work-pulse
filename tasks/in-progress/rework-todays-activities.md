---
id: rework-todays-activities
status: in-progress
priority: medium
created: 2026-11-09
owner: Walter Stocker
---

## Description
Rework Today's Activities to collect and edit activities for any selected date, with daily duration information and
quick entry for that date. Backend API additions or changes are allowed if analysis or implementation finds them useful.

## Acceptance criteria
- [ ] A date selector defaults to today and selects which day's activities are displayed and managed.
- [ ] Each activity displays its duration, and a total line displays the selected day's total duration.
- [ ] The daily total uses the Daily Report's duration highlighting: orange below 8 hours, green from 8 to 10 hours,
      and red above 10 hours.
- [ ] The quick-entry fields add an activity for the currently selected date, and the list and total reflect the change.
- [ ] Activities for the selected date can be edited and deleted, with the selected date retained when returning from edit.
- [ ] Changing the selected date updates the activity list and total, including when there are no activities.

## Related
- [Feature analysis: Today's Activities and Daily Report](../../doc/features/feature-analysis.md)