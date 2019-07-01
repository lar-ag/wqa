
use crate::Pump;

pub trait Stream {
    pub get_name(&self) -> String;
}