use jiff::tz::TimeZone;
use logforth::{Diagnostic, Error, Layout, record::Record};

use super::RecordLine;

#[derive(Default, Debug, Clone)]
pub struct JsonLayout {
    tz: Option<TimeZone>,
}

impl JsonLayout {
    /// Set the timezone for timestamps.
    ///
    /// # Examples
    ///
    /// ```
    /// use jiff::tz::TimeZone;
    /// use logforth_layout_json::JsonLayout;
    ///
    /// let layout = JsonLayout::default().timezone(TimeZone::UTC);
    /// ```
    pub fn timezone(mut self, tz: TimeZone) -> Self {
        self.tz = Some(tz);
        self
    }
}

impl Layout for JsonLayout {
    fn format(&self, record: &Record, diags: &[Box<dyn Diagnostic>]) -> Result<Vec<u8>, Error> {
        let diagnostics = diags;

        let record_line = RecordLine::from_record(record, diagnostics, self.tz.clone())?;

        // SAFETY: RecordLine is serializable.
        Ok(serde_json::to_vec(&record_line).unwrap_or_default())
    }
}
