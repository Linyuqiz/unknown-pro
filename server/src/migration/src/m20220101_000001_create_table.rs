use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                sea_query::Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(User::Id)
                            .big_unsigned()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(User::Username).string().not_null())
                    .col(ColumnDef::new(User::Nickname).string().not_null())
                    .col(ColumnDef::new(User::Password).string().not_null())
                    .col(ColumnDef::new(User::Mobile).string().not_null())
                    .col(ColumnDef::new(User::Email).string().not_null())
                    .col(ColumnDef::new(User::Enable).tiny_integer().not_null())
                    .col(
                        ColumnDef::new(User::Deleted)
                            .tiny_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(User::CreatedTime)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(User::UpdatedTime)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
    Username,
    Nickname,
    Password,
    Mobile,
    Email,
    Enable,
    Deleted,
    CreatedTime,
    UpdatedTime,
}
