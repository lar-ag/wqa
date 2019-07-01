use serde_derive::{Deserialize, Serialize};
use std::time::Duration;
use super::Adjustement;






#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Channel {
    pub id:     u64,
    pub name:   String,
    pub unit:   String,
    pub min:    f64,
    pub max:    f64,
    pub adjust:  Vec<Adjustment>,
}

/// Channel
impl Channel {
    pub fn new() -> Channel {
        Channel {
            stream: 0,
            on : false,
            name: "CH".to_owned(),
            unit:"mg/l".to_owned(),
            min: 0.0,
            max: 0.0,
            adjust: Adjustment::Lineal(),
        }
    }
}
