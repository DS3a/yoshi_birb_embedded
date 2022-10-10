use esp_idf_sys as _;

use std::io::{Read, Write};
use std::str::from_utf8; // find a way to send binary data alone
use std::net::{Ipv4Addr, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use utils::wifi_ap_lib;
use utils::wifi_ap_lib::Wifi;
use yoshi_msgs::yoshi_msgs;
use serde_json;

use esp_idf_hal::delay::Ets;
use esp_idf_hal::ledc::{config::TimerConfig, Channel, Timer};
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::prelude::*;

static HOST_ADDRESS: &str = "11.42.0.2:3000";

fn main() {
    esp_idf_sys::link_patches();

    let is_connected_handler: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
    let conf = wifi_ap_lib::generate_conf("yoshibirb_ap", "qwertyuiop", Ipv4Addr::new(11, 42, 0, 1));
    let mut wifi_handler = wifi_ap_lib::generate_handler();
    wifi_handler.set_configuration(&conf).unwrap();

    let tcp_thread_is_connected_handler = Arc::clone(&is_connected_handler);
    thread::spawn(move || {
        loop {
            match TcpStream::connect(HOST_ADDRESS) {
                Ok(mut stream) => {
                    println!("connected to host");
                    let msg_from_esp = yoshi_msgs::MsgFromEsp {
                        acc_x: 2f64,
                        acc_y: 2.231f64,
                        acc_z: 1.24f64,
                    };
                    let j = serde_json::to_string(&msg_from_esp).unwrap();
                    loop {
                        *tcp_thread_is_connected_handler.lock().unwrap() = true;
                        let msg = j.as_bytes();
                        stream.write(msg).unwrap();
                        println!("Sent Hello, awaiting reply...");

                        let mut data = [0 as u8; 500]; // using 6 byte buffer
                        match stream.read(&mut data) {
                            Ok(_) => {
                                    println!("Received data");
                            },
                            Err(e) => {
                                println!("Failed to receive data: {}", e);
                                break;
                            }
                        }
                    }
                },
                Err(_) => {
                    println!("connection failed, gonna try again");
                    *tcp_thread_is_connected_handler.lock().unwrap() = false;
                }
            }
        }
    });

    loop {

    }
}
