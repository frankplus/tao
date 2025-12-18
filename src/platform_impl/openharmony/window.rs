use crate::{
    dpi::{PhysicalPosition, PhysicalSize, Position, Size},
    error,
    monitor::MonitorHandle as RootMonitorHandle,
    window::{CursorIcon, Fullscreen, Icon, Theme, UserAttentionType, WindowAttributes, WindowSizeConstraints},
};
use std::collections::VecDeque;
use super::MonitorHandle;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WindowId;

impl WindowId {
    pub const fn dummy() -> Self {
        WindowId
    }
}

pub struct Window;

#[derive(Clone, Default)]
pub struct PlatformSpecificWindowBuilderAttributes;

impl Window {
    pub fn new<T>(
        _el: &crate::platform_impl::EventLoopWindowTarget<T>,
        _attrs: WindowAttributes,
        _pl_attrs: PlatformSpecificWindowBuilderAttributes,
    ) -> Result<Self, error::OsError> {
        Ok(Window)
    }

    pub fn id(&self) -> WindowId {
        WindowId
    }

    pub fn scale_factor(&self) -> f64 { 1.0 }
    pub fn request_redraw(&self) {}
    pub fn inner_position(&self) -> Result<PhysicalPosition<i32>, error::NotSupportedError> { Err(error::NotSupportedError::new()) }
    pub fn outer_position(&self) -> Result<PhysicalPosition<i32>, error::NotSupportedError> { Err(error::NotSupportedError::new()) }
    pub fn set_outer_position(&self, _position: Position) {}
    pub fn inner_size(&self) -> PhysicalSize<u32> { PhysicalSize::new(800, 600) }
    pub fn set_inner_size(&self, _size: Size) {}
    pub fn outer_size(&self) -> PhysicalSize<u32> { PhysicalSize::new(800, 600) }
    pub fn set_min_inner_size(&self, _dimensions: Option<Size>) {}
    pub fn set_max_inner_size(&self, _dimensions: Option<Size>) {}
    pub fn set_inner_size_constraints(&self, _constraints: WindowSizeConstraints) {}
    pub fn set_title(&self, _title: &str) {}
    pub fn title(&self) -> String { "OpenHarmony Window".to_string() }
    pub fn set_visible(&self, _visible: bool) {}
    pub fn set_focus(&self) {}
    pub fn set_focusable(&self, _focusable: bool) {}
    pub fn is_focused(&self) -> bool { true }
    pub fn is_always_on_top(&self) -> bool { false }
    pub fn set_resizable(&self, _resizable: bool) {}
    pub fn set_minimizable(&self, _minimizable: bool) {}
    pub fn set_maximizable(&self, _maximizable: bool) {}
    pub fn set_closable(&self, _closable: bool) {}
    pub fn set_minimized(&self, _minimized: bool) {}
    pub fn set_maximized(&self, _maximized: bool) {}
    pub fn is_maximized(&self) -> bool { false }
    pub fn is_minimized(&self) -> bool { false }
    pub fn is_visible(&self) -> bool { true }
    pub fn is_resizable(&self) -> bool { false }
    pub fn is_minimizable(&self) -> bool { false }
    pub fn is_maximizable(&self) -> bool { false }
    pub fn is_closable(&self) -> bool { false }
    pub fn is_decorated(&self) -> bool { false }
    pub fn set_fullscreen(&self, _monitor: Option<Fullscreen>) {}
    pub fn fullscreen(&self) -> Option<Fullscreen> { None }
    pub fn set_decorations(&self, _decorations: bool) {}
    pub fn set_always_on_bottom(&self, _always_on_bottom: bool) {}
    pub fn set_always_on_top(&self, _always_on_top: bool) {}
    pub fn set_window_icon(&self, _window_icon: Option<Icon>) {}
    pub fn set_ime_position(&self, _position: Position) {}
    pub fn request_user_attention(&self, _request_type: Option<UserAttentionType>) {}
    pub fn set_cursor_icon(&self, _cursor: CursorIcon) {}
    pub fn set_cursor_position(&self, _position: Position) -> Result<(), error::ExternalError> { Err(error::ExternalError::NotSupported(error::NotSupportedError::new())) }
    pub fn set_cursor_grab(&self, _grab: bool) -> Result<(), error::ExternalError> { Err(error::ExternalError::NotSupported(error::NotSupportedError::new())) }
    pub fn set_cursor_visible(&self, _visible: bool) {}
    pub fn drag_window(&self) -> Result<(), error::ExternalError> { Err(error::ExternalError::NotSupported(error::NotSupportedError::new())) }
    pub fn drag_resize_window(&self, _direction: crate::window::ResizeDirection) -> Result<(), error::ExternalError> { Err(error::ExternalError::NotSupported(error::NotSupportedError::new())) }
    pub fn set_background_color(&self, _color: Option<crate::window::RGBA>) {}
    pub fn set_ignore_cursor_events(&self, _ignore: bool) -> Result<(), error::ExternalError> { Err(error::ExternalError::NotSupported(error::NotSupportedError::new())) }
    pub fn theme(&self) -> Theme { Theme::Light }
    
    // Monitors
    pub fn primary_monitor(&self) -> Option<crate::monitor::MonitorHandle> { None }
    pub fn available_monitors(&self) -> VecDeque<MonitorHandle> { VecDeque::new() }
    pub fn current_monitor(&self) -> Option<crate::monitor::MonitorHandle> { None }
    pub fn monitor_from_point(&self, _x: f64, _y: f64) -> Option<crate::monitor::MonitorHandle> { None }
    
    // Raw Window Handle (Important for Wry!)
    // We can't implement this properly without RawWindowHandle version features check, so I'll leave it as Todo or stub.
    // Tao uses cfg features for rwh.
    
    pub fn set_progress_bar(&self, _progress: crate::window::ProgressBarState) {}
    pub fn set_theme(&self, _theme: Option<Theme>) {}
    pub fn set_visible_on_all_workspaces(&self, _visible: bool) {}
    pub fn cursor_position(&self) -> Result<PhysicalPosition<f64>, error::ExternalError> { Err(error::ExternalError::NotSupported(error::NotSupportedError::new())) }
    
    pub fn raw_window_handle_rwh_06(&self) -> Result<rwh_06::RawWindowHandle, rwh_06::HandleError> {
        Err(rwh_06::HandleError::NotSupported)
    }
    
    pub fn raw_display_handle_rwh_06(&self) -> Result<rwh_06::RawDisplayHandle, rwh_06::HandleError> {
        Err(rwh_06::HandleError::NotSupported)
    }
}
