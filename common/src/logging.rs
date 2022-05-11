use slog::Logger;
use sloggers::terminal::{Destination, TerminalLoggerBuilder};
use sloggers::types::{Format, Severity};
use sloggers::Build;

use slog::{o, Drain};

pub fn test_init() -> Logger {
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::CompactFormat::new(decorator).build();
    let drain = std::sync::Mutex::new(drain).fuse();
    let drain = slog_envlogger::new(drain);
    slog::Logger::root(drain, o!())
}

pub fn init() -> Logger {
    let mut builder = TerminalLoggerBuilder::new();
    builder.level(Severity::Debug);
    builder.destination(Destination::Stdout);
    builder.format(Format::Compact);

    builder.build().unwrap()
}
