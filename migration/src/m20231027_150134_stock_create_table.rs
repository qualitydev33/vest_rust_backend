use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(StockEntity::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(StockEntity::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(StockEntity::Symbol).string().not_null())
                    .col(ColumnDef::new(StockEntity::Exchange).string().not_null())
                    .col(ColumnDef::new(StockEntity::CompanyName).string().not_null())
                    .col(ColumnDef::new(StockEntity::StockType).string().not_null())
                    .col(ColumnDef::new(StockEntity::IsNasdaqListed).boolean().default(false))
                    .col(ColumnDef::new(StockEntity::IsNasdaq100).boolean().default(false))
                    .col(ColumnDef::new(StockEntity::IsHeld).boolean().default(false))
                    .col(ColumnDef::new(StockEntity::SecondaryData).string().not_null())
                    .col(ColumnDef::new(StockEntity::MarketStatus).string().not_null())
                    .col(ColumnDef::new(StockEntity::AssetClass).string().not_null())
                    .col(ColumnDef::new(StockEntity::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(StockEntity::DeletedAt).timestamp())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(StockEntity::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum StockEntity {
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
    AssetClass,
    CreatedAt,
    DeletedAt
}