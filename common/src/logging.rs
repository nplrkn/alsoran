extern crate slog;

use slog::{Logger};
use sloggers::terminal::{Destination, TerminalLoggerBuilder};
use sloggers::types::{Severity, Format};
use sloggers::Build;

use slog::{o, Drain};

pub fn test_init() -> Logger {
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::CompactFormat::new(decorator).build();
    let drain = std::sync::Mutex::new(drain).fuse();
    let log = slog::Logger::root(drain, o!());
    log
}

pub fn init() -> Logger {
    let mut builder = TerminalLoggerBuilder::new();
    builder.level(Severity::Debug);
    builder.destination(Destination::Stdout);
    builder.format(Format::Compact);

  builder.build().unwrap()
}
