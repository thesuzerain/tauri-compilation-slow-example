//! API for interacting with Theseus
pub mod jre;


pub mod data {
    pub use crate::state::{
        DirectoryInfo, Hooks,  MemorySettings, 
        ModrinthProject, ModrinthTeamMember, ModrinthUser, ModrinthVersion,
         ProjectMetadata, Settings, Theme, WindowSize,
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
