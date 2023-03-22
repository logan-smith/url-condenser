use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

// impl MigrationName for Migration {
//     fn name(&self) -> &str {
//         "m20220101_000001_create_url_table"
//     }
// }

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Urls::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Urls::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Urls::Url).string().not_null())
                    .col(ColumnDef::new(Urls::ShortUrlCode).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Urls::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Urls {
    Table,
    Id,
    Url,
    ShortUrlCode,
}
