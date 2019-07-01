//! Sensor
//! NDir1,NDir2 ,Zirox NO
//!
//!
pub mod airflow;
pub mod humidity;
pub mod pressure;

use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)] 
pub struct Range {
    pub min:f64, 
    pub max:f64, 
} 

impl Range {
    pub fn new(min:f64,max:f64) -> Self {
        Self {
            min: min,
            max: max,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Scale {
    pub min: f64,
    pub max: f64,
}

impl Scale {
    pub fn new(min:f64,max:f64) -> Self {
        Self {
            min: min,
            max: max,
        }
    }
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)] 
pub enum Plug {
    None,
    NDIR1,
    NDIR2,
    NO,
    Ziroc,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)] 
pub struct Sensor {
    pub signal: Vec<f32>,
    pub period: u64,
    pub plug: Plug,
}


impl Sensor {
    pub fn new(period:u64) -> Self {
        Self {
            signal: Vec::new(),
            period:period,
            plug:Plug::None,
        }
    }
}

pub struct Sensors {

}


pub struct Monitoring {
    
}