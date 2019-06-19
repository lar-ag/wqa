//! `wqms` is a fast and eventually full-featured framework for Rust programming language
//! intended to provide any possible information about the system it is running.
//!
//! ## Compatibility
//!
//! At the moment `wqms` is in **MVP** phase, which means that only the big triple
//! (Linux, MacOS and Windows) are supported.
//! You may want to check out [GitHub projects](https://github.com/lar-ag/wqms-core/projects)
//! for more information about cross-platform support.

#![allow(stable_features)] // Used for `future_api` feature
#![forbid(unused)]
#![forbid(bare_trait_objects)]
#![forbid(missing_docs)]


/// MIO  multi-io
pub mod mio{
    pub use wqms_mio::*;
}
// Airflow monitoring
// pub mod airflow {
    // pub use wqms_airflow::*;
// }

// Disk information
// pub mod disk {
    // pub use wqms_disk::*;
// }

// Host information
// pub mod host {
    // pub use wqms_host::*;
// }

// Memory information
// pub mod memory {
    // pub use wqms_memory::*;
// }




// Network information
// pub mod net {
//    pub use wqms_net::*;
// }

// pub use wqms_common::{Error, Result};
