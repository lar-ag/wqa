/// Monitor gear pump normally used for solution sampling.
///
use serde_derive::{Deserialize, Serialize};
// use lpcan::can::{
// can0,
// Message,
// };

pub enum State {
    Runned,
    Stop,
}



// use lazy_static::lazy_static;

// lazy_static! {
    // static ref GP: RwLock<GearPump> = RwLock::new(GearPump::new());
// }

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct GearPump {
    pub run: bool,
}

impl GearPump {
    pub fn new() -> GearPump {
        GearPump {
            run: false,
        }
    }
    pub fn start(&mut self) {
        self.run = true;
    }
    pub fn stop(&mut self) {
        self.run = false;
    }
}

pub struct Pumps {
    gp1: GearPump,
    gp2: GearPump,
    gp3: GearPump,
    gp4: GearPump,
    gp5: GearPump,
    gp6: GearPump,
}

// pub async fn gear_pumms_state() {}

// pub async fn
//
