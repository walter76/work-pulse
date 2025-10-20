use async_trait::async_trait;
use sqlx::{PgPool, Row};
use uuid::Uuid;

use crate::{
    adapters::AccountingCategoriesListRepository,
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
