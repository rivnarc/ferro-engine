use glutin::config::Config;
use glutin::context::{ContextAttributesBuilder, PossiblyCurrentContext};
use glutin::display::{Display, GetGlDisplay};
use glutin::prelude::*;
use glutin::surface::{Surface, SurfaceAttributesBuilder, WindowSurface};
use raw_window_handle::HasWindowHandle;
use std::num::NonZeroU32;
use winit::window::Window;

pub struct GraphicsContext {
    pub gl: glow::Context,
    pub surface: Surface<WindowSurface>,
    pub context: PossiblyCurrentContext,
    #[allow(dead_code)]
    pub gl_config: Config,
}

impl GraphicsContext {
    pub fn new(window: &Window, gl_config: Config) -> Self {
        let gl_display = gl_config.display();
        let raw_handle = window.window_handle().unwrap().as_raw();

        let context_attributes = ContextAttributesBuilder::new().build(Some(raw_handle));
        let not_current = unsafe {
            gl_display
                .create_context(&gl_config, &context_attributes)
                .unwrap()
        };

        let size = window.inner_size();
        let attrs = SurfaceAttributesBuilder::<WindowSurface>::new().build(
            raw_handle,
            NonZeroU32::new(size.width.max(1)).unwrap(),
            NonZeroU32::new(size.height.max(1)).unwrap(),
        );
        let surface = unsafe {
            gl_display
                .create_window_surface(&gl_config, &attrs)
                .unwrap()
        };

        let context = not_current.make_current(&surface).unwrap();

        let gl = unsafe {
            glow::Context::from_loader_function(|s| {
                let cs = std::ffi::CString::new(s).unwrap();
                gl_display.get_proc_address(&cs) as *const _
            })
        };

        Self {
            gl,
            surface,
            context,
            gl_config,
        }
    }

    pub fn resize(&self, width: u32, height: u32) {
        if let (Some(w), Some(h)) = (NonZeroU32::new(width), NonZeroU32::new(height)) {
            self.surface.resize(&self.context, w, h);
        }
    }

    pub fn swap_buffers(&self) {
        self.surface.swap_buffers(&self.context).unwrap();
    }

    #[allow(dead_code)]
    pub fn display(&self) -> Display {
        self.gl_config.display()
    }
}
