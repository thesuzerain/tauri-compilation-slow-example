//! API for interacting with Theseus
pub mod jre;


pub mod data {
    pub use crate::state::{
        DirectoryInfo, Hooks,  MemorySettings, 
          Settings, Theme, WindowSize,
    };
}

pub mod prelude {
    pub use crate::{
        data::*,
        jre, 
        state::JavaGlobals,
        util::jre::JavaVersion,
        State,
    };
}
