/// Monitor gear pump normally used for solution sampling.
///
///
use std::pin::Pin;
use std::marker::PhantomPinned;
use std::ptr::NonNull;
use std::fs::DirBuilder;
use serde_derive::{Deserialize, Serialize};
use lpcan::can::{
    can0,
    Message,
};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Sample {
    pub run: bool,
    pub node: u32,
    pub bit: u8,
}

impl Sample {
    pub fn new() -> Sample {
        GearPump { 
            run: false,
            node: 0x18,
            bit: 0x1,
           }
    }
    pub fn start(&mut self) {

        self.run = true;
    }
    pub fn stop(&mut self) {
        self.run = false;
    }
}


pub fn make_sample()