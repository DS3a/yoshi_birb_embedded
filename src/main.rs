// YOSHI ESP
use esp_idf_sys as _;

use std::io::{Read, Write};
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

    let msg_for_esp_global_handler = Arc::new(Mutex::new(yoshi_msgs::MsgForEsp::default()));
    let msg_from_esp_global_handler = Arc::new(Mutex::new(yoshi_msgs::MsgFromEsp::default()));
    let is_connected_handler: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));

    let conf = wifi_ap_lib::generate_conf("yoshibirb_ap", "qwertyuiop", Ipv4Addr::new(11, 42, 0, 1));
    let mut wifi_handler = wifi_ap_lib::generate_handler();
    wifi_handler.set_configuration(&conf).unwrap();
    // initiate a wifi accesspoint with name yoshibirb_ap for the RL agent to connect to

    let tcp_thread_msg_for_esp_handler = Arc::clone(&msg_for_esp_global_handler);
    let tcp_thread_msg_from_esp_handler = Arc::clone(&msg_from_esp_global_handler);
    let tcp_thread_is_connected_handler = Arc::clone(&is_connected_handler);
    thread::spawn(move || {
        /*
        This connects to the TCP server hosted at the HOST_ADDRESS, to send data regarding imu
        and hopefully the lighthouse too
        it receives data regarding the pwm signals that have to be sent to the motors
        */
        loop {
            match TcpStream::connect(HOST_ADDRESS) {
                Ok(mut stream) => {
                    println!("connected to host");

                    loop {
                        *tcp_thread_is_connected_handler.lock().unwrap() = true;
                        let msg_from_esp = (*tcp_thread_msg_from_esp_handler.lock().unwrap()).clone();
                        let str_msg_from_esp = serde_json::to_string(&msg_from_esp).unwrap();
                        let bin_msg_from_esp = str_msg_from_esp.as_bytes();
                        stream.write(bin_msg_from_esp).unwrap();
                        println!("Sent Hello, awaiting reply...");

                        let mut data = [0 as u8; 500];
                        match stream.read(&mut data) {
                            Ok(_) => {
                                let mut str_data = std::str::from_utf8(&data).unwrap();
                                str_data = str_data.trim_matches(char::from(0));
                                *tcp_thread_msg_for_esp_handler.lock().unwrap() = serde_json::from_str(str_data).unwrap();
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


    let msg_from_esp_imu_thread_handler = Arc::clone(&msg_from_esp_global_handler);
    // TODO imu thread
    thread::spawn(move || {
        loop {
            /*
            TODO this thread gets data from the imu and saves it in msg_from_esp
            This will be sent back to the main system to generate rewards, and actions
            */
            thread::sleep(std::time::Duration::from_nanos(5));
        }
    });

    let msg_for_esp_main_handler = Arc::clone(&msg_for_esp_global_handler);
    let peripherals = Peripherals::take().unwrap();
    let config = TimerConfig::default().frequency(25.kHz().into());
    let timer = Timer::new(peripherals.ledc.timer0, &config).unwrap();
    let mut pin_8 = Channel::new(peripherals.ledc.channel0, &timer, peripherals.pins.gpio26).unwrap();
    let pin_8_max_duty = pin_8.get_max_duty();
    let mut pin_9 = Channel::new(peripherals.ledc.channel1, &timer, peripherals.pins.gpio25).unwrap();
    let pin_9_max_duty = pin_9.get_max_duty();
    let mut pin_10 = Channel::new(peripherals.ledc.channel2, &timer, peripherals.pins.gpio33).unwrap();
    let pin_10_max_duty = pin_10.get_max_duty();
    let mut pin_11 = Channel::new(peripherals.ledc.channel3, &timer, peripherals.pins.gpio32).unwrap();
    let pin_11_max_duty = pin_11.get_max_duty();
    loop {
        /*
        This main loop takes data about the pwm values which
        have to be written to the motors and send it to the motors
         */
        // TODO check if the delay is too high or low
        let mut msg_for_esp = (*msg_for_esp_main_handler.lock().unwrap()).clone();
        if msg_for_esp.front_left < 0.0 { msg_for_esp.front_left = 0.0 };
        if msg_for_esp.front_right < 0.0 { msg_for_esp.front_right = 0.0 };
        if msg_for_esp.back_right < 0.0 { msg_for_esp.back_right = 0.0 };
        if msg_for_esp.back_left < 0.0 { msg_for_esp.back_left = 0.0 };

        pin_8.set_duty(((pin_8_max_duty as f64) * msg_for_esp.front_left) as u32);
        pin_9.set_duty(((pin_9_max_duty as f64) * msg_for_esp.front_right) as u32);
        pin_10.set_duty(((pin_10_max_duty as f64) * msg_for_esp.back_right) as u32);
        pin_11.set_duty(((pin_11_max_duty as f64) * msg_for_esp.back_left) as u32);
        thread::sleep(std::time::Duration::from_nanos(5));
    }
}

