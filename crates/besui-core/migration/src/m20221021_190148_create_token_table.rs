use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Token::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Token::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(Token::Symbol).string().not_null())
                    .col(ColumnDef::new(Token::Name).string().not_null())
                    .col(ColumnDef::new(Token::UpdatedAt).date_time().not_null())
                    .col(ColumnDef::new(Token::Exchanges).string())
                    .col(ColumnDef::new(Token::Image).string().null())
                    .col(
                        ColumnDef::new(Token::CirculatingSupply)
                            .big_integer()
                            .default(0),
                    )
                    .col(ColumnDef::new(Token::TotalSupply).big_integer().default(0))
                    .col(ColumnDef::new(Token::MaxSupply).big_integer().default(0))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Token::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Token {
    Table,
    Id,
    Symbol,
    Name,
    UpdatedAt,
    Exchanges,
    Image,
    CirculatingSupply,
    TotalSupply,
    MaxSupply,
}
