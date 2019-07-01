// use serde_derive::{Deserialize, Serialize};

mod solution;
mod adjustment;
mod stream;
mod measurement;
mod calibration;
mod signal;
mod indicators;
mod statistic;


pub use self::calibration::*;
pub use self::measurement::*;
pub use self::stream::*;

pub struct Prepare;
pub struct Analysis {
    pub start: u64,
}

pub use solution::{Concentration,Solution};
pub use signal::{Signal,SignalIter};
pub use measurement::{Measurement,MeasurementIter};
pub use calibration::{Adjustment,Calibration,Linear};



