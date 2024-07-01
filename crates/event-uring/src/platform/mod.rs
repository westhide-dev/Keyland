#[cfg(feature = "platform_linux")]
pub mod linux;

#[cfg(feature = "platform_macos")]
pub mod macos;

#[cfg(feature = "platform_webos")]
pub mod webos;

pub trait PlatformΞ {}
