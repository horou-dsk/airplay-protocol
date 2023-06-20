use std::net::SocketAddr;

use airplay2_protocol::airplay::AirPlayConfig;
use airplay2_protocol::airplay_bonjour::AirPlayBonjour;
use airplay2_protocol::control_handle::ControlHandle;
use airplay2_protocol::net::server::Server as MServer;
use env_logger::Env;

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    let mut builder = env_logger::Builder::new();
    let env = Env::default().filter_or("MY_LOG_LEVEL", "info");
    // builder.parse_filters("libmdns=debug");
    builder.parse_env(env);
    builder.init();

    let port = 31927;
    let name = "RustAirplay";

    // pin码认证功能缺失...
    let _air = AirPlayBonjour::new(name, port, false);

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
    Ok(())
}
