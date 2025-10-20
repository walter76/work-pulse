use async_trait::async_trait;
use sqlx::{PgPool, Row};
use uuid::Uuid;

use crate::{
    adapters::{AccountingCategoriesListRepository, AccountingCategoriesListRepositoryError},
    entities::accounting::{AccountingCategory, AccountingCategoryId},
};

#[derive(Clone)]
pub struct PsqlAccountingCategoriesListRepository {
    connection_pool: PgPool,
}

impl PsqlAccountingCategoriesListRepository {
    pub fn new(connection_pool: PgPool) -> Self {
        Self { connection_pool }
    }

    pub async fn with_database_url(database_url: &str) -> Self {
        let pool = PgPool::connect(database_url).await.unwrap();
        Self::new(pool)
    }
}

#[async_trait]
impl AccountingCategoriesListRepository for PsqlAccountingCategoriesListRepository {
    async fn get_all(&self) -> Vec<AccountingCategory> {
        let rows = sqlx::query("SELECT id, name FROM accounting_categories")
            .fetch_all(&self.connection_pool)
            .await
            .unwrap();

        rows.into_iter()
            .map(|row| {
                let id: Uuid = row.get("id");
                let name: String = row.get("name");
                AccountingCategory::with_id(AccountingCategoryId(id), name)
            })
            .collect()
    }

    async fn get_by_id(&self, id: AccountingCategoryId) -> Option<AccountingCategory> {
        let row = sqlx::query("SELECT id, name FROM accounting_categories WHERE id = $1")
            .bind(id.0)
            .fetch_optional(&self.connection_pool)
            .await
            .unwrap();

        row.map(|row| {
            let id: Uuid = row.get("id");
            let name: String = row.get("name");
            AccountingCategory::with_id(AccountingCategoryId(id), name)
        })
    }

    async fn add(&mut self, category: AccountingCategory) {
        sqlx::query("INSERT INTO accounting_categories (id, name) VALUES ($1, $2)")
            .bind(category.id().0)
            .bind(category.name())
            .execute(&self.connection_pool)
            .await
            .unwrap();
    }

    async fn update(
        &mut self,
        category: AccountingCategory,
    ) -> Result<(), AccountingCategoriesListRepositoryError> {
        sqlx::query("UPDATE accounting_categories SET name = $1 WHERE id = $2")
            .bind(category.name())
            .bind(category.id().0)
            .execute(&self.connection_pool)
            .await
            .map_err(|e| AccountingCategoriesListRepositoryError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn delete(
        &mut self,
        id: AccountingCategoryId,
    ) -> Result<(), AccountingCategoriesListRepositoryError> {
        sqlx::query("DELETE FROM accounting_categories WHERE id = $1")
            .bind(id.0)
            .execute(&self.connection_pool)
            .await
            .map_err(|e| AccountingCategoriesListRepositoryError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn get_or_create_by_name(
        &mut self,
        name: &str,
    ) -> Result<AccountingCategory, AccountingCategoriesListRepositoryError> {
        let row = sqlx::query("SELECT id, name FROM accounting_categories WHERE name = $1")
            .bind(name)
            .fetch_optional(&self.connection_pool)
            .await
            .map_err(|e| AccountingCategoriesListRepositoryError::DatabaseError(e.to_string()))?;

        if let Some(row) = row {
            let id: Uuid = row.get("id");
            let name: String = row.get("name");
            Ok(AccountingCategory::with_id(AccountingCategoryId(id), name))
        } else {
            let new_category = AccountingCategory::new(name.to_string());
            self.add(new_category.clone()).await;
            Ok(new_category)
        }
    }
}
