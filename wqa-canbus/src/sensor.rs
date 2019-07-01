use crate::error::MioError as Error;
use futures::prelude::*;
// use std::future::Future;
// use random::Source;



pub type UartReader = Box<dyn Future<Output=Result<Vec<u8>,Error>>>;
// use future::mpsc;
/// Edinburgh
pub struct NDirSensor{
    pub fsr: f64,
    fut: Box<dyn Future<Output=Result<Vec<u8>,Error>>>,
}


impl NDirSensor {
    pub fn new(fut: Box<dyn Future<Output=Result<Vec<u8>,Error>>>) -> NDirSensor {
        NDirSensor{
            fsr:0.0,
            fut: fut,
        }
    }
}

pub struct Sensor {
    pub fsr: f64,
}




// pub async fn read_ndir1uSei11Bereit


// pub async fn random_value(uart : u8) -> Result<f32,MioError>{
    // let rvalue = 0.0023;
    //source.read::<f32>();
    // Ok(rvalue)
// }
//
// pub async fn edinburg_decoder() -> Result<f32,MioError> {
    // Ok(0.003)
// }


// pub async fn is_edinburg(uart: u8)  -> Result<bool, MioError> {
    // Ok(false)
// }


// pub async fn ndir1_valur(entcoder: impl Future<Output = Result<f64,MioError>>) -> Result<NDir,MioError>{
//    let fsr =  entcoder.await?;
    // Ok(NDir::new(fsr))
// }
// pub async fn ndir2_valur(entcoder: impl Future<Output = Result<f64,MioError>>) -> Result<NDir,MioError>{
    // let val = entcoder.await?;
    // Ok(NDir::new(val))
// }

// pub async


// pub async fn search_sensor() -> impl Future<Output = Result<f32,MioError>> {

    // let sensor = random_value();
    // Ok(sensor)
// }

