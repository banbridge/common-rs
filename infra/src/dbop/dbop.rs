use std::{collections::HashMap, future::Future, pin::Pin};

use base::error::{AppErrorBuilt, AppResult};
use sea_orm::{
    Condition, ConnectionTrait, DatabaseTransaction, DbErr, EntityTrait, PaginatorTrait,
    QueryFilter, TransactionTrait, prelude::*, sea_query::OnConflict,
};

fn db_query_error(action: &str, e: DbErr) -> AppErrorBuilt {
    AppErrorBuilt::db_query_failed(format!("{action} query failed: {e:?}"))
        .with_base(e.into())
        .print_stack()
}

fn db_not_found_error(action: &str) -> AppErrorBuilt {
    AppErrorBuilt::db_not_found(format!("{action} query failed: no result")).print_stack()
}

fn db_update_error(e: DbErr) -> AppErrorBuilt {
    AppErrorBuilt::db_update_failed(format!("update query failed: {e:?}"))
        .with_base(e.into())
        .print_stack()
}

fn db_insert_error(e: DbErr) -> AppErrorBuilt {
    AppErrorBuilt::db_insert_failed(format!("insert query failed: {e:?}"))
        .with_base(e.into())
        .print_stack()
}

fn db_delete_error(e: DbErr) -> AppErrorBuilt {
    AppErrorBuilt::db_delete_failed(format!("delete query failed: {e:?}"))
        .with_base(e.into())
        .print_stack()
}

fn db_transaction_begin_error(e: DbErr) -> AppErrorBuilt {
    AppErrorBuilt::db_transaction_begin_failed(format!("begin transaction failed: {e:?}"))
        .with_base(e.into())
        .print_stack()
}

fn db_transaction_commit_error(e: DbErr) -> AppErrorBuilt {
    AppErrorBuilt::db_transaction_commit_failed(format!("commit transaction failed: {e:?}"))
        .with_base(e.into())
        .print_stack()
}

fn db_transaction_rollback_error(e: DbErr) -> AppErrorBuilt {
    AppErrorBuilt::db_common(format!("rollback transaction failed: {e:?}"))
        .with_base(e.into())
        .print_stack()
}

#[allow(dead_code)]
pub async fn count_by_cond<T>(db: &impl ConnectionTrait, cond: &Condition) -> AppResult<u64>
where
    T: EntityTrait,
    <T as EntityTrait>::Model: Sync,
{
    let query = T::find().filter(cond.to_owned());

    let count = PaginatorTrait::count(query, db)
        .await
        .map_err(|e| db_query_error("count", e))?;

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
        .map_err(|e| db_query_error("list", e))?;

    Ok(res)
}

#[allow(dead_code)]
pub async fn get_by_cond<T>(db: &impl ConnectionTrait, cond: &Condition) -> AppResult<T::Model>
where
    T: EntityTrait,
    <T as EntityTrait>::Model: Sync,
{
    let query = T::find().filter(cond.to_owned());

    let res = query.one(db).await.map_err(|e| db_query_error("get", e))?;

    res.ok_or_else(|| db_not_found_error("get"))
}

#[allow(dead_code)]
pub async fn delete_by_cond<T>(db: &impl ConnectionTrait, cond: &Condition) -> AppResult<u64>
where
    T: EntityTrait,
    <T as EntityTrait>::Model: Sync,
{
    let query = T::delete_many().filter(cond.to_owned());

    let res = query.exec(db).await.map_err(db_delete_error)?;

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

    let res = query.exec(db).await.map_err(db_update_error)?;

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

    let res = query.exec(db).await.map_err(db_update_error)?;

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
        .map_err(db_insert_error)?;

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
        .map_err(db_insert_error)?;

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
        .map_err(db_insert_error)?;

    Ok(res)
}

pub async fn with_shared_txn<T, F>(connection: &DatabaseConnection, f: F) -> AppResult<T>
where
    F: for<'a> FnOnce(
        &'a DatabaseTransaction,
    ) -> Pin<Box<dyn Future<Output = AppResult<T>> + Send + 'a>>,
{
    let txn = connection
        .begin()
        .await
        .map_err(db_transaction_begin_error)?;

    match f(&txn).await {
        Ok(result) => {
            txn.commit().await.map_err(db_transaction_commit_error)?;
            Ok(result)
        }
        Err(err) => {
            txn.rollback()
                .await
                .map_err(db_transaction_rollback_error)?;
            Err(err)
        }
    }
}
