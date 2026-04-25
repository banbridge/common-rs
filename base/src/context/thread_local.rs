use std::cell::RefCell;

use metainfo::MetaInfo;

use crate::context::types::LogId;

thread_local! {
pub static LOG_ID: RefCell<Option<LogId>> = RefCell::new(None);

pub static METAINFO: RefCell<Option<MetaInfo>> = RefCell::new(None);
}
