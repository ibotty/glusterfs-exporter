#[macro_use]
extern crate log;
extern crate hyper;
extern crate env_logger;
extern crate prometheus;

#[macro_use]
pub mod glusterfs_exporter {
    #[macro_use]
    pub mod errors;
    pub mod commands;
    pub mod http;
}

use glusterfs_exporter::errors::*;
use glusterfs_exporter::http::*;

use std::env;
use std::str;
use std::process::Command;
use hyper::server::Server;

fn main() {
    env_logger::init().unwrap();
    let arg0 = get_process_name();
    info!("starting {}", arg0);


    let default_listen_address = String::from("0.0.0.0:9189");
    let listen_address = env::var("GLUSTER_EXPORTER_LISTEN_ADDRESS")
        .unwrap_or(default_listen_address);

    info!("listening on {}", listen_address);

    let _server = Server::http(listen_address.as_str())
        .unwrap()
        .handle(serve_metrics)
        .unwrap();
}

fn get_process_name() -> String {
    let default_name = "gluster_exporter";

    env::current_exe()
        .ok()
        .as_ref()
        .and_then(|p| p.to_str())
        .unwrap_or(default_name)
        .to_string()
}
