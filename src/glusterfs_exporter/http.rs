extern crate hyper;
extern crate prometheus;

use glusterfs_exporter::commands::*;
use glusterfs_exporter::metrics::*;

use hyper::header;
use hyper::mime::Mime;
use hyper::server::{Request, Response};
use hyper::status::StatusCode;
use hyper::uri::RequestUri;
use prometheus::{Encoder, TextEncoder};

fn strip_slash(s: &str) -> &str {
    s.trim_right_matches('/')
}

pub fn serve_metrics(req: Request, mut res: Response) {
    match req.method {
        hyper::Get => {
            match req.uri {
                RequestUri::AbsolutePath(s) => {
                    match strip_slash(s.as_str()) {
                        "/metrics" => write_metrics(res),
                        "/healthz" => write_healthz(res),
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

pub fn write_healthz(_res: Response) {
}

pub fn write_metrics(mut res: Response) {
    collect_stats().map_err(|e| { EXPORTER_FAILURE_COUNTER.inc(); e}).unwrap();

    let metric_familys = prometheus::gather();
    let mut buffer = vec![];
    let encoder = TextEncoder::new();

    encoder.encode(&metric_familys, &mut buffer).unwrap();
    res.headers_mut()
        .set(header::ContentType(encoder.format_type().parse::<Mime>().unwrap()));
    res.send(&buffer).unwrap();
}
