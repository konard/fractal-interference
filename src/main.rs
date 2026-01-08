//! Fractal Interference CLI
//!
//! A command-line tool for generating fractal interference patterns.
//! This is primarily for testing; the main usage is via WebAssembly in the browser.

use fractal_interference::{FractalConfig, FractalRenderer, VERSION};

fn main() {
    println!("Fractal Interference v{VERSION}");
    println!();

    // Create a default configuration
    let mut config = FractalConfig::mandelbrot_default();
    config.width = 200;
    config.height = 150;
    config.max_iterations = 100;

    println!("Configuration:");
    println!("  Size: {}x{}", config.width, config.height);
    println!("  Max iterations: {}", config.max_iterations);
    println!("  Zoom: {}", config.zoom);
    println!("  Center: ({}, {})", config.center_x, config.center_y);
    println!();

    // Create renderer
    let mut renderer = FractalRenderer::new(config);

    // Render Mandelbrot set
    println!("Generating Mandelbrot set...");
    let pixels = renderer.render();
    println!(
        "  Generated {} pixels ({} bytes)",
        pixels.len() / 4,
        pixels.len()
    );

    // Display ASCII preview
    println!();
    println!("ASCII Preview (50x25):");
    print_ascii_preview(&pixels, 200, 150, 50, 25);

    // Test Julia set
    println!();
    println!("Switching to Julia set (c = -0.7 + 0.27015i)...");
    renderer.set_julia_params(-0.7, 0.27015);
    let julia_pixels = renderer.render();
    println!("  Generated {} pixels", julia_pixels.len() / 4);

    // Display Julia ASCII preview
    println!();
    println!("Julia Set Preview:");
    print_ascii_preview(&julia_pixels, 200, 150, 50, 25);

    // Test interference
    println!();
    println!("Testing fractal interference...");
    renderer.set_fractal_type("mandelbrot");
    renderer.enable_second_fractal("julia");
    renderer.set_interference_mode("wave");
    renderer.set_wave_params(1.0, 2.0, 0.5);
    let interference_pixels = renderer.render();
    println!(
        "  Generated interference pattern: {} pixels",
        interference_pixels.len() / 4
    );

    println!();
    println!("Interference Pattern Preview:");
    print_ascii_preview(&interference_pixels, 200, 150, 50, 25);

    println!();
    println!("Done! For interactive visualization, build the WebAssembly module");
    println!("and open the web app in your browser.");
}

/// Prints an ASCII art preview of the fractal.
fn print_ascii_preview(
    pixels: &[u8],
    width: u32,
    height: u32,
    preview_width: u32,
    preview_height: u32,
) {
    let chars = [' ', '.', ':', '-', '=', '+', '*', '#', '%', '@'];

    let scale_x = width / preview_width;
    let scale_y = height / preview_height;

    for py in 0..preview_height {
        for px in 0..preview_width {
            let src_x = px * scale_x;
            let src_y = py * scale_y;
            let idx = ((src_y * width + src_x) * 4) as usize;

            if idx + 2 < pixels.len() {
                // Calculate brightness from RGB
                let r = u32::from(pixels[idx]);
                let g = u32::from(pixels[idx + 1]);
                let b = u32::from(pixels[idx + 2]);
                let brightness = (r + g + b) / 3;

                // Map to ASCII character
                let char_idx = (brightness * 9 / 255) as usize;
                print!("{}", chars[char_idx.min(chars.len() - 1)]);
            } else {
                print!(" ");
            }
        }
        println!();
    }
}
