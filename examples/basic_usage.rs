//! Basic usage example for fractal-interference.
//!
//! This example demonstrates the basic functionality of the fractal library.
//!
//! Run with: `cargo run --example basic_usage`

use fractal_interference::{
    FractalConfig, FractalGenerator, FractalRenderer, FractalType, VERSION,
};

fn main() {
    println!("Fractal Interference Library v{VERSION}");
    println!("========================================");
    println!();

    // Example 1: Basic Mandelbrot set
    example_mandelbrot();

    // Example 2: Julia set with custom parameters
    example_julia();

    // Example 3: Fractal interference
    example_interference();

    // Example 4: 3D heightmap
    example_3d_heightmap();

    println!();
    println!("Examples completed successfully!");
}

fn example_mandelbrot() {
    println!("Example 1: Mandelbrot Set");
    println!("--------------------------");

    let config = FractalConfig::mandelbrot_default();
    let generator = FractalGenerator::new(FractalType::Mandelbrot, config);
    let data = generator.generate();

    // Calculate statistics
    let max_val = data.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let min_val = data.iter().copied().fold(f64::INFINITY, f64::min);
    let avg_val: f64 = data.iter().sum::<f64>() / data.len() as f64;

    println!("  Generated {} data points", data.len());
    println!("  Value range: {min_val:.2} - {max_val:.2}");
    println!("  Average value: {avg_val:.2}");
    println!();
}

fn example_julia() {
    println!("Example 2: Julia Set");
    println!("--------------------");

    // Classic Julia set parameters
    let params = [
        (-0.7, 0.27015, "Dendrite"),
        (-0.8, 0.156, "Siegel disk"),
        (0.285, 0.01, "Douady rabbit"),
        (-0.4, 0.6, "San Marco"),
    ];

    for (c_real, c_imag, name) in params {
        let config = FractalConfig::julia_default();
        let generator = FractalGenerator::new(FractalType::Julia { c_real, c_imag }, config);
        let data = generator.generate();
        println!("  {name} (c = {c_real} + {c_imag}i): {} points", data.len());
    }
    println!();
}

fn example_interference() {
    println!("Example 3: Fractal Interference");
    println!("--------------------------------");

    let config = FractalConfig {
        width: 200,
        height: 200,
        ..Default::default()
    };

    let mut renderer = FractalRenderer::new(config);

    // Enable a second fractal for interference
    renderer.enable_second_fractal("julia");

    // Test different interference modes
    let modes = ["add", "subtract", "multiply", "wave", "phase"];

    for mode in modes {
        renderer.set_interference_mode(mode);

        if mode == "wave" {
            // Set custom wave parameters
            renderer.set_wave_params(0.8, 1.5, 0.5);
        }

        let pixels = renderer.render();
        let non_black = pixels
            .chunks(4)
            .filter(|chunk| chunk[0] > 0 || chunk[1] > 0 || chunk[2] > 0)
            .count();

        println!(
            "  Mode '{mode}': {non_black} colored pixels out of {}",
            pixels.len() / 4
        );
    }
    println!();
}

fn example_3d_heightmap() {
    println!("Example 4: 3D Heightmap");
    println!("-----------------------");

    let config = FractalConfig {
        width: 100,
        height: 100,
        ..FractalConfig::mandelbrot_default()
    };

    let renderer = FractalRenderer::new(config);
    let heightmap = renderer.render_3d_heightmap();

    // Calculate height distribution
    let mut buckets = [0usize; 10];
    for &height in &heightmap {
        let bucket = (height as usize * 9) / 255;
        buckets[bucket] += 1;
    }

    println!("  Heightmap generated: {} values", heightmap.len());
    println!("  Height distribution:");
    for (i, &count) in buckets.iter().enumerate() {
        let bar = "#".repeat(count / 100);
        println!("    {i:>2}0-{:>3}: {bar}", (i + 1) * 10 - 1);
    }
    println!();
}
