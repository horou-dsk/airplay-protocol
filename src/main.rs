use std::net::SocketAddr;

use airplay2_protocol::airplay::AirPlayConfig;
use env_logger::Env;
// use hyper::{
//     service::{make_service_fn, service_fn},
//     Body, Request, Response, Server,
// };
use airplay2_protocol::airplay_bonjour::AirPlayBonjour;
use airplay2_protocol::control_handle::ControlHandle;
use airplay2_protocol::net::server::Server as MServer;
// use tokio::{io::AsyncBufReadExt, net::TcpListener};

// async fn handle(req: Request<Body>) -> Result<Response<Body>, Infallible> {
//     // hyper::body::to_bytes(req.into_body())
//     println!("method = {} uri = {}", req.method(), req.uri());
//     Ok(Response::new(Body::from("Hello World")))
// }

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    let mut builder = env_logger::Builder::new();
    let env = Env::default().filter_or("MY_LOG_LEVEL", "info");
    // builder.parse_filters("libmdns=debug");
    builder.parse_env(env);
    builder.init();

    let port = 31927;
    let name = "RustAirplay";

    let _air = AirPlayBonjour::new(name, port, true);

    let addr: SocketAddr = ([0, 0, 0, 0], port).into();
    let airplay_config = AirPlayConfig {
        server_name: name.to_string(),
        width: 1920,
        height: 1080,
        fps: 30,
        port,
    };
    let mserver = MServer::bind(addr, ControlHandle::new(airplay_config));
    mserver.run().await?;
    // And a MakeService to handle each connection...
    // let make_service = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(handle)) });
    // let listener = TcpListener::bind(addr).await?;
    // loop {
    //     log::warn!("Start Server ..........");
    //     let (mut socket, _) = listener.accept().await?;
    //     tokio::task::spawn(async move {
    //         log::warn!("连接进入....");
    //         let mut buf = Vec::new();
    //         let mut reader = tokio::io::BufReader::new(&mut socket);
    //         let mut initial_line = String::new();
    //         reader.read_line(&mut initial_line);
    //         log::info!("{}", String::from_utf8_lossy(&buf));
    //     });
    // }
    // Then bind and serve...
    // let server = Server::bind(&addr).serve(make_service);

    // And run forever...
    // if let Err(e) = server.await {
    //     eprintln!("server error: {}", e);
    // }
    Ok(())
}
