#[macro_use]
extern crate log;
extern crate hyper;
extern crate env_logger;
extern crate prometheus;

use std::env;
use hyper::header;
use hyper::uri::RequestUri;
use hyper::server::{Server, Request, Response};
use hyper::status::StatusCode;
use hyper::mime::Mime;
use prometheus::{Encoder, TextEncoder};

fn main() {
    env_logger::init().unwrap();
    let arg0 = get_process_name();
    info!("starting {}", arg0);

    fn serve_metrics(req: Request, mut res: Response) {
        match req.method {
            hyper::Get => {
                match req.uri {
                    RequestUri::AbsolutePath(s) => {
                        match s.as_str() {
                            "/metrics/" => write_metrics(res),
                            "/healthz/" => write_healthz(res),
                            "/metrics" => write_redirect(res, "/metrics/"),
                            "/healthz" => write_redirect(res, "/healthz/"),
                            _ => write_notfound(res),
                        }
                    }
                    _ => write_notfound(res),
                }
            }
            _ => *res.status_mut() = StatusCode::MethodNotAllowed,
        }
    }

    fn write_notfound(mut res: Response) {
        *res.status_mut() = StatusCode::NotFound;
    }

    fn write_redirect(mut res: Response, uri: &str) {
        *res.status_mut() = StatusCode::MovedPermanently;
        res.headers_mut().set(header::Location(uri.to_string()));
    }

    fn write_healthz(mut res: Response) {}

    fn write_metrics(mut res: Response) {
        let metric_familys = prometheus::gather();
        let mut buffer = vec![];
        let encoder = TextEncoder::new();

        encoder.encode(&metric_familys, &mut buffer).unwrap();
        res.headers_mut().set(header::ContentType(encoder.format_type().parse::<Mime>().unwrap()));
        res.send(&buffer).unwrap();
    }

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

    env::current_exe().ok().as_ref().and_then(|p| p.to_str()).unwrap_or(default_name).to_string()
}
