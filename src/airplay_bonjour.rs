use std::net::{IpAddr, Ipv4Addr};

use default_net::interface::MacAddr;
use libmdns::Service;

const AIRPLAY_SERVICE_TYPE: &str = "_airplay._tcp";
#[allow(dead_code)]
const AIRTUNES_SERVICE_TYPE: &str = "_raop._tcp";

fn get_ip() -> Result<Vec<(Ipv4Addr, Ipv4Addr, Option<MacAddr>)>, String> {
    if cfg!(windows) {
        let default_interface = default_net::get_default_interface()?;
        let mac_addr = default_interface.mac_addr;
        Ok(default_interface
            .ipv4
            .into_iter()
            .map(|ip| (ip.addr, ip.netmask, mac_addr.clone()))
            .collect())
    } else {
        let mut ip_list = Vec::new();
        let interfaces = default_net::get_interfaces();
        for interface in interfaces {
            if interface.if_type == default_net::interface::InterfaceType::Ethernet
                || interface.if_type == default_net::interface::InterfaceType::Wireless80211
                || interface.name == "wlan0"
            {
                let mac_addr = interface.mac_addr;
                for ip in interface.ipv4 {
                    ip_list.push((ip.addr, ip.netmask, mac_addr.clone()));
                }
            }
        }
        if ip_list.is_empty() {
            let default_interface = default_net::get_default_interface()?;
            let mac_addr = default_interface.mac_addr;
            ip_list = default_interface
                .ipv4
                .into_iter()
                .map(|ip| (ip.addr, ip.netmask, mac_addr.clone()))
                .collect();
        }
        Ok(ip_list)
    }
}

pub struct AirPlayBonjour {
    _services: Vec<Service>,
}

impl AirPlayBonjour {
    pub fn new(service_name: &str, port: u16, pw: bool) -> Self {
        let mut services = Vec::new();
        let local_ips = get_ip().unwrap();
        let responder = libmdns::Responder::new_with_ip_list(
            local_ips.iter().map(|v| IpAddr::V4(v.0)).collect(),
        )
        .expect("libmdns new error!");
        let mac = local_ips[0]
            .2
            .as_ref()
            .map(|v| v.to_string())
            .unwrap_or_else(|| "00:AA:BB:CC:DD:FF".to_string()); // TODO: Fix Me
        let props = vec![
            ("deviceid", mac.to_string()),
            ("features", "0x5A7FFFF7,0x1E".to_string()),
            ("srcvers", "220.68".to_string()),
            ("flags", "0x44".to_string()),
            ("vv", "2".to_string()),
            ("model", "AppleTV3,2C".to_string()),
            ("rhd", "5.6.0.0".to_string()),
            ("pw", pw.to_string()), // 是否需要密码认证
            (
                "pk",
                "b07727d6f6cd6e08b58ede525ec3cdeaa252ad9f683feb212ef8a205246554e7".to_string(),
            ),
            ("rmodel", "PC1.0".to_string()),
            ("rrv", "1.01".to_string()),
            ("rsv", "1.00".to_string()),
            ("pcversion", "1715".to_string()),
        ];
        let props: Vec<String> = props.iter().map(|v| format!("{}={}", v.0, v.1)).collect();
        let props: Vec<&str> = props.iter().map(|v| v.as_str()).collect();
        let svc = responder.register(
            AIRPLAY_SERVICE_TYPE.to_string(),
            service_name.into(),
            port,
            &props[..],
        );
        services.push(svc);
        let service_name = format!("{}@{}", mac.replace(':', ""), service_name);
        let props = vec![
            ("ch", "2"),
            ("cn", "1,3"),
            ("da", "true"),
            ("et", "0,3,5"),
            ("ek", "1"),
            ("ft", "0x5A7FFFF7,0x1E"),
            ("am", "AppleTV3,2C"),
            ("md", "0,1,2"),
            ("sr", "44100"),
            ("ss", "16"),
            ("sv", "false"),
            ("sm", "false"),
            ("tp", "UDP"),
            ("txtvers", "1"),
            ("sf", "0x44"),
            ("vs", "220.68"),
            ("vn", "65537"),
            (
                "pk",
                "b07727d6f6cd6e08b58ede525ec3cdeaa252ad9f683feb212ef8a205246554e7",
            ),
        ];
        let props: Vec<String> = props.iter().map(|v| format!("{}={}", v.0, v.1)).collect();
        let props: Vec<&str> = props.iter().map(|v| v.as_str()).collect();
        let svc = responder.register(AIRPLAY_SERVICE_TYPE.to_string(), service_name, port, &props);
        services.push(svc);
        Self {
            _services: services,
        }
    }
}
