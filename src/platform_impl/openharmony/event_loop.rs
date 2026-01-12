use std::sync::{Arc, Mutex, OnceLock};
use std::sync::mpsc::{self, Receiver, Sender};
use std::collections::VecDeque;
use std::time::Instant;

use crate::{
    event::{Event, WindowEvent, Touch, TouchPhase, StartCause, DeviceId as RootDeviceId},
    event_loop::{ControlFlow, EventLoopWindowTarget as RootELWT, EventLoopClosed},
    monitor::MonitorHandle as RootMonitorHandle,
    window::WindowId as RootWindowId,
    platform_impl::platform::{
        DeviceIdImpl, WindowId, MonitorHandle,
    },
};

#[derive(Debug)]
pub enum InternalEvent {
    OnWindowCreated,
    OnWindowDestroyed,
    OnWindowFocus,
    OnWindowBlur,
    TouchEvent { x: f32, y: f32, phase: i32, id: i32 },
    Wakeup,
}

struct GlobalChannel {
    tx: Sender<InternalEvent>,
    rx: Mutex<Receiver<InternalEvent>>,
}

static CHANNEL: OnceLock<GlobalChannel> = OnceLock::new();

fn ensure_channel() {
    CHANNEL.get_or_init(|| {
        let (tx, rx) = mpsc::channel();
        GlobalChannel { tx, rx: Mutex::new(rx) }
    });
}

// Public API for the NAPI glue to push events
pub fn publish_event(event: InternalEvent) {
    ensure_channel();
    let _ = CHANNEL.get().unwrap().tx.send(event);
}

// Exported functions for NAPI
pub fn on_window_created() {
    publish_event(InternalEvent::OnWindowCreated);
}

pub fn on_window_destroyed() {
    publish_event(InternalEvent::OnWindowDestroyed);
}

pub fn on_window_focus() {
    publish_event(InternalEvent::OnWindowFocus);
}

pub fn on_window_blur() {
    publish_event(InternalEvent::OnWindowBlur);
}

pub struct EventLoop<T: 'static> {
    window_target: RootELWT<T>,
    user_event_tx: Sender<T>,
    user_event_rx: Receiver<T>,
}

impl<T: 'static> EventLoop<T> {
    pub(crate) fn new(_: &crate::platform_impl::PlatformSpecificEventLoopAttributes) -> Self {
        ensure_channel();
        let (tx, rx) = mpsc::channel();
        EventLoop {
            window_target: RootELWT {
                p: EventLoopWindowTarget { _marker: std::marker::PhantomData },
                _marker: std::marker::PhantomData,
            },
            user_event_tx: tx,
            user_event_rx: rx,
        }
    }

    pub fn run<F>(self, mut event_handler: F) -> !
    where
        F: 'static + FnMut(Event<'_, T>, &RootELWT<T>, &mut ControlFlow),
    {
        let rx = CHANNEL.get().unwrap().rx.lock().unwrap();
        let mut control_flow = ControlFlow::default();

        event_handler(Event::NewEvents(StartCause::Init), &self.window_target, &mut control_flow);

        loop {
            // 1. Process System Events
            if let Ok(event) = rx.recv() {
                 match event {
                     InternalEvent::OnWindowCreated => {
                         // Similar to Android's Resumed/FocusGained
                         event_handler(Event::Resumed, &self.window_target, &mut control_flow);
                     }
                     InternalEvent::OnWindowDestroyed => {
                         // Similar to Android's Suspended
                         event_handler(Event::Suspended, &self.window_target, &mut control_flow);
                     }
                     InternalEvent::OnWindowFocus => {
                        event_handler(Event::WindowEvent {
                            window_id: RootWindowId(WindowId),
                            event: WindowEvent::Focused(true),
                        }, &self.window_target, &mut control_flow);
                     }
                     InternalEvent::OnWindowBlur => {
                        event_handler(Event::WindowEvent {
                            window_id: RootWindowId(WindowId),
                            event: WindowEvent::Focused(false),
                        }, &self.window_target, &mut control_flow);
                     }
                     InternalEvent::TouchEvent { x, y, phase, id } => {
                         let phase = match phase {
                             0 => TouchPhase::Started, // DOWN
                             1 => TouchPhase::Ended,   // UP
                             2 => TouchPhase::Moved,   // MOVE
                             3 => TouchPhase::Cancelled, // CANCEL
                             _ => TouchPhase::Moved,
                         };
                         
                         let touch = Touch {
                             device_id: RootDeviceId(DeviceIdImpl),
                             phase,
                             location: crate::dpi::PhysicalPosition::new(x as f64, y as f64),
                             force: None,
                             id: id as u64,
                         };
                         
                         event_handler(Event::WindowEvent {
                             window_id: RootWindowId(WindowId),
                             event: WindowEvent::Touch(touch),
                         }, &self.window_target, &mut control_flow);
                     }
                     InternalEvent::Wakeup => {}
                 }
            }

            // 2. Process User Events
            while let Ok(event) = self.user_event_rx.try_recv() {
                event_handler(Event::UserEvent(event), &self.window_target, &mut control_flow);
            }
            
            // 3. MainEventsCleared
            event_handler(Event::MainEventsCleared, &self.window_target, &mut control_flow);
            
            // 4. Handle ControlFlow
            match control_flow {
                ControlFlow::Exit | ControlFlow::ExitWithCode(_) => break,
                _ => {}
            }
        }
        
        std::process::exit(0)
    }

    pub fn window_target(&self) -> &RootELWT<T> {
        &self.window_target
    }

    pub fn create_proxy(&self) -> EventLoopProxy<T> {
        EventLoopProxy { tx: self.user_event_tx.clone() }
    }
}

pub struct EventLoopProxy<T> {
    tx: Sender<T>,
}

impl<T> EventLoopProxy<T> {
    pub fn send_event(&self, event: T) -> Result<(), EventLoopClosed<T>> {
        let res = self.tx.send(event).map_err(|e| EventLoopClosed(e.0));
        publish_event(InternalEvent::Wakeup);
        res
    }
}

impl<T> Clone for EventLoopProxy<T> {
    fn clone(&self) -> Self {
        Self { tx: self.tx.clone() }
    }
}

// Stub for EventLoopExtRunReturn
impl<T> crate::platform::run_return::EventLoopExtRunReturn for EventLoop<T> {
    type UserEvent = T;
    fn run_return<F>(&mut self, _event_handler: F) -> i32
    where
        F: FnMut(crate::event::Event<'_, T>, &RootELWT<T>, &mut ControlFlow),
    {
        // Not implemented for now, as OpenHarmony app lifecycle is managed by system
        0
    }
}

#[derive(Clone)]
pub struct EventLoopWindowTarget<T> {
    _marker: std::marker::PhantomData<T>,
}

impl<T> EventLoopWindowTarget<T> {
    pub fn primary_monitor(&self) -> Option<RootMonitorHandle> {
        None 
    }
    
    pub fn available_monitors(&self) -> VecDeque<MonitorHandle> {
        VecDeque::new()
    }

    pub fn monitor_from_point(&self, _x: f64, _y: f64) -> Option<MonitorHandle> { None }
    pub fn cursor_position(&self) -> Result<crate::dpi::PhysicalPosition<f64>, crate::error::ExternalError> { Err(crate::error::ExternalError::NotSupported(crate::error::NotSupportedError::new())) }
    pub fn set_progress_bar(&self, _progress: crate::window::ProgressBarState) {}
    pub fn set_theme(&self, _theme: Option<crate::window::Theme>) {}
    pub fn raw_display_handle_rwh_06(&self) -> Result<rwh_06::RawDisplayHandle, rwh_06::HandleError> { Err(rwh_06::HandleError::NotSupported) }
    }

