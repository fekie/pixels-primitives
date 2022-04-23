//! A quick and simple wrapper for lines and simple shapes for the [pixels](https://docs.rs/pixels/latest/pixels/) crate.

use std::mem;

mod math;

//#[warn(missing_docs)]
pub fn line(
    frame: &mut [u8],
    width: i32,
    starting_x: f64,
    starting_y: f64,
    ending_x: f64,
    ending_y: f64,
    rgba: &[u8; 4],
) {
    // Clone our immutable values into mutable values.
    let (mut mx0, mut my0, mut mx1, mut my1) = (
        starting_x as i32,
        starting_y as i32,
        ending_x as i32,
        ending_y as i32,
    );

    // Checks to see if range is bigger than the domain.
    let steep = (mx0 - mx1).abs() < (my0 - my1).abs();

    // If the line is steep, we transpose the line (by swapping our Xs and Ys, we will undo it later) .
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

    let mut coords_to_be_filled = Vec::new();

    let mut y = my0;
    for x in mx0..mx1 {
        if steep {
            coords_to_be_filled.push((y as u32, x as u32));
        } else {
            coords_to_be_filled.push((x as u32, y as u32));
        }
        error2 += error_increment2;
        if error2 > dx {
            y += if my1 > my0 { 1 } else { -1 };
            error2 -= dx * 2;
        }
    }

    println!("{}", coords_to_be_filled.len());

    // draw the coordinates
    /* for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
        let x = i as i32 % width;
        let y = i as i32 / width;

        let distance = math::distance(center_x, center_y, x as f64, y as f64);
        if (distance <= radius) && (distance >= (radius - outline_width)) {
            pixel.copy_from_slice(rgba);
        }
    } */
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
    width: i32,
    center_x: f64,
    center_y: f64,
    radius: f64,
    outline_width: f64,
    rgba: &[u8; 4],
) {
    let rough_minimum_y = (center_y - radius) as i32;
    let rough_minimum_x = (center_x - radius) as i32;
    let rough_maximum_y = (center_y + radius) as i32;
    let rough_maximum_x = (center_x + radius) as i32;

    for y in rough_minimum_y..=rough_maximum_y {
        for x in rough_minimum_x..=rough_maximum_x {
            let distance = math::distance(center_x, center_y, x as f64, y as f64);
            if (distance <= radius) && (distance >= (radius - outline_width)) {
                let index = (((y * width) + (x)) * 4) as usize;
                frame[index] = rgba[0];
                frame[index + 1] = rgba[1];
                frame[index + 2] = rgba[2];
                frame[index + 3] = rgba[3];
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
    width: i32,
    center_x: f64,
    center_y: f64,
    radius: f64,
    rgba: &[u8; 4],
) {
    let rough_minimum_y = (center_y - radius) as i32;
    let rough_minimum_x = (center_x - radius) as i32;
    let rough_maximum_y = (center_y + radius) as i32;
    let rough_maximum_x = (center_x + radius) as i32;

    for y in rough_minimum_y..=rough_maximum_y {
        for x in rough_minimum_x..=rough_maximum_x {
            let distance = math::distance(center_x, center_y, x as f64, y as f64);
            if distance <= radius {
                let index = (((y * width) + (x)) * 4) as usize;
                frame[index] = rgba[0];
                frame[index + 1] = rgba[1];
                frame[index + 2] = rgba[2];
                frame[index + 3] = rgba[3];
            }
        }
    }
}

//#[warn(missing_docs)]
pub fn square(
    frame: &mut [u8],
    width: i32,
    center_x: f64,
    center_y: f64,
    side_length: f64,
    outline_width: f64,
    rgba: &[u8; 4],
) {
    unimplemented!()
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
    width: i32,
    center_x: f64,
    center_y: f64,
    side_length: f64,
    rgba: &[u8; 4],
) {
    let rough_minimum_y = (center_y - (side_length / 2.0)) as i32;
    let rough_minimum_x = (center_x - (side_length / 2.0)) as i32;
    let rough_maximum_y = (center_y + (side_length / 2.0)) as i32;
    let rough_maximum_x = (center_x + (side_length / 2.0)) as i32;

    for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
        let x = i as i32 % width;
        let y = i as i32 / width;

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

#[warn(missing_docs)]
pub fn rect(buffer: &mut [u8]) {
    unimplemented!()
}

#[warn(missing_docs)]
pub fn rect_filled(buffer: &mut [u8]) {
    unimplemented!()
}
