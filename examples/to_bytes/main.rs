use hyper::{http::HeaderValue, Response};

fn main() {
    let mut resp = Response::new(());
    resp.headers_mut()
        .append("Content-Length", HeaderValue::from_static("20"));
    println!("{:?}", resp.into_parts());
}
