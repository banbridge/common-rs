mod layout;

use std::{num::NonZero, sync::Once};

use fastrace::collector::{Config, ConsoleReporter};
use layout::{JsonLayout, TextLayout};
use logforth::{
    append::{self, file::FileBuilder},
    record::{Level, LevelFilter},
};

static INIT: Once = Once::new();

pub fn init_log() {
    INIT.call_once(|| {
        let file = FileBuilder::new("logs", "banbridge-rs")
            .filename_suffix("log")
            .rollover_daily()
            .max_log_files(NonZero::new(50).unwrap())
            .rollover_size(NonZero::new(1024 * 1024 * 100).unwrap())
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
