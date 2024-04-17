#[cfg(feature = "platform_linux")]
pub mod linux;

#[cfg(feature = "platform_macos")]
pub mod macos;

#[cfg(feature = "platform_webos")]
pub mod webos;

#[cfg_attr(feature = "platform_linux", path = "linux/mod.rs")]
#[cfg_attr(feature = "platform_macos", path = "macos/mod.rs")]
#[cfg_attr(feature = "platform_webos", path = "webos/mod.rs")]
pub mod __implement__;

pub trait Platform {}
