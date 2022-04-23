//! An easy and simple wrapper for lines and simple shapes for the [pixels](https://docs.rs/pixels/latest/pixels/) crate.

use std::mem;

mod math;

/// Draws a 2d line to a frame of pixels used in the [pixels](https://docs.rs/pixels/latest/pixels/) crate.
///
/// # Example
///
/// ```no_run
/// use pixels::{Pixels, SurfaceTexture};
/// use winit::dpi::LogicalSize;
/// use winit::event_loop::{EventLoop};
/// use std::error::Error;
/// use winit::window::WindowBuilder;
///
/// const WIDTH: i32 = 800;
/// const HEIGHT: i32 = 800;
///
/// fn main() -> Result<(), Box<dyn Error>> {
///     let event_loop = EventLoop::new();
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
///         Pixels::new(WIDTH as u32, HEIGHT as u32, surface_texture)?
///     };
///
///     pixels_primitives::line(
///         pixels.get_frame(),
///         WIDTH,
///         200.0,
///         100.0,
///         700.0,
///         300.0,
///         &[255, 255, 255, 255],
///     );
///
///     // Run your event loop here!
///
///     Ok(())
/// }
///
/// ```
#[warn(missing_docs)]
pub fn line(
    frame: &mut [u8],
    canvas_width: i32,
    starting_x: f64,
    starting_y: f64,
    ending_x: f64,
    ending_y: f64,
    rgba: &[u8; 4],
) {
    let canvas_height = frame.len() as i32 / 4 / canvas_width;

    // Clone our immutable values into mutable values.
    let (mut mx0, mut my0, mut mx1, mut my1) = (
        starting_x as i32,
        starting_y as i32,
        ending_x as i32,
        ending_y as i32,
    );

    // Checks to see if range is bigger than the domain.
    let steep = (mx0 - mx1).abs() < (my0 - my1).abs();

    // If the line is steep, we transpose the line (by swapping our Xs and Ys, we will undo it later).
    if steep {
        mem::swap(&mut mx0, &mut my0);
        mem::swap(&mut mx1, &mut my1);
    };

    // Make it left−to−right.
    if mx0 > mx1 {
        mem::swap(&mut mx0, &mut mx1);
        mem::swap(&mut my0, &mut my1);
    }

    // Error is the distance from the mathematically "correct" line. (because we're displaying in terms of pixels and not precise mathematically terms)
    let dx: i32 = mx1 - mx0;
    let dy: i32 = my1 - my0;
    let error_increment2 = dy.abs() * 2;
    let mut error2: i32 = 0;

    let mut y = my0;
    for x in mx0..mx1 {
        if steep {
            color_position(y, x, canvas_width, canvas_height, frame, rgba);
        } else {
            color_position(x, y, canvas_width, canvas_height, frame, rgba);
        }
        error2 += error_increment2;
        if error2 > dx {
            y += if my1 > my0 { 1 } else { -1 };
            error2 -= dx * 2;
        }
    }
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

/// Draws an outline of a circle to a frame of pixels used in the [pixels](https://docs.rs/pixels/latest/pixels/) crate.
///
/// # Example
///
/// ```no_run
/// use pixels::{Pixels, SurfaceTexture};
/// use winit::dpi::LogicalSize;
/// use winit::event_loop::{EventLoop};
/// use std::error::Error;
/// use winit::window::WindowBuilder;
///
/// const WIDTH: i32 = 800;
/// const HEIGHT: i32 = 800;
///
/// fn main() -> Result<(), Box<dyn Error>> {
///     let event_loop = EventLoop::new();
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
///         Pixels::new(WIDTH as u32, HEIGHT as u32, surface_texture)?
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
///     // Run your event loop here!
///
///     Ok(())
/// }
///
/// ```
#[warn(missing_docs)]
pub fn circle(
    frame: &mut [u8],
    canvas_width: i32,
    center_x: f64,
    center_y: f64,
    radius: f64,
    outline_width: f64,
    rgba: &[u8; 4],
) {
    // Note that rough_maximum_y will not actually be rendered higher than rough_minimum_y, as we are working in the 4th quadrant
    let canvas_height = frame.len() as i32 / 4 / canvas_width;
    let rough_minimum_y = (center_y - radius) as i32;
    let rough_minimum_x = (center_x - radius) as i32;
    let rough_maximum_y = (center_y + radius) as i32;
    let rough_maximum_x = (center_x + radius) as i32;

    for y in rough_minimum_y..=rough_maximum_y {
        for x in rough_minimum_x..=rough_maximum_x {
            let distance = math::distance(center_x, center_y, x as f64, y as f64);
            if (distance <= radius) && (distance >= (radius - outline_width)) {
                color_position(x, y, canvas_width, canvas_height, frame, rgba);
            }
        }
    }
}

// TODO: this function can be optimized by removing the square root used in the distance function

/// Draws a filled circle to a frame of pixels used in the [pixels](https://docs.rs/pixels/latest/pixels/) crate.
///
/// # Example
///
/// ```no_run
/// use pixels::{Pixels, SurfaceTexture};
/// use winit::dpi::LogicalSize;
/// use winit::event_loop::{EventLoop};
/// use std::error::Error;
/// use winit::window::WindowBuilder;
///
/// const WIDTH: i32 = 800;
/// const HEIGHT: i32 = 800;
///
/// fn main() -> Result<(), Box<dyn Error>> {
///     let event_loop = EventLoop::new();
///     let window = {
///     let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
///     WindowBuilder::new()
///         .with_title("Filled Circle Example")
///         .with_inner_size(size)
///         .with_min_inner_size(size)
///         .build(&event_loop)
///         .unwrap()
///     };
///
///     let mut pixels = {
///         let window_size = window.inner_size();
///         let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
///         Pixels::new(WIDTH as u32, HEIGHT as u32, surface_texture)?
///     };
///
///     pixels_primitives::circle_filled(
///         pixels.get_frame(),
///         WIDTH,
///         200.0,
///         200.0,
///         50.0,
///         &[255, 255, 255, 255],
///     );
///
///     // Run your event loop here!
///
///     Ok(())
/// }
///
/// ```
#[warn(missing_docs)]
pub fn circle_filled(
    frame: &mut [u8],
    canvas_width: i32,
    center_x: f64,
    center_y: f64,
    radius: f64,
    rgba: &[u8; 4],
) {
    let canvas_height = frame.len() as i32 / 4 / canvas_width;
    let rough_minimum_y = (center_y - radius) as i32;
    let rough_minimum_x = (center_x - radius) as i32;
    let rough_maximum_y = (center_y + radius) as i32;
    let rough_maximum_x = (center_x + radius) as i32;

    for y in rough_minimum_y..=rough_maximum_y {
        for x in rough_minimum_x..=rough_maximum_x {
            let distance = math::distance(center_x, center_y, x as f64, y as f64);
            if distance <= radius {
                color_position(x, y, canvas_width, canvas_height, frame, rgba);
            }
        }
    }
}

/// Draws an outline of a square to a frame of pixels used in the [pixels](https://docs.rs/pixels/latest/pixels/) crate.
///
/// # Example
///
/// ```no_run
/// use pixels::{Pixels, SurfaceTexture};
/// use winit::dpi::LogicalSize;
/// use winit::event_loop::{EventLoop};
/// use std::error::Error;
/// use winit::window::WindowBuilder;
///
/// const WIDTH: i32 = 800;
/// const HEIGHT: i32 = 800;
///
/// fn main() -> Result<(), Box<dyn Error>> {
///     let event_loop = EventLoop::new();
///     let window = {
///     let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
///     WindowBuilder::new()
///         .with_title("Filled Square Example")
///         .with_inner_size(size)
///         .with_min_inner_size(size)
///         .build(&event_loop)
///         .unwrap()
///     };
///
///     let mut pixels = {
///         let window_size = window.inner_size();
///         let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
///         Pixels::new(WIDTH as u32, HEIGHT as u32, surface_texture)?
///     };
///
///     pixels_primitives::square(
///         pixels.get_frame(),
///         WIDTH,
///         200.0,
///         200.0,
///         100.0,
///         &[255, 255, 255, 255],
///     );
///
///     // Run your event loop here!
///
///     Ok(())
/// }
///
/// ```
#[warn(missing_docs)]
pub fn square(
    frame: &mut [u8],
    canvas_width: i32,
    center_x: f64,
    center_y: f64,
    side_length: f64,
    rgba: &[u8; 4],
) {
    // Note that top_right_y will not actually be rendered on the top right of the square, as we are working in the 4th quadrant
    let bottom_left_x = center_x - (side_length / 2.0);
    let bottom_left_y = center_y - (side_length / 2.0);
    let top_right_x = center_x + (side_length / 2.0);
    let top_right_y = center_y + (side_length / 2.0);

    line(
        frame,
        canvas_width,
        bottom_left_x,
        bottom_left_y,
        bottom_left_x,
        top_right_y,
        rgba,
    );

    line(
        frame,
        canvas_width,
        bottom_left_x,
        bottom_left_y,
        top_right_x,
        bottom_left_y,
        rgba,
    );

    line(
        frame,
        canvas_width,
        top_right_x,
        bottom_left_y,
        top_right_x,
        top_right_y,
        rgba,
    );

    line(
        frame,
        canvas_width,
        top_right_x,
        bottom_left_y,
        top_right_x,
        top_right_y,
        rgba,
    );

    line(
        frame,
        canvas_width,
        top_right_x,
        top_right_y,
        bottom_left_x,
        top_right_y,
        rgba,
    );
}

/// Draws a filled square to a frame of pixels used in the [pixels](https://docs.rs/pixels/latest/pixels/) crate.
///
/// # Example
///
/// ```no_run
/// use pixels::{Pixels, SurfaceTexture};
/// use winit::dpi::LogicalSize;
/// use winit::event_loop::{EventLoop};
/// use std::error::Error;
/// use winit::window::WindowBuilder;
///
/// const WIDTH: i32 = 800;
/// const HEIGHT: i32 = 800;
///
/// fn main() -> Result<(), Box<dyn Error>> {
///     let event_loop = EventLoop::new();
///     let window = {
///     let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
///     WindowBuilder::new()
///         .with_title("Square Example")
///         .with_inner_size(size)
///         .with_min_inner_size(size)
///         .build(&event_loop)
///         .unwrap()
///     };
///
///     let mut pixels = {
///         let window_size = window.inner_size();
///         let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
///         Pixels::new(WIDTH as u32, HEIGHT as u32, surface_texture)?
///     };
///
///     pixels_primitives::square_filled(
///         pixels.get_frame(),
///         WIDTH,
///         200.0,
///         200.0,
///         100.0,
///         &[255, 255, 255, 255],
///     );
///
///     // Run your event loop here!
///
///     Ok(())
/// }
///
/// ```
#[warn(missing_docs)]
pub fn square_filled(
    frame: &mut [u8],
    canvas_width: i32,
    center_x: f64,
    center_y: f64,
    side_length: f64,
    rgba: &[u8; 4],
) {
    // Note that rough_maximum_y will not actually be rendered higher than rough_minimum_y, as we are working in the 4th quadrant
    let rough_minimum_y = (center_y - (side_length / 2.0)) as i32;
    let rough_minimum_x = (center_x - (side_length / 2.0)) as i32;
    let rough_maximum_y = (center_y + (side_length / 2.0)) as i32;
    let rough_maximum_x = (center_x + (side_length / 2.0)) as i32;

    for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
        let x = i as i32 % canvas_width;
        let y = i as i32 / canvas_width;

        // dont calculate distance if its not within the bounding box
        if (x < rough_minimum_x)
            || (x > rough_maximum_x)
            || (y < rough_minimum_y)
            || (y > rough_maximum_y)
        {
            continue;
        }

        pixel.copy_from_slice(rgba);
    }
}

/// Draws an outline of a rectangle to a frame of pixels used in the [pixels](https://docs.rs/pixels/latest/pixels/) crate.
///
/// Note: bottom_left_y and top_right_y are only named correctly mathematically. [pixels](https://docs.rs/pixels/latest/pixels/)
/// renders in the 4th quadrant, so the y values are flipped, with y=0 starting at the top. This means that bottom_left_y is actually
/// rendered to the top left of the rectangle, and top_right_y is rendered to the bottom right of the triangle.
///
/// # Example
///
/// ```no_run
/// use pixels::{Pixels, SurfaceTexture};
/// use winit::dpi::LogicalSize;
/// use winit::event_loop::{EventLoop};
/// use std::error::Error;
/// use winit::window::WindowBuilder;
///
/// const WIDTH: i32 = 800;
/// const HEIGHT: i32 = 800;
///
/// fn main() -> Result<(), Box<dyn Error>> {
///     let event_loop = EventLoop::new();
///     let window = {
///     let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
///     WindowBuilder::new()
///         .with_title("Rectangle Example")
///         .with_inner_size(size)
///         .with_min_inner_size(size)
///         .build(&event_loop)
///         .unwrap()
///     };
///
///     let mut pixels = {
///         let window_size = window.inner_size();
///         let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
///         Pixels::new(WIDTH as u32, HEIGHT as u32, surface_texture)?
///     };
///
///     pixels_primitives::rect(
///         pixels.get_frame(),
///         WIDTH,
///         200,
///         200,
///         500,
///         300,
///         &[255, 255, 255, 255],
///     );
///
///     // Run your event loop here!
///
///     Ok(())
/// }
///
/// ```
#[warn(missing_docs)]
pub fn rect(
    frame: &mut [u8],
    canvas_width: i32,
    bottom_left_x: i32,
    bottom_left_y: i32,
    top_right_x: i32,
    top_right_y: i32,
    rgba: &[u8; 4],
) {
    line(
        frame,
        canvas_width,
        bottom_left_x as f64,
        bottom_left_y as f64,
        bottom_left_x as f64,
        top_right_y as f64,
        rgba,
    );

    line(
        frame,
        canvas_width,
        bottom_left_x as f64,
        bottom_left_y as f64,
        top_right_x as f64,
        bottom_left_y as f64,
        rgba,
    );

    line(
        frame,
        canvas_width,
        top_right_x as f64,
        bottom_left_y as f64,
        top_right_x as f64,
        top_right_y as f64,
        rgba,
    );

    line(
        frame,
        canvas_width,
        top_right_x as f64,
        bottom_left_y as f64,
        top_right_x as f64,
        top_right_y as f64,
        rgba,
    );

    line(
        frame,
        canvas_width,
        top_right_x as f64,
        top_right_y as f64,
        bottom_left_x as f64,
        top_right_y as f64,
        rgba,
    );
}

// TODO: make it so this function works with two arbitrary opposite corners

/// Draws a filled rectangle to a frame of pixels used in the [pixels](https://docs.rs/pixels/latest/pixels/) crate.
///
/// Note: bottom_left_y and top_right_y are only named correctly mathematically. [pixels](https://docs.rs/pixels/latest/pixels/)
/// renders in the 4th quadrant, so the y values are flipped, with y=0 starting at the top. This means that bottom_left_y is actually
/// rendered to the top left of the rectangle, and top_right_y is rendered to the bottom right of the triangle.
///
/// # Example
///
/// ```no_run
/// use pixels::{Pixels, SurfaceTexture};
/// use winit::dpi::LogicalSize;
/// use winit::event_loop::{EventLoop};
/// use std::error::Error;
/// use winit::window::WindowBuilder;
///
/// const WIDTH: i32 = 800;
/// const HEIGHT: i32 = 800;
///
/// fn main() -> Result<(), Box<dyn Error>> {
///     let event_loop = EventLoop::new();
///     let window = {
///     let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
///     WindowBuilder::new()
///         .with_title("Filled Rectangle Example")
///         .with_inner_size(size)
///         .with_min_inner_size(size)
///         .build(&event_loop)
///         .unwrap()
///     };
///
///     let mut pixels = {
///         let window_size = window.inner_size();
///         let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
///         Pixels::new(WIDTH as u32, HEIGHT as u32, surface_texture)?
///     };
///
///     pixels_primitives::rect_filled(
///         pixels.get_frame(),
///         WIDTH,
///         200,
///         200,
///         500,
///         300,
///         &[255, 255, 255, 255],
///     );
///
///     // Run your event loop here!
///
///     Ok(())
/// }
///
/// ```
#[warn(missing_docs)]
pub fn rect_filled(
    frame: &mut [u8],
    canvas_width: i32,
    bottom_left_x: i32,
    bottom_left_y: i32,
    top_right_x: i32,
    top_right_y: i32,
    rgba: &[u8; 4],
) {
    assert!(
        bottom_left_x <= top_right_x,
        "bottom_left_x must be smaller or equal to top_right_x"
    );
    assert!(
        bottom_left_y <= top_right_y,
        "bottom_left_y must be smaller or equal to top_right_y"
    );
    for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
        let x = i as i32 % canvas_width;
        let y = i as i32 / canvas_width;

        // dont calculate distance if its not within the bounding box
        if (x < bottom_left_x) || (x > top_right_x) || (y < bottom_left_y) || (y > top_right_y) {
            continue;
        }

        pixel.copy_from_slice(rgba);
    }
}

#[inline]
fn get_starting_pixel_index(x: i32, y: i32, canvas_width: i32) -> usize {
    (((y * canvas_width) + (x)) * 4) as usize
}

#[inline]
fn color_position(
    x: i32,
    y: i32,
    canvas_width: i32,
    canvas_height: i32,
    frame: &mut [u8],
    rgba: &[u8],
) {
    if (x < 0) || (y < 0) || (x >= canvas_width) || (y >= canvas_height) {
        return;
    }
    let index = get_starting_pixel_index(x, y, canvas_width);
    let pixel = &mut frame[index..index + 4];
    pixel.copy_from_slice(rgba);
}
