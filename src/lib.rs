pub mod math;

#[warn(missing_docs)]
pub fn line(buffer: &mut [u8]) {
    unimplemented!()
}

#[warn(missing_docs)]
pub fn triangle(buffer: &mut [u8]) {
    unimplemented!()
}

#[warn(missing_docs)]
pub fn triangle_filled(buffer: &mut [u8]) {
    unimplemented!()
}

// TODO: this function can be optimized by removing the square root used in the distance function

/// Draws a circle to a frame of pixels used in the [pixels](https://docs.rs/pixels/latest/pixels/) crate.
///
/// # Example
///
/// ```no_run
/// use pixels::{Pixels, SurfaceTexture};
/// use winit::dpi::LogicalSize;
/// use winit::event_loop::{EventLoop};
/// use winit_input_helper::WinitInputHelper;
/// use std::error::Error;
/// use winit::window::WindowBuilder;
///
/// const WIDTH: u32 = 800;
/// const HEIGHT: u32 = 800;
///
/// fn main() -> Result<(), Box<dyn Error>> {
///     let event_loop = EventLoop::new();
///     let mut input = WinitInputHelper::new();
///     let window = {
///     let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
///     WindowBuilder::new()
///         .with_title("Circle Example")
///         .with_inner_size(size)
///         .with_min_inner_size(size)
///         .build(&event_loop)
///         .unwrap()
///     };
///
///     let mut pixels = {
///         let window_size = window.inner_size();
///         let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
///         Pixels::new(WIDTH, HEIGHT, surface_texture)?
///     };
///
///     pixels_primitives::circle(
///         pixels.get_frame(),
///         WIDTH,
///         200.0,
///         200.0,
///         50.0,
///         1.5,
///         &[255, 255, 255, 255],
///     );
///
///     Ok(())
/// }

/// ```
#[warn(missing_docs)]
pub fn circle(
    frame: &mut [u8],
    width: u32,
    center_x: f64,
    center_y: f64,
    radius: f64,
    outline_width: f64,
    rgba: &[u8; 4],
) {
    let rough_minimum_y = (center_y - radius) as u32;
    let rough_minimum_x = (center_x - radius) as u32;
    let rough_maximum_y = (center_y + radius) as u32;
    let rough_maximum_x = (center_x + radius) as u32;

    for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
        let x = i as u32 % width;
        let y = i as u32 / width;

        // dont calculate distance if its not within the bounding box
        if (x < rough_minimum_x)
            || (x > rough_maximum_x)
            || (y < rough_minimum_y)
            || (y > rough_maximum_y)
        {
            continue;
        }

        let distance = math::distance(center_x, center_y, x as f64, y as f64);
        if (distance <= radius) && (distance >= (radius - outline_width)) {
            pixel.copy_from_slice(rgba);
        }
    }
}

#[warn(missing_docs)]
pub fn circle_filled(buffer: &mut [u8]) {
    unimplemented!()
}

#[warn(missing_docs)]
pub fn square(buffer: &mut [u8]) {
    unimplemented!()
}

#[warn(missing_docs)]
pub fn square_filled(buffer: &mut [u8]) {
    unimplemented!()
}

#[warn(missing_docs)]
pub fn rect(buffer: &mut [u8]) {
    unimplemented!()
}

#[warn(missing_docs)]
pub fn rect_filled(buffer: &mut [u8]) {
    unimplemented!()
}
