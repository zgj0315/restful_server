use super::m20240516_102649_create_kl_role::KlRole;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(KlUser::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(KlUser::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(KlUser::Name).string().not_null())
                    .col(ColumnDef::new(KlUser::RoleId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("ur-user-role_id")
                            .from(KlUser::Table, KlUser::RoleId)
                            .to(KlRole::Table, KlRole::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(KlUser::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum KlUser {
    Table,
    Id,
    Name,
    RoleId,
}
