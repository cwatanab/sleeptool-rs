#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(clippy::derivable_impls)]
#![allow(clippy::ineffective_open_options)]
#![allow(clippy::new_without_default)]
#![allow(clippy::missing_const_for_thread_local)]
#![allow(clippy::redundant_closure)]
#![allow(clippy::needless_borrows_for_generic_args)]
#![allow(clippy::unnecessary_mut_passed)]
#![allow(clippy::clone_on_copy)]

pub mod cli;
pub mod config;
pub mod engine;
pub mod error;
pub mod logging;
pub mod monitors;
pub mod platform;
pub mod platform_win32;
pub mod sleep;
pub mod state;
pub mod settings_gui;
pub mod tray;

pub mod tracing {
    pub use crate::{debug, error, info, warn};
}
