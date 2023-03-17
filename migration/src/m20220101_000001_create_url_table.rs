use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220101_000001_create_url_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        todo!();

        manager
            .create_table(
                Table::create()
                    .table(Url::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Url::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Post::Body).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Url::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Url {
    Table,
    Id,
    Body,
}
