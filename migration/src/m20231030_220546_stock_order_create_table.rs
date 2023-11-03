use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(StockOrderEntity::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(StockOrderEntity::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(StockOrderEntity::Symbol).string().not_null())
                    .col(ColumnDef::new(StockOrderEntity::BidPrice).float().not_null())
                    .col(ColumnDef::new(StockOrderEntity::BidSize).integer().not_null())
                    .col(ColumnDef::new(StockOrderEntity::OrderType).string().not_null())
                    .col(ColumnDef::new(StockOrderEntity::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(StockOrderEntity::DeletedAt).timestamp())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(StockOrderEntity::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum StockOrderEntity {
    Table,
    Id,
    Symbol,
    BidPrice,
    BidSize,
    OrderType,
    CreatedAt,
    DeletedAt
}
