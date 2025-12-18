#![cfg(target_env = "ohos")]

pub mod event_loop;
pub mod window;
pub mod monitor;
pub mod clipboard;
pub mod icon;

pub use event_loop::{EventLoop, EventLoopProxy, EventLoopWindowTarget, register_xcomponent};
pub use window::{PlatformSpecificWindowBuilderAttributes, Window, WindowId};
pub use monitor::{MonitorHandle, VideoMode};

pub type DeviceId = DeviceIdImpl;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct KeyEventExtra {
    pub text_with_all_modifiers: Option<String>,
    pub key_without_modifiers: crate::keyboard::Key<'static>,
}

pub fn keycode_from_scancode(_: u32) -> crate::keyboard::KeyCode {
    crate::keyboard::KeyCode::Unidentified(crate::keyboard::NativeKeyCode::Unidentified)
}

pub fn keycode_to_scancode(_: crate::keyboard::KeyCode) -> Option<u32> {
    None
}

#[derive(Debug, Clone, Default)]
pub struct PlatformSpecificEventLoopAttributes;

use crate::event::DeviceId as RootDeviceId;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DeviceIdImpl;

impl DeviceIdImpl {
    pub const fn dummy() -> Self {
        DeviceIdImpl
    }
}

#[derive(Debug, Clone)]
pub struct OsError;

impl std::fmt::Display for OsError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "OpenHarmony OS Error")
  }
}

pub(crate) use crate::icon::NoIcon as PlatformIcon;
