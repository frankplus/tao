use crate::dpi::{PhysicalPosition, PhysicalSize};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MonitorHandle;

impl MonitorHandle {
    pub fn name(&self) -> Option<String> { None }
    pub fn size(&self) -> PhysicalSize<u32> { PhysicalSize::new(800, 600) }
    pub fn position(&self) -> PhysicalPosition<i32> { PhysicalPosition::new(0, 0) }
    pub fn scale_factor(&self) -> f64 { 1.0 }
    pub fn video_modes(&self) -> impl Iterator<Item = crate::monitor::VideoMode> { std::iter::empty() }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct VideoMode;

impl VideoMode {
    pub fn size(&self) -> PhysicalSize<u32> { PhysicalSize::new(800, 600) }
    pub fn bit_depth(&self) -> u16 { 32 }
    pub fn refresh_rate(&self) -> u16 { 60 }
    pub fn monitor(&self) -> crate::monitor::MonitorHandle { 
        crate::monitor::MonitorHandle { inner: MonitorHandle }
    }
}
