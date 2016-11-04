#[macro_use]
extern crate log;
extern crate hyper;
extern crate env_logger;
#[macro_use]
extern crate prometheus;
extern crate serde;
extern crate serde_xml;
#[macro_use]
extern crate lazy_static;

#[macro_use]
pub mod glusterfs_exporter;

use glusterfs_exporter::http::*;
use hyper::server::Server;

use std::env;
use std::str;

fn main() {
    env_logger::init().unwrap();
    let arg0 = get_process_name();
    info!("starting {}", arg0);

    let default_listen_address = String::from("0.0.0.0:9189");
    let listen_address = env::var("GLUSTER_EXPORTER_LISTEN_ADDRESS").unwrap_or(default_listen_address);
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
