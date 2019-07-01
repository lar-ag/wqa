#![feature(async_await)]

//! This crate is an placeholder for a future development.
mod error;
mod dbus;
mod rpc;
mod pump;
mod valve;
mod mio;
mod pcan;
mod airflow;
mod humidity;
mod pressure;
mod control;
mod sensor;
mod temperatur;
// mod analog;
// pub use analog::AIN;


// pub use mio::*;
// pub use pcan::*;
// pub use simulation as io;



mod simulation;
// mod pump;
// pub use sensor::*;
// pub use control::*;

pub use self::error::MioError;
pub use self::mio::*;
pub use self::dbus::*;
pub use self::airflow::*;
pub use self::humidity::*;
pub use self::pressure::*;
pub use self::temperatur::*;
pub use self::sensor::*;




// #[cfg(test)]
// mod tests {

    // #[test]
    // fn airflow_value() {
    // }
// }
//

//end
