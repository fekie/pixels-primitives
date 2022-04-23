use log::error;
use pixels::{Pixels, SurfaceTexture};
use pixels_primitives;
use std::error::Error;
use winit::dpi::LogicalSize;
use winit::event::Event;
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

const WIDTH: i32 = 800;
const HEIGHT: i32 = 800;

fn main() -> Result<(), Box<dyn Error>> {
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Triangles Example")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH as u32, HEIGHT as u32, surface_texture)?
    };

    pixels_primitives::triangle_filled(
        pixels.get_frame(),
        WIDTH,
        100,
        500,
        300,
        50,
        400,
        400,
        &[255, 255, 255, 255],
    );

    pixels_primitives::triangle(
        pixels.get_frame(),
        WIDTH,
        410,
        500,
        700,
        180,
        600,
        430,
        &[255, 0, 0, 255],
    );

    pixels_primitives::triangle(
        pixels.get_frame(),
        WIDTH,
        -100,
        500,
        200,
        750,
        500,
        700,
        &[0, 255, 0, 255],
    );

    pixels_primitives::triangle_filled(
        pixels.get_frame(),
        WIDTH,
        600,
        600,
        200,
        500,
        550,
        700,
        &[0, 0, 255, 255],
    );

    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            if pixels
                .render()
                .map_err(|e| error!("pixels.render() failed: {}", e))
                .is_err()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        // Handle input events
        if input.update(&event) {
            if input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            window.request_redraw();
        }
    });
}
