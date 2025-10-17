use sqlx::{PgPool, Row};

use crate::{
    adapters::{AccountingCategoriesListRepository, AccountingCategoriesListRepositoryError},
    entities::accounting::{AccountingCategory, AccountingCategoryId},
};

#[derive(Debug, Clone)]
pub struct PsqlAccountingCategoriesListRepository {
    connection_pool: PgPool,
}

impl PsqlAccountingCategoriesListRepository {
    pub fn new(connection_pool: PgPool) -> Self {
        Self { connection_pool }
    }

    async fn get_all_async(
        &self,
    ) -> Result<Vec<AccountingCategory>, AccountingCategoriesListRepositoryError> {
        let rows = sqlx::query("SELECT id, name FROM accounting_categories")
            .fetch_all(&self.connection_pool)
            .await
            .map_err(|e| AccountingCategoriesListRepositoryError::DatabaseError(e.to_string()))?;

        let categories = rows
            .into_iter()
            .map(|row| {
                let id = AccountingCategoryId::parse_str(&row.get::<String, _>("id"))
                    .expect("Invalid UUID format in database");

                AccountingCategory::with_id(id, row.get::<String, _>("name"))
            })
            .collect();

        Ok(categories)
    }
}

impl AccountingCategoriesListRepository for PsqlAccountingCategoriesListRepository {
    fn get_all(&self) -> Vec<AccountingCategory> {
        let rt = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");

        rt.block_on(self.get_all_async()).unwrap_or_else(|_| vec![])
    }

    fn get_by_id(&self, id: AccountingCategoryId) -> Option<AccountingCategory> {
        unimplemented!()
    }

    fn add(&mut self, category: crate::entities::accounting::AccountingCategory) {
        unimplemented!()
    }

    fn update(
        &mut self,
        category: crate::entities::accounting::AccountingCategory,
    ) -> Result<(), crate::adapters::AccountingCategoriesListRepositoryError> {
        unimplemented!()
    }

    fn delete(
        &mut self,
        id: crate::entities::accounting::AccountingCategoryId,
    ) -> Result<(), crate::adapters::AccountingCategoriesListRepositoryError> {
        unimplemented!()
    }

    fn get_or_create_by_name(
        &mut self,
        name: &str,
    ) -> Result<
        crate::entities::accounting::AccountingCategory,
        crate::adapters::AccountingCategoriesListRepositoryError,
    > {
        unimplemented!()
    }
}
