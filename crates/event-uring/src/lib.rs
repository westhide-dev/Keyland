#![feature(proc_macro_hygiene)]

use kmacro::vis_make;

#[cfg_attr(feature = "protocol", vis_make(pub))]
mod protocol;

pub mod event_uring;
pub mod events;
pub mod handler;
pub mod ident;
pub mod platform;
