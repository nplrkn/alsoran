//! logging - functions for setting up the root logger

use slog::Logger;
use slog::{o, Drain};

pub fn test_init() -> Logger {
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::CompactFormat::new(decorator).build();
    let drain = std::sync::Mutex::new(drain).fuse();
    let drain = slog_envlogger::new(drain);
    slog::Logger::root(drain, o!())
}

pub fn init() -> Logger {
    // Use info level logging by default
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info")
    }

    init_terminal_logging()
}

pub fn init_terminal_logging() -> Logger {
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    let drain = slog_envlogger::new(drain);
    slog::Logger::root(drain, o!())
}
