use std::cell::RefCell;

use metainfo::{METAINFO, MetaInfo};
use tokio::task_local;

use crate::context::types::LogId;

task_local! {

    #[allow(clippy::declare_interior_mutable_const)]
    pub static LOG_ID: RefCell<Option<LogId>>;
}

pub fn spawn<T>(future: T) -> tokio::task::JoinHandle<T::Output>
where
    T: Future + Send + 'static,
    T::Output: Send + 'static,
{
    let (log_id, mi) = get_log_id_and_metainfo();

    // let current_span = tracing::Span::current();

    tokio::spawn(LOG_ID.scope(
        RefCell::new(log_id),
        METAINFO.scope(RefCell::new(mi), future),
    ))
}

// pub fn spawn_blocking<F, R>(f: F) -> tokio::task::JoinHandle<R>
// where
//     F: FnOnce() -> R + Send + 'static,
//     R: Send + 'static,
// {
//     let (log_id, mi) = get_log_id_and_metainfo();

//     let current_span = tracing::Span::current();

//     tokio::task::spawn_blocking(move || {
//         let _enter = current_span.enter();

//         LOG_ID.sync_scope(RefCell::new(log_id), || {
//             METAINFO.sync_scope(RefCell::new(mi), f)
//         })
//     })
// }

// pub fn spawn_local<T>(future: T) -> tokio::task::JoinHandle<T::Output>
// where
//     T: Future + 'static,
//     T::Output: 'static,
// {
//     let (log_id, mi) = get_log_id_and_metainfo();

//     let current_span = tracing::Span::current();

//     tokio::task::spawn_local(LOG_ID.scope(
//         RefCell::new(log_id),
//         METAINFO.scope(RefCell::new(mi), future.instrument(current_span)),
//     ))
// }

// pub fn spawn_with_handler<T>(
//     future: T,
//     handler: tokio::runtime::Handle,
// ) -> tokio::task::JoinHandle<T::Output>
// where
//     T: Future + Send + 'static,
//     T::Output: Send + 'static,
// {
//     let (log_id, mi) = get_log_id_and_metainfo();

//     let current_span = tracing::Span::current();

//     handler.spawn(LOG_ID.scope(
//         RefCell::new(log_id),
//         METAINFO.scope(RefCell::new(mi), future.instrument(current_span)),
//     ))
// }

#[allow(unused)]
fn get_log_id_and_metainfo() -> (Option<LogId>, MetaInfo) {
    let log_id = LOG_ID
        .try_with(|log_id| log_id.borrow().clone())
        .unwrap_or(None);

    let mi = METAINFO
        .try_with(|mi| {
            let pre_mi = mi.take();

            let (m1, m2) = pre_mi.derive();

            mi.replace(m1);

            m2
        })
        .unwrap_or_else(|_| MetaInfo::new());

    (log_id, mi)
}
