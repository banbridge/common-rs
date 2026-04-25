use metainfo::MetaInfo;

use crate::context::types::LogId;

#[allow(dead_code)]

pub trait ChrysaorMetaInfo {
    fn get_log_id(&self) -> Option<LogId>;

    fn set_log_id(&mut self, log_id: LogId);
}

impl ChrysaorMetaInfo for MetaInfo {
    #[inline]

    fn get_log_id(&self) -> Option<LogId> {
        self.get_faststr::<LogId>().map(|l| l.to_owned())
        // .cloned()
        // .map(LogId::from)
    }

    #[inline]

    fn set_log_id(&mut self, log_id: LogId) {
        self.insert_faststr::<LogId>(log_id)
    }
}
