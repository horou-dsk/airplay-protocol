use std::{convert::Infallible, net::SocketAddr};

use hyper::{
    service::{make_service_fn, service_fn},
    Body, Request, Response, Server,
};

async fn handle(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    println!("method = {} uri = {}", req.method(), req.uri());
    Ok(Response::new(Body::from("Hello World")))
}

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    let addr: SocketAddr = ([127, 0, 0, 1], 31927).into();
    // And a MakeService to handle each connection...
    let make_service = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(handle)) });

    // Then bind and serve...
    let server = Server::bind(&addr).serve(make_service);

    // And run forever...
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
    Ok(())
}
