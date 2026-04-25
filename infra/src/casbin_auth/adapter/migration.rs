pub use entity::Entity as CasbinRule;
use sea_orm::{
    ConnectionTrait, DbErr, EntityName, ExecResult,
    sea_query::{ColumnDef, Index, Table},
};

use crate::casbin_auth::adapter::entity;

pub async fn up<C: ConnectionTrait>(conn: &C) -> Result<ExecResult, DbErr> {
    let create_table = Table::create()
        .if_not_exists()
        .table(CasbinRule.table_ref())
        .col(
            ColumnDef::new(entity::Column::Id)
                .big_integer()
                .not_null()
                .auto_increment()
                .primary_key(),
        )
        // MySQL max key length is `3072` bytes, in `utf8mb4` charset, it's `3072 / 4 = 768`
        // characters 18 + 125 * 6 = 768
        .col(
            ColumnDef::new(entity::Column::Ptype)
                .string_len(18)
                .not_null(),
        )
        .col(
            ColumnDef::new(entity::Column::V0)
                .string_len(125)
                .not_null(),
        )
        .col(
            ColumnDef::new(entity::Column::V1)
                .string_len(125)
                .not_null(),
        )
        .col(
            ColumnDef::new(entity::Column::V2)
                .string_len(125)
                .not_null(),
        )
        .col(
            ColumnDef::new(entity::Column::V3)
                .string_len(125)
                .not_null(),
        )
        .col(
            ColumnDef::new(entity::Column::V4)
                .string_len(125)
                .not_null(),
        )
        .col(
            ColumnDef::new(entity::Column::V5)
                .string_len(125)
                .not_null(),
        )
        .index(
            Index::create()
                .name("unique_key_sea_orm_adapter")
                .unique()
                .table(CasbinRule.table_ref())
                .col(entity::Column::Ptype)
                .col(entity::Column::V0)
                .col(entity::Column::V1)
                .col(entity::Column::V2)
                .col(entity::Column::V3)
                .col(entity::Column::V4)
                .col(entity::Column::V5),
        )
        .to_owned();

    let builder = conn.get_database_backend();

    // println!("sql {:}", builder.build(&create_table));

    conn.execute_raw(builder.build(&create_table)).await
}

#[allow(dead_code)]

pub async fn down<C: ConnectionTrait>(conn: &C) -> Result<ExecResult, DbErr> {
    let drop_table = Table::drop()
        .if_exists()
        .table(entity::Entity.table_ref())
        .to_owned();

    let builder = conn.get_database_backend();

    conn.execute_raw(builder.build(&drop_table)).await
}
