#![cfg(target_env = "ohos")]

pub use crate::platform_impl::{
    on_window_created, on_window_destroyed, on_window_focus, on_window_blur,
    event_loop,
};
