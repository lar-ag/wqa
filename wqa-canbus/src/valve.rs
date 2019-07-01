//! Valve

pub enum State {
    Close,
    Open,
}

pub struct Valve
{
    state: State,
}
/// Single digital push-pull output pin
///
impl Valve
{
    /// Valve open
    ///
    /// *NOTE* the actual electrical state of the pin may not actually be low, e.g. due to external
    /// electrical sources
    pub fn set_open(&mut self) {
        self.state = State::Open;
    }
    /// Valve close
    ///
    /// *NOTE* the actual electrical state of the pin may not actually be high, e.g. due to external
    /// electrical sources
    pub fn set_close(&mut self) {
        self.state = State::Open;
    }
    // /// Is the valve is open?
    // ///
    // /// *NOTE* this does *not* read the electrical state of the pin
    // pub fn is_open(&self) -> bool {
    //     match self.state {
    //         State::Open => true,
    //         State::Close => false,
    //     }
    // }
    // /// Is the valve close?
    // ///
    // /// *NOTE* this does *not* read the electrical state of the pin
    // pub fn is_close(&self) -> bool {
    //     match self.state {
    //         State::Close => true,
    //         State::Open => false,
    //     }
    // }

    // /// Valve that can be toggled
    // /// See [toggleable](toggleable) to use a software implementation if
    // pub fn toggle(&mut self) {
    //     if self.is_open() {
    //         self.set_close();
    //     } else {
    //         self.set_open();
    //     }
    // }
}




