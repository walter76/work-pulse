# WorkPulse - Domain Model Documentation

## Overview

WorkPulse is a time tracking and activity management system designed to track work activities and associate them with accounting categories. The domain model follows Domain-Driven Design (DDD) principles with clear separation between entities, value objects, and repository abstractions.

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
```sql
id          uuid PRIMARY KEY
date        date NOT NULL
start_time  time NOT NULL
end_time    time (nullable)
category_id uuid NOT NULL (FK to accounting_categories)
task        text NOT NULL
comment     text (nullable - not in domain model yet)
created_at  timestamp with time zone
updated_at  timestamp with time zone
```

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
```sql
id          uuid PRIMARY KEY
name        varchar(255) NOT NULL UNIQUE
created_at  timestamp with time zone
updated_at  timestamp with time zone
```

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

### 1. AccountingCategoriesListRepository

**Purpose**: Abstract interface for persisting and retrieving accounting categories.

**Operations**:
- `get_all()` → `Vec<AccountingCategory>` - Retrieve all categories
- `get_by_id(id)` → `Option<AccountingCategory>` - Retrieve by ID
- `add(category)` - Add a new category
- `update(category)` → `Result<(), Error>` - Update existing category
- `delete(id)` → `Result<(), Error>` - Delete a category
- `get_or_create_by_name(name)` → `Result<AccountingCategory, Error>` - Get or create by name

**Error Types**:
- `NotFound(AccountingCategoryId)` - Category not found
- `DatabaseError(String)` - Database-related issues

**Implementations**:
- `InMemoryAccountingCategoriesListRepository` - In-memory storage for testing
- `PostgresAccountingCategoriesListRepository` - PostgreSQL persistence

---

### 2. ActivitiesListRepository

**Purpose**: Abstract interface for persisting and retrieving activities.

**Operations**:
- `get_all()` → `Vec<Activity>` - Retrieve all activities
- `get_by_date(date)` → `Vec<Activity>` - Retrieve activities for a specific date
- `get_by_date_range(start, end)` → `Vec<Activity>` - Retrieve activities within date range
- `add(activity)` - Add a new activity
- `update(activity)` → `Result<(), Error>` - Update existing activity
- `delete(id)` → `Result<(), Error>` - Delete an activity
- `delete_all()` → `Result<(), Error>` - Delete all activities

**Error Types**:
- `NotFound(ActivityId)` - Activity not found
- `DatabaseError(String)` - Database-related issues

**Implementations**:
- `InMemoryActivitiesListRepository` - In-memory storage for testing
- `PostgresActivitiesListRepository` - PostgreSQL persistence

---

## Domain Services

### ActivitiesImporter

**Purpose**: Abstract interface for importing activities from external sources.

**Operations**:
- `import<R: Read>(reader, year)` → `Result<Vec<Activity>, Error>` - Import activities from a reader

**Error Types**:
- `ParseError` - Could not parse activities from source
- `RepositoryError(String)` - Error accessing the repository

**Implementations**:
- `CsvActivitiesImporter` - Import from CSV files

---

## Use Cases

The domain model supports the following use cases (application layer):

1. **ActivitiesList** - Manage the list of activities (CRUD operations)
2. **AccountingCategoriesList** - Manage accounting categories (CRUD operations)
3. **DailyReport** - Generate reports for a specific day
4. **WeeklyReport** - Generate reports for a week

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

The domain model follows Clean Architecture / Hexagonal Architecture:

```
┌─────────────────────────────────────────┐
│         Application Layer               │
│  (Use Cases: daily_report,              │
│   weekly_report, activities_list, etc.) │
└─────────────────────────────────────────┘
              ↓ uses
┌─────────────────────────────────────────┐
│         Domain Layer                    │
│  (Entities: Activity,                   │
│   AccountingCategory)                   │
│  (Value Objects: ActivityId,            │
│   AccountingCategoryId)                 │
└─────────────────────────────────────────┘
              ↑ implements
┌─────────────────────────────────────────┐
│         Adapter Layer                   │
│  (Repository Traits)                    │
└─────────────────────────────────────────┘
              ↑ implements
┌─────────────────────────────────────────┐
│         Infrastructure Layer            │
│  (Postgres/InMemory Implementations,    │
│   CSV Importer)                         │
└─────────────────────────────────────────┘
```

---

## Database Indices

The database schema includes the following indices for query optimization:

- `idx_accounting_categories_name` - B-tree index on accounting_categories.name
- `idx_activities_category_id` - B-tree index on activities.category_id
- `idx_activities_date` - B-tree index on activities.date
- `idx_activities_date_category` - Composite B-tree index on (date, category_id)

---

## Design Patterns Applied

1. **Repository Pattern** - Abstracts data persistence
2. **Value Object Pattern** - ActivityId, AccountingCategoryId
3. **Entity Pattern** - Activity, AccountingCategory with identity
4. **Aggregate Pattern** - Activity and AccountingCategory as aggregate roots
5. **Factory Pattern** - `new()` and `with_id()` constructors
6. **Adapter Pattern** - Repository traits serve as adapters
7. **Dependency Inversion** - Use cases depend on repository abstractions, not implementations

---

## Type Safety Features

The domain model leverages Rust's type system for domain correctness:

1. **NewType Pattern** - ActivityId and AccountingCategoryId wrap UUID
2. **Result Types** - Explicit error handling with `Result<T, E>`
3. **Option Types** - Explicit nullability for end_time
4. **Trait Bounds** - `Send + Sync` for async safety
5. **Custom Error Types** - `thiserror` for domain-specific errors

---

## Missing Domain Concepts

Based on the database schema, the following fields exist in persistence but are not yet modeled in the domain layer:

1. **Activity.comment** (text, nullable) - Additional comments for activities
2. **Timestamps** (created_at, updated_at) - Audit trail information

These could be added to enrich the domain model.

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

The WorkPulse domain model is a clean, well-structured implementation following DDD principles. It has:

- **2 main entities**: Activity and AccountingCategory
- **2 value objects**: ActivityId and AccountingCategoryId
- **2 repository abstractions**: For persistence independence
- **1 domain service**: ActivitiesImporter for importing from external sources
- **Clear separation of concerns**: Domain logic is isolated from infrastructure

The model is type-safe, well-tested (based on the test code in entities), and follows Rust best practices with async/await support and proper error handling.
