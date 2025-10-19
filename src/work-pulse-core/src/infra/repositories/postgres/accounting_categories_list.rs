use async_trait::async_trait;

use crate::{
    adapters::AccountingCategoriesListRepository,
    entities::accounting::{AccountingCategory, AccountingCategoryId},
};

#[derive(Clone)]
pub struct PsqlAccountingCategoriesListRepository {}

impl PsqlAccountingCategoriesListRepository {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl AccountingCategoriesListRepository for PsqlAccountingCategoriesListRepository {
    async fn get_all(&self) -> Vec<AccountingCategory> {
        unimplemented!()
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
