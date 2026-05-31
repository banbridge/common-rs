mod layout;

use std::{num::NonZero, sync::Once};

use fastrace::collector::{Config, ConsoleReporter};
use getset::{Getters, Setters};
use layout::{JsonLayout, TextLayout};
use logforth::{
    append::{self, file::FileBuilder},
    record::{Level, LevelFilter},
};

static INIT: Once = Once::new();

#[derive(Debug, Clone, Getters, Setters)]
#[getset(get = "pub", set = "pub")]
pub struct TraceConfig {
    log_directory: String,
    log_file_prefix: String,
    max_log_files: usize,
    max_log_file_size: usize,
}


impl Default for TraceConfig {
    fn default() -> Self {
        Self {
            log_directory: "./logs".to_string(),
            log_file_prefix: "banbridge-rs".to_string(),
            max_log_files: 50,
            max_log_file_size: 1024 * 1024 * 100, // 100 MB
        }
    }
}


pub fn init_log(config: TraceConfig) {
    INIT.call_once(|| {
        let file = FileBuilder::new(&config.log_directory, &config.log_file_prefix)
            .filename_suffix("log")
            .rollover_daily()
            .max_log_files(NonZero::new(config.max_log_files).unwrap())
            .rollover_size(NonZero::new(config.max_log_file_size).unwrap())
            .layout(JsonLayout::default())
            .build()
            .unwrap();

        logforth::starter_log::builder()
            .dispatch(|d| {
                d.filter(LevelFilter::MoreSevereEqual(Level::Info))
                    .diagnostic(logforth::diagnostic::StaticDiagnostic::default())
                    .append(append::Stdout::default().with_layout(TextLayout::default()))
            })
            .dispatch(|d| {
                d.filter(LevelFilter::MoreSevereEqual(Level::Info))
                    .append(file)
            })
            .dispatch(|d| d.append(append::FastraceEvent::default()))
            .apply();

        fastrace::set_reporter(ConsoleReporter, Config::default());
    });
}
