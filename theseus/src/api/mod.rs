//! API for interacting with Theseus
pub mod jre;


pub mod prelude {
    pub use crate::{
        jre, 
        util::jre::JavaVersion,
    };
}
