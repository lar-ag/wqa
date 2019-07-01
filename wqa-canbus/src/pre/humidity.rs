//! Humidity observation
use serde_derive::{Deserialize, Serialize};
use lazy_static::lazy_static;
use std::sync::RwLock;
use tide::{error::ResultExt, response, App, Context, EndpointResult};

lazy_static! {
    pub static ref HUMIDITY: RwLock<Humidity> = {
        RwLock::new(Humidity::new(0))
    };
} 

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Humidity {
    pub value: f32,
    pub broken: bool,
    pub warn: bool,
    pub crit: bool,
    pub max: f32,
}

impl Humidity {
    pub fn new(value: u16) -> Humidity {

        let mut signal =  value as f32 / 4096.0 * 5.0;
        let broken = signal < 0.8 * 4.0 / 5.0;
        signal = (signal - 0.8)  / (3.6 - 0.8);
        Humidity {
            value: signal,
            broken: broken,
            warn:false,
            crit:false,
            max: 60.0,
        }
    }
}





