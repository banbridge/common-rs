use std::{fmt::Write, str::FromStr};

use faststr::FastStr;
use jiff::{Timestamp, TimestampDisplayWithOffset, tz::TimeZone};
use logforth::{
    Diagnostic, Error,
    kv::{Key, Value, Visitor},
    layout::text::colored::{Color, ColoredString, Colorize},
    record::{Level, Record},
};
use serde::Serialize;
use serde_json::Map;

use crate::log_id::get_or_default_log_id;

#[derive(Debug, Clone, Serialize)]
pub(super) struct RecordLine<'a> {
    #[serde(serialize_with = "serialize_timestamp")]
    timestamp: TimestampDisplayWithOffset,
    level: &'a str,
    target: &'a str,
    file: &'a str,
    line: u32,
    message: &'a str,
    #[serde(skip_serializing_if = "Map::is_empty")]
    kvs: Map<String, serde_json::Value>,
    #[serde(skip_serializing_if = "Map::is_empty")]
    diags: Map<String, serde_json::Value>,
    log_id: FastStr,
}

fn serialize_timestamp<S>(
    timestamp: &TimestampDisplayWithOffset,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.collect_str(&format_args!("{timestamp:.6}"))
}

struct KvCollector<'a> {
    kvs: &'a mut Map<String, serde_json::Value>,
}

impl Visitor for KvCollector<'_> {
    fn visit(&mut self, key: Key, value: Value) -> Result<(), Error> {
        let key = key.to_string();
        match serde_json::to_value(&value) {
            Ok(value) => self.kvs.insert(key, value),
            Err(_) => self.kvs.insert(key, value.to_string().into()),
        };
        Ok(())
    }
}

impl<'a> RecordLine<'a> {
    pub fn from_record(
        record: &'a Record,
        diags: &[Box<dyn Diagnostic>],
        tz: Option<TimeZone>,
    ) -> Result<Self, Error> {
        let diagnostics = diags;

        // SAFETY: jiff::Timestamp::try_from only fails if the time is out of range,
        // which is very unlikely if the system clock is correct.
        let ts = Timestamp::try_from(record.time()).unwrap();
        let tz = tz.clone().unwrap_or_else(TimeZone::system);
        let offset = tz.to_offset(ts);
        let timestamp = ts.display_with_offset(offset);

        let mut kvs = Map::new();
        let mut kvs_visitor = KvCollector { kvs: &mut kvs };
        record.key_values().visit(&mut kvs_visitor)?;

        let mut diags = Map::new();
        let mut diags_visitor = KvCollector { kvs: &mut diags };
        for d in diagnostics {
            d.visit(&mut diags_visitor)?;
        }

        let log_id = get_or_default_log_id();

        let record_line = RecordLine {
            timestamp,
            level: record.level().name(),
            target: record.target(),
            file: record.file().unwrap_or_default(),
            line: record.line().unwrap_or_default(),
            message: record.payload(),
            kvs,
            diags,
            log_id,
        };
        Ok(record_line)
    }

    pub fn level(&self) -> &'a str {
        self.level
    }

    pub fn target(&self) -> &'a str {
        self.target
    }

    pub fn file(&self) -> &'a str {
        self.file
    }

    pub fn line(&self) -> u32 {
        self.line
    }

    pub fn message(&self) -> &'a str {
        self.message
    }

    pub fn time(&self) -> &TimestampDisplayWithOffset {
        &self.timestamp
    }

    pub fn text(&self) -> Result<String, Error> {
        let level =
            LevelColor::default().colorize_record_level(false, Level::from_str(self.level())?);
        let target = self.target();
        let file = self.file();
        let line = self.line();
        let message = self.message();

        let time = self.time();

        let log_id = self.log_id.as_str();

        let mut text = format!("{time:.6} {level:>6} {log_id} {target}: {file}:{line} {message}");

        for (k, v) in self.kvs.iter() {
            write!(&mut text, " {}={}", k, v).unwrap_or_default();
        }

        Ok(text)
    }
}

/// Colors for different log levels.
#[derive(Debug, Clone)]
struct LevelColor {
    /// Color for fatal level logs.
    fatal: Color,
    /// Color for error level logs.
    error: Color,
    /// Color for warning level logs.
    warn: Color,
    /// Color for info level logs.
    info: Color,
    /// Color for debug level logs.
    debug: Color,
    /// Color for trace level logs.
    trace: Color,
}

impl Default for LevelColor {
    fn default() -> Self {
        Self {
            fatal: Color::BrightRed,
            error: Color::Red,
            warn: Color::Yellow,
            info: Color::Green,
            debug: Color::Blue,
            trace: Color::Magenta,
        }
    }
}

impl LevelColor {
    /// Colorize the log level.
    fn colorize_record_level(&self, no_color: bool, level: Level) -> ColoredString {
        if no_color {
            ColoredString::from(level.to_string())
        } else {
            let color = match level {
                Level::Fatal | Level::Fatal2 | Level::Fatal3 | Level::Fatal4 => self.fatal,
                Level::Error | Level::Error2 | Level::Error3 | Level::Error4 => self.error,
                Level::Warn | Level::Warn2 | Level::Warn3 | Level::Warn4 => self.warn,
                Level::Info | Level::Info2 | Level::Info3 | Level::Info4 => self.info,
                Level::Debug | Level::Debug2 | Level::Debug3 | Level::Debug4 => self.debug,
                Level::Trace | Level::Trace2 | Level::Trace3 | Level::Trace4 => self.trace,
            };
            ColoredString::from(level.to_string()).color(color)
        }
    }
}
