use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(KlRole::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(KlRole::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(KlRole::Name).string().not_null())
                    .col(
                        ColumnDef::new(KlRole::CreateTime)
                            .timestamp()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(KlRole::ModifyTime)
                            .timestamp()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(KlRole::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum KlRole {
    Table,
    Id,
    Name,
    CreateTime,
    ModifyTime,
}
