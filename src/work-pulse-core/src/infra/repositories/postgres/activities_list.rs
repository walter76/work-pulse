use async_trait::async_trait;
use chrono::{NaiveDate, NaiveTime};
use sqlx::Row;
use uuid::Uuid;

use crate::{
    adapters::{ActivitiesListRepository, ActivitiesListRepositoryError},
    entities::{
        accounting::AccountingCategoryId,
        activity::{Activity, ActivityId},
    },
    infra::repositories::postgres::PsqlConnection,
};

#[derive(Clone)]
pub struct PsqlActivitiesListRepository {
    psql_connection: PsqlConnection,
}

impl PsqlActivitiesListRepository {
    pub fn new(psql_connection: PsqlConnection) -> Self {
        Self { psql_connection }
    }
}

#[async_trait]
impl ActivitiesListRepository for PsqlActivitiesListRepository {
    async fn get_all(&self) -> Vec<Activity> {
        let rows =
            sqlx::query("SELECT id, date, start_time, end_time, category_id, task FROM activities")
                .fetch_all(self.psql_connection.pool())
                .await
                .unwrap();

        rows.into_iter()
            .map(|row| {
                let id: Uuid = row.get("id");
                let date: NaiveDate = row.get("date");
                let start_time: NaiveTime = row.get("start_time");
                let end_time: Option<NaiveTime> = row.get("end_time");
                let category_id: Uuid = row.get("category_id");
                let task: String = row.get("task");

                let mut activity = Activity::with_id(
                    ActivityId(id),
                    date,
                    start_time,
                    AccountingCategoryId(category_id),
                    task,
                );

                activity.set_end_time(end_time);

                activity
            })
            .collect()
    }

    async fn get_by_date(&self, date: NaiveDate) -> Vec<Activity> {
        let rows = sqlx::query(
                "SELECT id, date, start_time, end_time, category_id, task FROM activities WHERE date = $1",
            )
            .bind(date)
            .fetch_all(self.psql_connection.pool())
            .await
            .unwrap();

        rows.into_iter()
            .map(|row| {
                let id: Uuid = row.get("id");
                let date: NaiveDate = row.get("date");
                let start_time: NaiveTime = row.get("start_time");
                let end_time: Option<NaiveTime> = row.get("end_time");
                let category_id: Uuid = row.get("category_id");
                let task: String = row.get("task");

                let mut activity = Activity::with_id(
                    ActivityId(id),
                    date,
                    start_time,
                    AccountingCategoryId(category_id),
                    task,
                );

                activity.set_end_time(end_time);

                activity
            })
            .collect()
    }

    async fn get_by_date_range(&self, start: NaiveDate, end: NaiveDate) -> Vec<Activity> {
        let rows = sqlx::query(
                "SELECT id, date, start_time, end_time, category_id, task FROM activities WHERE date BETWEEN $1 AND $2",
            )
            .bind(start)
            .bind(end)
            .fetch_all(self.psql_connection.pool())
            .await
            .unwrap();

        rows.into_iter()
            .map(|row| {
                let id: Uuid = row.get("id");
                let date: NaiveDate = row.get("date");
                let start_time: NaiveTime = row.get("start_time");
                let end_time: Option<NaiveTime> = row.get("end_time");
                let category_id: Uuid = row.get("category_id");
                let task: String = row.get("task");

                let mut activity = Activity::with_id(
                    ActivityId(id),
                    date,
                    start_time,
                    AccountingCategoryId(category_id),
                    task,
                );

                activity.set_end_time(end_time);

                activity
            })
            .collect()
    }

    async fn add(&mut self, activity: Activity) {
        sqlx::query(
                "INSERT INTO activities (id, date, start_time, end_time, category_id, task) VALUES ($1, $2, $3, $4, $5, $6)",
            )
            .bind(activity.id().0)
            .bind(activity.date())
            .bind(activity.start_time())
            .bind(activity.end_time())
            .bind(activity.accounting_category_id().0)
            .bind(activity.task())
            .execute(self.psql_connection.pool())
            .await
            .unwrap();
    }

    async fn update(&mut self, activity: Activity) -> Result<(), ActivitiesListRepositoryError> {
        sqlx::query(
                "UPDATE activities SET date = $1, start_time = $2, end_time = $3, category_id = $4, task = $5 WHERE id = $6",
            )
            .bind(activity.date())
            .bind(activity.start_time())
            .bind(activity.end_time())
            .bind(activity.accounting_category_id().0)
            .bind(activity.task())
            .bind(activity.id().0)
            .execute(self.psql_connection.pool())
            .await
            .map_err(|e| ActivitiesListRepositoryError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn delete(&mut self, id: ActivityId) -> Result<(), ActivitiesListRepositoryError> {
        sqlx::query("DELETE FROM activities WHERE id = $1")
            .bind(id.0)
            .execute(self.psql_connection.pool())
            .await
            .map_err(|e| ActivitiesListRepositoryError::DatabaseError(e.to_string()))?;

        Ok(())
    }
}
