/// Valve I/O
use serde_derive::{Deserialize, Serialize};


pub struct NO;
pub struct NC;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)] 
pub struct Valve {
    open: bool,
}


/// Single digital push-pull output pin
impl Valve {
    pub fn new() ->Self {
        Self {
            open: false
        }
    }
    /// Open valve
    pub fn open(&mut self) {
        self.open = true;
    }

    /// Close valve
    ///
    pub fn close(&mut self) {
        self.open = false;
    }
}

