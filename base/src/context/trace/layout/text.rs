use jiff::tz::TimeZone;
use logforth::{Diagnostic, Error, Layout, record::Record};

use super::RecordLine;

#[derive(Debug, Clone, Default)]
pub struct TextLayout {
    tz: Option<TimeZone>,
}

impl TextLayout {
    /// Set the timezone for timestamps.
    ///
    /// # Examples
    ///
    /// ```
    /// use jiff::tz::TimeZone;
    /// use logforth_layout_text::TextLayout;
    ///
    /// let layout = TextLayout::default().timezone(TimeZone::UTC);
    /// ```
    pub fn timezone(mut self, tz: TimeZone) -> Self {
        self.tz = Some(tz);
        self
    }
}

impl Layout for TextLayout {
    fn format(&self, record: &Record, diags: &[Box<dyn Diagnostic>]) -> Result<Vec<u8>, Error> {
        let record_line = RecordLine::from_record(record, diags, self.tz.clone())?;

        Ok(record_line.text().unwrap_or_default().into_bytes())
    }
}
