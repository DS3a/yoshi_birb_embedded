use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use wifi_ap_lib;
use wifi_ap_lib::Wifi;

use std::net::{Ipv4Addr, TcpStream};

fn main() {
    // Temporary. Will disappear once ESP-IDF 4.4 is released, but for now it is necessary to call this function once,
    // or else some patches to the runtime implemented by esp-idf-sys might not link properly.
    esp_idf_sys::link_patches();


    let conf = wifi_ap_lib::generate_conf("yoshibirb_ap", "qwertyuiop", Ipv4Addr::new(11, 42, 0, 1));
    let mut wifi_handler = wifi_ap_lib::generate_handler();
    wifi_handler.set_configuration(&conf).unwrap();
    println!("Hello, world!");
    loop {
    }
}
