pub use embedded_svc::wifi::Wifi;
use embedded_svc::wifi::{Configuration, AccessPointConfiguration, AuthMethod, Protocol};
use embedded_svc::ipv4::{RouterConfiguration, Subnet, Mask};
use enumset::enum_set;
use esp_idf_svc::netif::EspNetifStack;
pub use esp_idf_svc::wifi::EspWifi;
use esp_idf_svc::sysloop::EspSysLoopStack;
use esp_idf_svc::nvs::EspDefaultNvs;
use heapless;
use std::net::Ipv4Addr;
use std::sync::Arc;
/*
pub fn create_ap(ssid: &str, password: &str, ip_addr: Ipv4Addr) {
    let wifi_conf = Configuration::AccessPoint(AccessPointConfiguration {
        ssid: heapless::String::from(ssid),
        ssid_hidden: false,
        channel: 2u8,
        secondary_channel: Some(11u8),
        protocols: enum_set!(Protocol::P802D11B),
        auth_method: AuthMethod::WPA2Personal,
        password: heapless::String::from(password),
        max_connections: 5u16,
        ip_conf: Some(RouterConfiguration {
            subnet: Subnet {
                gateway: ip_addr.clone(),
                mask: Mask(24),
            },
            dhcp_enabled: true,
            dns: None,
            secondary_dns: None
        }),
    });



    esp_wifi_handler.set_configuration(&wifi_conf).unwrap();
}
*/
pub fn generate_conf(ssid: &str, password: &str, ip_addr: Ipv4Addr) -> Configuration {
    Configuration::AccessPoint(AccessPointConfiguration {
        ssid: heapless::String::from(ssid),
        ssid_hidden: false,
        channel: 2u8,
        secondary_channel: Some(11u8),
        protocols: enum_set!(Protocol::P802D11B),
        auth_method: AuthMethod::WPA2Personal,
        password: heapless::String::from(password),
        max_connections: 5u16,
        ip_conf: Some(RouterConfiguration {
            subnet: Subnet {
                gateway: ip_addr.clone(),
                mask: Mask(24),
            },
            dhcp_enabled: true,
            dns: None,
            secondary_dns: None
        }),
    })
}

pub fn generate_handler() -> EspWifi {
    let mut esp_wifi_handler = EspWifi::new(Arc::new(EspNetifStack::new().unwrap()),
                                         Arc::new(EspSysLoopStack::new().unwrap()),
                                         Arc::new(EspDefaultNvs::new().unwrap())).unwrap();
    esp_wifi_handler
}

// Go to Cargo.toml and add this
/*
[lib]
name = "wifi_ap_lib"
path = "src/wifi_ap_lib.rs"
*/
