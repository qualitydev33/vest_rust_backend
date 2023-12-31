pub use sea_orm_migration::prelude::*;

mod m20231027_150134_stock_create_table;
mod m20231030_220546_stock_order_create_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20231027_150134_stock_create_table::Migration),
            Box::new(m20231030_220546_stock_order_create_table::Migration),
        ]
    }
}
