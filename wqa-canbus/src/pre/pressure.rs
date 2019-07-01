/// Pressure monitoring.

use serde_derive::{Deserialize, Serialize};
use lazy_static::lazy_static;
use std::sync::RwLock;
use super::Range;
lazy_static! {
    pub static ref PRESSURE: RwLock<Pressure> = {
        RwLock::new(Pressure::new(0))
    };
} 



/// Presure sensor 
/// Model `presurei877`
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Pressure {
    pub fsr: f32,
    pub broken: bool,
}

impl Pressure {
    pub fn new(value: u16) -> Pressure {
        let mut signal =  value as f32 / 4096.0 * 5.0;
        let broken = signal < 1.0 * 4.0 / 5.0;
        signal = (signal - 1.0)  / (5.0 - 1.0);
        Pressure{
            fsr: signal,
            broken: broken,
        }
    }
}
