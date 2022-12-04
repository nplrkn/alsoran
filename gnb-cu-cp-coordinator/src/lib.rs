mod config;
mod control;
mod server;

pub use config::{Config, ConnectionControlConfig};
pub use server::spawn;

pub struct Context {}

//pub type Coordinator = server::Server<Context>;
pub use server::Server as Coordinator;
