use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(EventEntity::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(EventEntity::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(EventEntity::Topic).string().not_null())
                    .col(ColumnDef::new(EventEntity::Partition).integer().not_null())
                    .col(ColumnDef::new(EventEntity::Key).string().not_null())
                    .col(ColumnDef::new(EventEntity::Payload).string().not_null())
                    .col(ColumnDef::new(EventEntity::TraceId).string().not_null())
                    .col(ColumnDef::new(EventEntity::EventType).string().not_null())
                    .col(ColumnDef::new(EventEntity::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(EventEntity::DeletedAt).timestamp().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(EventEntity::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum EventEntity {
    Table,
    Id,
    Topic,
    Partition,
    Key,
    Payload,
    TraceId,
    EventType,
    CreatedAt,
    DeletedAt
}
