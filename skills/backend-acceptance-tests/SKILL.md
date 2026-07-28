---
name: backend-acceptance-tests
description: 'Write or update backend acceptance tests for work-pulse-service using cucumber Gherkin feature files under src/work-pulse-service/tests/features/. Use when the user asks for backend acceptance scenarios, feature coverage, or edits to existing .feature files. Do not use to modify Rust step definitions, world state, or test runners unless explicitly requested.'
user-invocable: false
---

# Backend Acceptance Tests

Rust backend acceptance testing conventions for work-pulse-service.

## When to Use

- User asks to add or change backend acceptance scenarios
- The task should stay in Gherkin `.feature` files under `src/work-pulse-service/tests/features/`
- Existing Rust step definitions are expected to be reused

## Do Not Use

- Implementing or changing Rust step definitions
- Modifying world structs, hooks, test runners, or other glue code
- Converting an acceptance-test request into lower-level unit or integration test work unless the user asks for that explicitly

## Scope

Applies only to backend Rust acceptance tests in `src/work-pulse-service/tests/`.

## Framework

`cucumber` crate with async step definitions in `src/work-pulse-service/tests/services.rs`.

## Feature Files

Gherkin `.feature` files under `src/work-pulse-service/tests/features/`.

## Execution Model

`ServiceWorld::run("tests/features/<name>.feature")` in the Rust test entrypoint.

## Step Pattern

`#[given]`, `#[when]`, `#[then]` mutate/read a `World` state object.

## HTTP Verification Style

Build Axum router in-world, send request with `oneshot`, assert status/body fields.

## Procedure

1. Identify the target `.feature` file in `src/work-pulse-service/tests/features/`.
2. Add or update scenarios only in that `.feature` file.
3. Reuse existing step phrasing and vocabulary where possible so the current Rust glue remains valid.
4. If the requested behavior cannot be expressed with the existing step vocabulary, stop and state that new Rust glue is required.

## Authoring Rule

- When user asks to write a backend acceptance test, write only scenarios in a `.feature` file.
- Do not implement or modify Rust step definitions, world structs, hooks, runners, or other glue code unless explicitly requested.
