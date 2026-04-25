use std::collections::HashMap;

use base::error::{AppErrorBuilt, AppResult};
use sea_orm::{
    Condition, ConnectionTrait, EntityTrait, PaginatorTrait, QueryFilter, prelude::*,
    sea_query::OnConflict,
};

#[allow(dead_code)]

pub async fn count_by_cond<T>(db: &impl ConnectionTrait, cond: &Condition) -> AppResult<u64>
where
    T: EntityTrait,
    <T as EntityTrait>::Model: Sync,
{
    let query = T::find().filter(cond.to_owned());

    let count = PaginatorTrait::count(query, db).await.map_err(|e| {
        AppErrorBuilt::db_query_failed(format!("count query failed: {:?}", e))
            .with_base(e.into())
            .print_stack()
    })?;

    Ok(count)
}

#[allow(dead_code)]

pub async fn list_by_cond<T>(
    db: &impl ConnectionTrait,
    cond: &Condition,
    page: u64,
    page_size: u64,
) -> AppResult<Vec<T::Model>>
where
    T: EntityTrait,
    <T as EntityTrait>::Model: Sync,
{
    let query = T::find().filter(cond.to_owned());

    let res = PaginatorTrait::paginate(query, db, page_size)
        .fetch_page(page)
        .await
        .map_err(|e| {
            AppErrorBuilt::db_query_failed(format!("list query failed: {:?}", e))
                .with_base(e.into())
                .print_stack()
        })?;

    Ok(res)
}

#[allow(dead_code)]

pub async fn get_by_cond<T>(db: &impl ConnectionTrait, cond: &Condition) -> AppResult<T::Model>
where
    T: EntityTrait,
    <T as EntityTrait>::Model: Sync,
{
    let query = T::find().filter(cond.to_owned());

    let res = query.one(db).await.map_err(|e| {
        AppErrorBuilt::db_query_failed(format!("get query failed: {:?}", e))
            .with_base(e.into())
            .print_stack()
    })?;

    match res {
        Some(m) => Ok(m),
        None => Err(
            AppErrorBuilt::db_query_failed("get query failed: no result".to_string()).print_stack(),
        ),
    }
}

#[allow(dead_code)]

pub async fn delete_by_cond<T>(db: &impl ConnectionTrait, cond: &Condition) -> AppResult<u64>
where
    T: EntityTrait,
    <T as EntityTrait>::Model: Sync,
{
    let query = T::delete_many().filter(cond.to_owned());

    let res = query.exec(db).await.map_err(|e| {
        AppErrorBuilt::db_delete_failed(format!("delete query failed: {:?}", e))
            .with_base(e.into())
            .print_stack()
    })?;

    Ok(res.rows_affected)
}

#[allow(dead_code)]

pub async fn update_with_model<T>(
    db: &impl ConnectionTrait,
    m: T::ActiveModel,
    cond: &Condition,
) -> AppResult<u64>
where
    T: EntityTrait,
    <T as EntityTrait>::Model: Sync,
{
    let query = T::update_many().filter(cond.to_owned()).set(m);

    let res = query.exec(db).await.map_err(|e| {
        AppErrorBuilt::db_update_failed(format!("update query failed: {:?}", e))
            .with_base(e.into())
            .print_stack()
    })?;

    Ok(res.rows_affected)
}

#[allow(dead_code)]

pub async fn update_by_cond<T>(
    db: &impl ConnectionTrait,
    m: HashMap<String, Expr>,
    cond: &Condition,
) -> AppResult<u64>
where
    T: EntityTrait,
    <T as EntityTrait>::Model: Sync,
{
    let mut query = T::update_many().filter(cond.to_owned());

    for (column, expr) in m {
        query = query.col_expr(column, expr);
    }

    let res = query.exec(db).await.map_err(|e| {
        AppErrorBuilt::db_update_failed(format!("update query failed: {:?}", e))
            .with_base(e.into())
            .print_stack()
    })?;

    Ok(res.rows_affected)
}

// 泛型插入接口
#[allow(dead_code)]

pub async fn insert_one<T>(db: &impl ConnectionTrait, model: T::ActiveModel) -> AppResult<u64>
where
    T: EntityTrait,
    <T as EntityTrait>::Model: Sync,
{
    let res = T::insert(model)
        .exec_without_returning(db)
        .await
        .map_err(|e| {
            AppErrorBuilt::db_insert_failed(format!("insert query failed: {:?}", e))
                .with_base(e.into())
                .print_stack()
        })?;

    println!("insert one success: {:?}", res);

    Ok(res)
}

#[allow(dead_code)]

pub async fn insert_many<T>(
    db: &impl ConnectionTrait,
    models: Vec<T::ActiveModel>,
) -> AppResult<u64>
where
    T: EntityTrait,
    <T as EntityTrait>::Model: Sync,
{
    let res = T::insert_many(models)
        .exec_without_returning(db)
        .await
        .map_err(|e| {
            AppErrorBuilt::db_insert_failed(format!("insert query failed: {:?}", e))
                .with_base(e.into())
                .print_stack()
        })?;

    Ok(res)
}

#[allow(dead_code)]

pub async fn insert_many_with_conflict<T>(
    db: &impl ConnectionTrait,
    models: Vec<T::ActiveModel>,
    conflict: OnConflict,
) -> AppResult<u64>
where
    T: EntityTrait,
    <T as EntityTrait>::Model: Sync,
{
    let res = T::insert_many(models)
        .on_conflict(conflict)
        .exec_without_returning(db)
        .await
        .map_err(|e| {
            AppErrorBuilt::db_insert_failed(format!("insert query failed: {:?}", e))
                .with_base(e.into())
                .print_stack()
        })?;

    Ok(res)
}
