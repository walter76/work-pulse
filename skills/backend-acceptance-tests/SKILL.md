# Backend Acceptance Tests

Rust backend acceptance testing conventions for work-pulse-service.

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

## Authoring Rule

- When user asks to write a backend acceptance test, write only scenarios in a `.feature` file.
- Do not implement or modify Rust step definitions, world structs, hooks, runners, or other glue code unless explicitly requested.
