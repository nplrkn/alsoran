mod config;
mod control;
mod server;

pub use config::Config;

pub struct Context {}

pub type Coordinator = server::Server<Context>;
