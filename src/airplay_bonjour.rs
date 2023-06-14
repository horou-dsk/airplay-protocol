use std::net::IpAddr;

use libmdns::Service;

const AIRPLAY_SERVICE_TYPE: &str = "_airplay._tcp";
const AIRTUNES_SERVICE_TYPE: &str = "_raop._tcp";

pub struct AirPlayBonjour {
    _services: Vec<Service>,
}

impl AirPlayBonjour {
    pub fn new(service_name: &str, port: u16) -> Self {
        let mut services = Vec::new();
        let interface = default_net::get_default_interface().unwrap();
        let ip = IpAddr::V4(interface.ipv4[0].addr);
        let responder = libmdns::Responder::new_with_ip_list(vec![ip]).unwrap();
        let mac = interface.mac_addr.unwrap().to_string(); //"DC:21:48:FE:13:2A";
        let props = vec![
            ("deviceid", mac.to_string()),
            ("features", "0x5A7FFFF7,0x1E".to_string()),
            ("srcvers", "220.68".to_string()),
            ("flags", "0x44".to_string()),
            ("vv", "2".to_string()),
            ("model", "AppleTV3,2C".to_string()),
            ("rhd", "5.6.0.0".to_string()),
            ("pw", "true".to_string()), // 是否需要密码认证
            (
                "pk",
                "f3769a660475d27b4f6040381d784645e13e21c53e6d2da6a8c3d757086fc336".to_string(),
            ),
            ("rmodel", "PC1.0".to_string()),
            ("rrv", "1.01".to_string()),
            ("rsv", "1.00".to_string()),
            ("pcversion", "1715".to_string()),
        ];
        let props: Vec<String> = props.iter().map(|v| format!("{}={}", v.0, v.1)).collect();
        let props: Vec<&str> = props.iter().map(|v| v.as_str()).collect();
        let svc = responder.register(
            format!("{}", AIRPLAY_SERVICE_TYPE),
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
                "f3769a660475d27b4f6040381d784645e13e21c53e6d2da6a8c3d757086fc336",
            ),
        ];
        let props: Vec<String> = props.iter().map(|v| format!("{}={}", v.0, v.1)).collect();
        let props: Vec<&str> = props.iter().map(|v| v.as_str()).collect();
        let svc = responder.register(
            format!("{}", AIRTUNES_SERVICE_TYPE),
            service_name,
            port,
            &props,
        );
        services.push(svc);
        Self {
            _services: services,
        }
    }
}
