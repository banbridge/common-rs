use std::{future::Future, pin::Pin};

use base::error::{AppErrorBuilt, AppResult};
use toasty::Db;

fn db_transaction_begin_error(e: toasty::Error) -> AppErrorBuilt {
    AppErrorBuilt::db_transaction_begin_failed(format!("begin transaction failed: {e:?}"))
        .with_base(e.into())
        .print_stack()
}

fn db_transaction_commit_error(e: toasty::Error) -> AppErrorBuilt {
    AppErrorBuilt::db_transaction_commit_failed(format!("commit transaction failed: {e:?}"))
        .with_base(e.into())
        .print_stack()
}

fn db_transaction_rollback_error(e: toasty::Error) -> AppErrorBuilt {
    AppErrorBuilt::db_common(format!("rollback transaction failed: {e:?}"))
        .with_base(e.into())
        .print_stack()
}

