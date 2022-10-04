use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize, Debug)]
pub struct MsgForEsp {
    /*
    - action
        - pwm value for each wing
     */
    pub front_left: u8,
    pub front_right: u8,
    pub back_left: u8,
    pub back_right: u8,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MsgFromEsp {
    /*
    - state or imu readings
     */
    pub acc_x: f64,
    pub acc_y: f64,
    pub acc_z: f64,
}
