use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Stock::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Stock::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Stock::Symbol).string().not_null())
                    .col(ColumnDef::new(Stock::Exchange).string().not_null())
                    .col(ColumnDef::new(Stock::CompanyName).string().not_null())
                    .col(ColumnDef::new(Stock::StockType).string().not_null())
                    .col(ColumnDef::new(Stock::IsNasdaqListed).boolean().default(false))
                    .col(ColumnDef::new(Stock::IsNasdaq100).boolean().default(false))
                    .col(ColumnDef::new(Stock::IsHeld).boolean().default(false))
                    .col(ColumnDef::new(Stock::SecondaryData).string().not_null())
                    .col(ColumnDef::new(Stock::MarketStatus).string().not_null())
                    .col(ColumnDef::new(Stock::AssetClass).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Stock::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Stock {
    Table,
    Id,
    Symbol,
    Exchange,
    CompanyName,
    StockType,
    IsNasdaqListed,
    IsNasdaq100,
    IsHeld,
    SecondaryData,
    MarketStatus,
    AssetClass
}