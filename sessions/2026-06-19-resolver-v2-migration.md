# Cargo Resolver "1" → "2" Migration Plan

## Background: What Changes

Resolver "1" (legacy) unifies features across the entire workspace: if crate A requests `foo`
with `features = ["x"]` and crate B requests `foo` with `features = ["y"]`, then `foo` is
compiled once with `["x", "y"]`. Resolver "2" compiles each crate against only the features
_it_ requests, plus features that flow in from its direct dependents. For edition-2024 crates,
resolver "2" is the intended default and is what Cargo's own documentation recommends.

---

## Feature Unification Analysis

### Findings: No Problematic Unification Detected

After a complete cross-examination of all three member crates, here is the full picture:

| Dependency | `work-pulse-core` | `work-pulse-service` | `work-pulse-cli` | Unification risk? |
|---|---|---|---|---|
| `chrono` | defaults | defaults | defaults | None — identical |
| `serde` | `["derive"]` | `["derive"]` | `["derive"]` | None — identical |
| `tokio` | `["full"]` | `["full"]` | not used | None — both use `full` |
| `tracing` | defaults | defaults | not used | None — identical |
| `clap` | not used | `["derive"]` v4.5.50 | `["derive"]` v4.5.40 | None for features; version delta noted below |
| `csv` | defaults | not used | defaults | None — identical |
| `sqlx` | `["runtime-tokio-rustls","postgres","uuid","chrono"]` | not used | not used | None — only one consumer |

No crate uses `default-features = false`. No crate has a `[features]` section. No optional
dependencies exist. There is no case where one crate requests a dependency with a strict subset
of features that another crate expands — the only differences are that some crates simply do not
use a dependency at all.

**The resolver "2" change will not alter feature selection for any dependency in this workspace.**
The feature sets are already orthogonal or identical.

---

### One Observation: `sqlx` Pulls MySQL + SQLite Even Though Only PostgreSQL Is Used

`work-pulse-core/Cargo.toml` requests:

```toml
sqlx = { version = "0.8.6", features = ["runtime-tokio-rustls", "postgres", "uuid", "chrono"] }
```

But `Cargo.lock` shows `sqlx` was resolved with `sqlx-mysql` and `sqlx-sqlite` as dependencies.
This is because sqlx's own `Cargo.toml` makes those backends unconditional when the `macros`
feature is active (implied by other feature combinations). This is a sqlx-internal issue, **not**
a workspace resolver artifact — it will behave identically under resolver "2". Not a blocker,
but it is excess compile weight worth knowing about.

---

## Migration Checklist

### Step 1 — Change `resolver` in `src/Cargo.toml` (REQUIRED)

**File:** `src/Cargo.toml`
**Change:** `resolver = "1"` → `resolver = "2"`

```toml
[workspace]
resolver = "2"
members = [
    "work-pulse-cli",
    "work-pulse-core",
    "work-pulse-service"
]
```

**Why:** This is the primary goal. Resolver "2" is the default for edition-2021+ crates and is
the correct setting for this workspace. Keeping "1" is technically incorrect for edition-2024 and
may produce unexpected behavior in the future as dependencies add features that rely on resolver
"2" semantics.

---

### Step 2 — Change `resolver` in `src/Cargo.docker.toml` (REQUIRED)

**File:** `src/Cargo.docker.toml`
**Change:** `resolver = "1"` → `resolver = "2"`

```toml
[workspace]
resolver = "2"
members = [
    "work-pulse-core",
    "work-pulse-service"
]
```

**Why:** This file is a parallel workspace manifest used for Docker builds. It must stay in sync
with `Cargo.toml` — inconsistency between the two would mean the Docker image resolves features
differently from local development builds, a subtle and hard-to-diagnose divergence.

---

### Step 3 — Regenerate `Cargo.lock` and verify the build (REQUIRED)

After making the two changes above, run from `src/`:

```cmd
cargo build --workspace
cargo test --workspace
```

**Why:** Even though the feature analysis shows no conflicts, Cargo may re-resolve some dependency
versions after the resolver change (particularly around `dev-dependencies` and
`build-dependencies`, which resolver "2" can now de-unify). The lock file will be regenerated.
You must confirm the build and all tests still pass.

**Expected outcome:** Clean build and passing tests, since no feature divergence was found. If any
failure occurs, the resolver change exposed a latent issue.

---

### Step 4 — Pin `clap` to a consistent version across crates (RECOMMENDED)

**Current state:**
- `work-pulse-service/Cargo.toml`: `clap = { version = "4.5.50", features = ["derive"] }`
- `work-pulse-cli/Cargo.toml`: `clap = { version = "4.5.40", features = ["derive"] }`

The lock file resolves to a single `clap 4.5.50` (because both semver ranges are satisfied by
4.5.50). This is fine under both resolvers since the features are identical and the version
constraint is compatible.

However, the version divergence is a maintenance hazard: a future `clap` release could cause
them to resolve to different minor versions and suddenly compile `clap` twice. Align both to the
same version string (e.g., `"4.5"` or a specific patch).

**Why recommended (not required):** No actual breakage today, but resolver "2" makes per-crate
compilation more independent, so the risk of dual compilation increases if versions diverge
further.

---

### Step 5 — Align `tokio` version constraints (RECOMMENDED)

**Current state:**
- `work-pulse-core/Cargo.toml`: `tokio = { version = "1.48.0", features = ["full"] }`
- `work-pulse-service/Cargo.toml`: `tokio = { version = "1.45.1", features = ["full"] }`

Both specify `features = ["full"]`, so there is no feature divergence. The lockfile resolves to
a single `tokio 1.48.0`. However, the minimum version pins differ by 3 minor releases with no
apparent reason. Align them to the same minimum version (e.g., both `"1.48.0"`) to make the
version policy explicit.

**Why recommended (not required):** No current breakage. Differing minimums with resolver "2"
are harmless only as long as the features stay the same.

---

## What Does NOT Need to Change

- **No member crate `Cargo.toml` edits are required.** No `default-features = false` overrides,
  no optional dependency configurations, no `[features]` sections — nothing in the member crates
  creates a feature unification hazard.
- **No `Cargo.lock` manual editing.** Just run `cargo build` — Cargo regenerates it automatically.
- **No edition changes.** All crates are already `edition = "2024"`. Resolver "2" is fully
  compatible.

---

## Risk Summary

| Risk | Severity | Verdict |
|---|---|---|
| Feature set changes for any dependency | High | **None found** — all shared deps have identical feature requests across consumers |
| `default-features = false` mismatches | High | **None found** — no crate uses `default-features = false` |
| Optional dependencies silently activated | Medium | **None found** — no optional deps defined in any member crate |
| `sqlx` compiling unused backends | Low | Pre-existing sqlx internal behavior, not resolver-related |
| `clap` version divergence becoming a dual-compile | Low | Harmless today; align versions as hygiene |

This is a low-risk migration. The two manifest changes in Steps 1 and 2 are the only required edits.
