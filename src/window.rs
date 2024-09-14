use femtovg::{renderer::OpenGl, Canvas};
use glutin::{
    config::ConfigTemplateBuilder,
    context::ContextAttributesBuilder,
    context::PossiblyCurrentContext,
    display::Display,
    display::GetGlDisplay,
    prelude::*,
    surface::{Surface, SurfaceAttributesBuilder, WindowSurface},
};
use glutin_winit::DisplayBuilder;
use raw_window_handle::HasWindowHandle;
use std::num::NonZeroU32;
use winit::event_loop::EventLoop;
use winit::{dpi::PhysicalSize, window::Window};

pub(crate) fn create_window<T>(
    event_loop: &EventLoop<T>,
) -> (
    PossiblyCurrentContext,
    Display,
    Window,
    Surface<WindowSurface>,
) {
    let attributes = Window::default_attributes()
        .with_title("Tetris")
        .with_inner_size(PhysicalSize::new(1000, 600));
    let template = ConfigTemplateBuilder::new().with_alpha_size(8);
    let display_builder = DisplayBuilder::new();
    let display_builder = display_builder.with_window_attributes(Some(attributes));
    let (window, gl_config) = display_builder
        .build(event_loop, template, |mut configs| configs.next().unwrap())
        .unwrap();
    let window = window.unwrap();
    let gl_display = gl_config.display();
    let raw_handle = window.window_handle().unwrap().into();
    let context_attributes = ContextAttributesBuilder::new().build(Some(raw_handle));
    let mut not_current_gl_context = Some(unsafe {
        gl_display
            .create_context(&gl_config, &context_attributes)
            .unwrap()
    });
    let attrs = SurfaceAttributesBuilder::<WindowSurface>::new().build(
        raw_handle,
        NonZeroU32::new(1000).unwrap(),
        NonZeroU32::new(600).unwrap(),
    );
    let surface = unsafe {
        gl_config
            .display()
            .create_window_surface(&gl_config, &attrs)
            .unwrap()
    };
    (
        not_current_gl_context
            .take()
            .unwrap()
            .make_current(&surface)
            .unwrap(),
        gl_display,
        window,
        surface,
    )
}
pub(crate) fn create_canvas(gl_display: Display, window: &Window) -> Canvas<OpenGl> {
    let renderer =
        unsafe { OpenGl::new_from_function_cstr(|s| gl_display.get_proc_address(s) as *const _) }
            .expect("Could not create renderer");

    let mut canvas = Canvas::new(renderer).expect("Could not create canvas");
    canvas.set_size(1000, 600, window.scale_factor() as f32);
    canvas
}
