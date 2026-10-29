use glutin::config::ConfigTemplateBuilder;
use glutin_winit::DisplayBuilder;
use winit::event_loop::ActiveEventLoop;
use winit::window::WindowAttributes;

pub struct WindowState {
    pub window: winit::window::Window,
    pub gl_config: glutin::config::Config,
}

impl WindowState {
    pub fn new(event_loop: &ActiveEventLoop, title: &str) -> Self {
        let window_attributes = WindowAttributes::default().with_title(title);
        let template = ConfigTemplateBuilder::new().with_alpha_size(8);
        let display_builder = DisplayBuilder::new().with_window_attributes(Some(window_attributes));

        let (window, gl_config) = display_builder
            .build(event_loop, template, |mut configs| configs.next().unwrap())
            .unwrap();

        Self {
            window: window.unwrap(),
            gl_config,
        }
    }
}
