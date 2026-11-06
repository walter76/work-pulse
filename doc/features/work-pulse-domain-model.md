# WorkPulse - Domain Model Documentation

## Overview

WorkPulse is a time tracking and activity management system designed to track work activities and associate them with accounting categories. The domain model follows Domain-Driven Design (DDD) principles with clear separation between entities, value objects, and repository abstractions.

For system boundaries, runtime configuration, and persistence implementation, see the [architecture review](../architecture/work-pulse-architecture.md).

---

## Domain Entities

### 1. Activity

**Purpose**: Represents a work activity performed by a user during their working day.

**Attributes**:
- `id: ActivityId` - Unique identifier for the activity
- `date: NaiveDate` - The date when the activity was performed
- `start_time: NaiveTime` - When the activity started
- `end_time: Option<NaiveTime>` - When the activity ended (optional, None for ongoing activities)
- `accounting_category_id: AccountingCategoryId` - Reference to the accounting category
- `task: String` - Description of the task
- `comment: Option<String>` - Optional activity note

**Behaviors**:
- `new()` - Creates a new activity with a random ID
- `with_id()` - Creates an activity with a specific ID (for reconstitution from persistence)
- `duration()` - Calculates the duration of the activity (returns `Duration::zero()` if end_time is None)
- Getters and setters for all attributes

**Business Rules**:
- An activity can be ongoing (end_time is None)
- Duration calculation returns zero for ongoing activities
- Every activity must be associated with an accounting category
- Every activity must have a task description

**Database Mapping** (`activities` table):
The persistence schema is documented in the [architecture review](../architecture/work-pulse-architecture.md#64-database-schema-postgresql).

---

### 2. AccountingCategory

**Purpose**: Represents a category used for organizing and classifying activities for accounting purposes.

**Attributes**:
- `id: AccountingCategoryId` - Unique identifier for the category
- `name: String` - The name of the accounting category

**Behaviors**:
- `new()` - Creates a new category with a random ID
- `with_id()` - Creates a category with a specific ID (for reconstitution from persistence)
- Getters and setters for all attributes

**Business Rules**:
- Category names must be unique (enforced at database level)
- Every category must have a name

**Database Mapping** (`accounting_categories` table):
The persistence schema is documented in the [architecture review](../architecture/work-pulse-architecture.md#64-database-schema-postgresql).

---

## Value Objects

### 1. ActivityId

**Purpose**: Type-safe wrapper for Activity unique identifiers.

**Attributes**:
- `Uuid` - Underlying UUID value

**Behaviors**:
- `new()` - Generates a new random UUID
- `parse_str()` - Parses a string into an ActivityId
- `Display` trait implementation for string formatting

**Validation**:
- Throws `ActivityIdError::NotAValidId` if parsing fails

**Characteristics**:
- Immutable
- Implements `Debug`, `Clone`, `PartialEq`, `Eq`, `Hash`

---

### 2. AccountingCategoryId

**Purpose**: Type-safe wrapper for AccountingCategory unique identifiers.

**Attributes**:
- `Uuid` - Underlying UUID value

**Behaviors**:
- `new()` - Generates a new random UUID
- `parse_str()` - Parses a string into an AccountingCategoryId
- `Display` trait implementation for string formatting

**Validation**:
- Throws `AccountingCategoryIdError::NotAValidId` if parsing fails

**Characteristics**:
- Immutable
- Implements `Debug`, `Clone`, `PartialEq`, `Eq`, `Hash`

---

## Aggregates

### Activity Aggregate

**Aggregate Root**: Activity

**Invariants**:
- An activity must have a valid date
- An activity must have a start time
- An activity must be associated with an existing accounting category
- End time, if present, must be after start time (not enforced in current code)
- Task description must not be empty (not enforced in current code)

**Aggregate Boundary**:
- The Activity entity owns its lifecycle
- References AccountingCategory through AccountingCategoryId (weak reference)

---

### AccountingCategory Aggregate

**Aggregate Root**: AccountingCategory

**Invariants**:
- Category name must be unique across the system
- Category name must not be empty

**Aggregate Boundary**:
- The AccountingCategory entity is independent
- Multiple Activities can reference the same AccountingCategory

---

## Relationships

```
AccountingCategory 1 ----< * Activity
   (One category can be associated with many activities)

Relationship Type: One-to-Many
Cardinality: 1..* (One AccountingCategory to Zero or Many Activities)
Reference: Activity holds AccountingCategoryId (foreign key)
Cascade: ON DELETE CASCADE (deleting a category deletes all associated activities)
```

---

## Repository Interfaces
Repository trait operations, error types, and in-memory/PostgreSQL implementations are documented in the [architecture review](../architecture/work-pulse-architecture.md#412-adapters-trait-contracts).

---

## Domain Services
The importer contract and CSV implementation are described in the [architecture review](../architecture/work-pulse-architecture.md#416-infrastructure--csv-importer).

---

## Use Cases

Use-case responsibilities are documented in the [architecture review](../architecture/work-pulse-architecture.md#413-use-cases).

---

## Domain Events

*Currently not implemented in the codebase*

Potential domain events for future implementation:
- `ActivityCreated`
- `ActivityUpdated`
- `ActivityDeleted`
- `ActivityCompleted` (when end_time is set)
- `AccountingCategoryCreated`
- `AccountingCategoryUpdated`
- `AccountingCategoryDeleted`

---

## Architecture Layers

Layer boundaries and dependency direction are documented in the [architecture review](../architecture/work-pulse-architecture.md#71-clean-architecture-dependency-rule).

---

## Database Indices

The authoritative schema and indexes are documented in the [architecture review](../architecture/work-pulse-architecture.md#64-database-schema-postgresql).

---

## Design Patterns Applied

Architectural patterns are documented in the [architecture review](../architecture/work-pulse-architecture.md#7-architectural-patterns).

---

## Type Safety Features

The domain model leverages Rust's type system for domain correctness:

1. **NewType Pattern** - ActivityId and AccountingCategoryId wrap UUID
2. **Result Types** - Explicit error handling with `Result<T, E>`
3. **Option Types** - Explicit nullability for end_time
4. **Trait Bounds** - `Send + Sync` for async safety
5. **Custom Error Types** - `thiserror` for domain-specific errors

---

## Persistence-Only Metadata

`created_at` and `updated_at` are stored in the database but are not modeled as domain attributes. `Activity.comment` is part of the domain entity and is persisted and exposed by the API.

---

## Potential Improvements

1. **Domain Validation**
   - Validate that end_time > start_time
   - Validate that task is not empty
   - Validate that category name is not empty

2. **Domain Events**
   - Implement event sourcing or domain events for better auditability

3. **Rich Domain Behavior**
   - Add methods like `is_ongoing()`, `complete()`, `pause()`, `resume()`
   - Add calculated properties like `total_hours()`, `billable_hours()`

4. **Value Objects**
   - Consider making `Task` a value object with validation
   - Consider making `CategoryName` a value object

5. **Business Rules**
   - Activities should not overlap for the same day
   - Maximum activity duration limits
   - Required breaks between activities

6. **Aggregate Enhancements**
   - Consider making Activity and AccountingCategory part of a larger aggregate like `Timesheet` or `WorkDay`

---

## Summary

The domain consists of `Activity` and `AccountingCategory`, identified by `ActivityId` and `AccountingCategoryId`. Persistence and application boundaries are documented in the [architecture review](../architecture/work-pulse-architecture.md).
