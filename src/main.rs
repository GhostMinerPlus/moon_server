//! Parse configire and startup server

mod server;
mod state;

use serde::Deserialize;
use std::{fs, io};

#[derive(Deserialize, Clone)]
struct Config {
    ip: String,
    port: u16,
    name: String,
    thread_num: u8,
}

fn main() -> io::Result<()> {
    let arg_v: Vec<String> = std::env::args().collect();
    let file_name = if arg_v.len() == 2 {
        arg_v[1].as_str()
    } else {
        "config.toml"
    };
    let config_s = fs::read_to_string(file_name)?;
    let config: Config =
        toml::from_str(&config_s).map_err(|e| io::Error::new(io::ErrorKind::Other, e.message()))?;
    // Config log
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("INFO")).init();
    // Run server
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .worker_threads(config.thread_num as usize)
        .build()?
        .block_on(server::Server::new(config.ip, config.port, config.name).run())
}
