use crate::components::*;
use crate::store::measurement;
use std::time::Duration;




pub type XAxis   = axis::Axis;
pub type YAxis   = axis::Axis;
pub type ZAxis   = axis::Axis;
pub type Furnace = furnace::Furnace;
pub type Tic     = ticport::Ticport;
pub type Air     = airflow::Airflow;
pub type Sample  = pump::GearPump;
pub type NDir1   = sensor::Sensor;
pub type NDir2   = sensor::Sensor;
pub type NO      = sensor::Sensor;
pub type Zirox   = sensor::Sensor;
pub type Stirrer = stirrer::Stirrer;
pub type Meas    = measurement::Measurement;

pub type TcDelay  = Duration;
pub type TicDelay = Duration;
pub type Timer    = Duration;

pub struct GP1;
pub struct GP2;
pub struct GP3;
pub struct GP4;
pub struct GP5;
pub struct GP6;


pub struct Ultra {
    pub xpos:       u32,
    pub ypos:       u32,
    pub zpos:       u32,
    pub pumps:      [bool,7],
    
    pub ndir:       Vec<f64>
    pub inje:       ZAxis, 
    pub furnace:    Furnace,
    pub tic:        Tic,
    pub air:        Air,
    pub ndir1:      NDir1,
    pub ndir2:      NDir2,
    pub no:         NO,
    pub zirox:      Zirox,
    pub stirrer:    Stirrer,
    pub timer1:     Timer,
}




impl Ultra {
    pub fn new() -> Ultra {
        let sample =   Sample::default(); 
        let x =        XAxis::default(); 
        let y =        YAxis::default(); 
        let z =        ZAxis::default(); 
        let furnace =  Furnace::default();
        let tic =      Tic::default();
        let air =      Air::default();
        let ndir1   =  NDir1::default();
        let ndir2   =  NDir2::default();
        let no      =  NO::default();
        let zirox   =  Zirox::default();
        let stirrer =  Stirrer::default();
        let timer1  =  Timer::from_secs(0);
        Self {
            sample,
            x,y,z,
            furnace,
            tic,
            air,
            ndir1,
            ndir2,
            no,
            zirox,
            stirrer,
            timer1
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn measurement_tc_works() {
        let mut ultra = Ultra::new();
        assert_eq!(2 + 2, 4);
    }
}
