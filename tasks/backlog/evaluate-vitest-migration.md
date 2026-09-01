---
id: evaluate-vitest-migration
status: backlog
priority: low
created: 2026-09-01
owner: Walter Stocker
---

## Description
Evaluate migrating from Jest to Vitest for the React frontend (work-pulse-app). Vitest is the recommended test runner
for Vite projects and would align the test setup with the build toolchain. Assess migration effort, compatibility with
current test suite, and benefits/drawbacks.

## Acceptance criteria
- [ ] Document current Jest setup (config, test files, dependencies)
- [ ] Research Vitest compatibility with Testing Library, MUI Joy, and existing test patterns
- [ ] Estimate migration effort and risk
- [ ] Assess performance improvements and DX benefits
- [ ] Document findings and recommendation in ADR or session log
- [ ] If approved, create implementation task

## Related
- Frontend test setup documented in [CLAUDE.md](../../CLAUDE.md#frontend)
