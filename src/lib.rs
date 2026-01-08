//! Fractal Interference Library
//!
//! A WebAssembly-powered library for generating and interfering fractals.
//! Supports 2D fractals (Mandelbrot, Julia sets) and 3D fractal visualization
//! with wave interference capabilities.

mod fractals;
mod interference;

pub use fractals::{FractalGenerator, FractalType};
pub use interference::{FractalInterference, InterferenceMode, WaveParameters};

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

/// Package version (matches Cargo.toml version).
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Configuration for fractal generation.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[wasm_bindgen]
pub struct FractalConfig {
    /// Width of the output image in pixels.
    pub width: u32,
    /// Height of the output image in pixels.
    pub height: u32,
    /// Maximum number of iterations for escape-time fractals.
    pub max_iterations: u32,
    /// Zoom level (higher = more zoomed in).
    pub zoom: f64,
    /// X coordinate of the center point.
    pub center_x: f64,
    /// Y coordinate of the center point.
    pub center_y: f64,
}

#[wasm_bindgen]
impl FractalConfig {
    /// Creates a new fractal configuration with default values.
    #[wasm_bindgen(constructor)]
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a configuration for Mandelbrot set visualization.
    #[must_use]
    pub fn mandelbrot_default() -> Self {
        Self {
            width: 800,
            height: 600,
            max_iterations: 256,
            zoom: 1.0,
            center_x: -0.5,
            center_y: 0.0,
        }
    }

    /// Creates a configuration for Julia set visualization.
    #[must_use]
    pub fn julia_default() -> Self {
        Self {
            width: 800,
            height: 600,
            max_iterations: 256,
            zoom: 1.0,
            center_x: 0.0,
            center_y: 0.0,
        }
    }
}

impl Default for FractalConfig {
    fn default() -> Self {
        Self {
            width: 800,
            height: 600,
            max_iterations: 256,
            zoom: 1.0,
            center_x: 0.0,
            center_y: 0.0,
        }
    }
}

/// Renderer for fractal interference visualization.
///
/// This is the main entry point for WebAssembly usage.
#[wasm_bindgen]
pub struct FractalRenderer {
    config: FractalConfig,
    fractal1: FractalGenerator,
    fractal2: Option<FractalGenerator>,
    interference: FractalInterference,
}

#[wasm_bindgen]
impl FractalRenderer {
    /// Creates a new fractal renderer with the given configuration.
    #[wasm_bindgen(constructor)]
    #[must_use]
    pub fn new(config: FractalConfig) -> Self {
        Self {
            config: config.clone(),
            fractal1: FractalGenerator::new(FractalType::Mandelbrot, config),
            fractal2: None,
            interference: FractalInterference::new(),
        }
    }

    /// Sets the primary fractal type.
    pub fn set_fractal_type(&mut self, fractal_type: &str) {
        let ft = match fractal_type {
            "julia" => FractalType::Julia {
                c_real: -0.7,
                c_imag: 0.27015,
            },
            "burning_ship" => FractalType::BurningShip,
            "tricorn" => FractalType::Tricorn,
            _ => FractalType::Mandelbrot,
        };
        self.fractal1 = FractalGenerator::new(ft, self.config.clone());
    }

    /// Sets Julia set parameters.
    pub fn set_julia_params(&mut self, c_real: f64, c_imag: f64) {
        self.fractal1 =
            FractalGenerator::new(FractalType::Julia { c_real, c_imag }, self.config.clone());
    }

    /// Enables a second fractal for interference.
    pub fn enable_second_fractal(&mut self, fractal_type: &str) {
        let ft = match fractal_type {
            "julia" => FractalType::Julia {
                c_real: -0.7,
                c_imag: 0.27015,
            },
            "burning_ship" => FractalType::BurningShip,
            "tricorn" => FractalType::Tricorn,
            _ => FractalType::Mandelbrot,
        };
        self.fractal2 = Some(FractalGenerator::new(ft, self.config.clone()));
    }

    /// Disables the second fractal.
    pub fn disable_second_fractal(&mut self) {
        self.fractal2 = None;
    }

    /// Sets the interference mode.
    pub fn set_interference_mode(&mut self, mode: &str) {
        self.interference.set_mode(match mode {
            "subtract" => InterferenceMode::Subtract,
            "multiply" => InterferenceMode::Multiply,
            "wave" => InterferenceMode::Wave,
            "phase" => InterferenceMode::Phase,
            _ => InterferenceMode::Add,
        });
    }

    /// Sets wave parameters for interference.
    pub fn set_wave_params(&mut self, amplitude: f64, frequency: f64, phase: f64) {
        self.interference.set_wave_params(WaveParameters {
            amplitude,
            frequency,
            phase,
        });
    }

    /// Updates the configuration.
    pub fn update_config(&mut self, config: FractalConfig) {
        self.config = config.clone();
        self.fractal1.update_config(config.clone());
        if let Some(ref mut f2) = self.fractal2 {
            f2.update_config(config);
        }
    }

    /// Renders the fractal(s) and returns RGBA pixel data.
    ///
    /// Returns a flat array of RGBA values (4 bytes per pixel).
    #[must_use]
    pub fn render(&self) -> Vec<u8> {
        let data1 = self.fractal1.generate();

        let final_data = if let Some(ref fractal2) = self.fractal2 {
            let data2 = fractal2.generate();
            self.interference.interfere(&data1, &data2)
        } else {
            data1
        };

        self.colorize(&final_data)
    }

    /// Renders a 3D height map representation.
    ///
    /// Returns height values normalized to 0-255.
    #[must_use]
    pub fn render_3d_heightmap(&self) -> Vec<u8> {
        let data = self.fractal1.generate();
        let max_iter = f64::from(self.config.max_iterations);

        data.iter()
            .map(|&v| ((v / max_iter) * 255.0) as u8)
            .collect()
    }

    /// Colorizes fractal data into RGBA format.
    fn colorize(&self, data: &[f64]) -> Vec<u8> {
        let max_iter = f64::from(self.config.max_iterations);
        let mut pixels = Vec::with_capacity(data.len() * 4);

        for &value in data {
            let (r, g, b) = if value >= max_iter {
                (0, 0, 0) // Inside the set
            } else {
                // Smooth coloring using logarithmic scale
                let normalized = value / max_iter;
                let hue = normalized * 360.0;
                hsv_to_rgb(hue, 0.8, 1.0 - normalized * 0.3)
            };

            pixels.push(r);
            pixels.push(g);
            pixels.push(b);
            pixels.push(255); // Alpha
        }

        pixels
    }

    /// Returns the current version of the library.
    #[wasm_bindgen(getter)]
    #[must_use]
    pub fn version(&self) -> String {
        VERSION.to_string()
    }
}

/// Converts HSV color to RGB.
#[allow(clippy::many_single_char_names)]
fn hsv_to_rgb(hue: f64, saturation: f64, value: f64) -> (u8, u8, u8) {
    let h = hue % 360.0;
    let c = value * saturation;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = value - c;

    let (red, green, blue) = match (h / 60.0) as u32 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };

    (
        ((red + m) * 255.0) as u8,
        ((green + m) * 255.0) as u8,
        ((blue + m) * 255.0) as u8,
    )
}

/// Initialize the WebAssembly module.
/// Call this once when the module is loaded.
#[wasm_bindgen(start)]
pub fn init() {
    // Set up panic hook for better error messages in the browser
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_not_empty() {
        assert!(!VERSION.is_empty());
    }

    #[test]
    fn test_default_config() {
        let config = FractalConfig::default();
        assert_eq!(config.width, 800);
        assert_eq!(config.height, 600);
        assert_eq!(config.max_iterations, 256);
    }

    #[test]
    fn test_renderer_creation() {
        let config = FractalConfig::new();
        let renderer = FractalRenderer::new(config);
        assert_eq!(renderer.version(), VERSION);
    }

    #[test]
    fn test_render_produces_correct_size() {
        let mut config = FractalConfig::new();
        config.width = 100;
        config.height = 100;
        let renderer = FractalRenderer::new(config);
        let pixels = renderer.render();
        // 100 * 100 pixels * 4 bytes (RGBA)
        assert_eq!(pixels.len(), 100 * 100 * 4);
    }

    #[test]
    fn test_hsv_to_rgb() {
        // Red
        let (r, g, b) = hsv_to_rgb(0.0, 1.0, 1.0);
        assert_eq!(r, 255);
        assert_eq!(g, 0);
        assert_eq!(b, 0);

        // Green (approximately)
        let (r, g, b) = hsv_to_rgb(120.0, 1.0, 1.0);
        assert_eq!(r, 0);
        assert_eq!(g, 255);
        assert_eq!(b, 0);
    }

    #[test]
    fn test_mandelbrot_config() {
        let config = FractalConfig::mandelbrot_default();
        assert!((config.center_x - (-0.5)).abs() < 0.001);
    }

    #[test]
    fn test_julia_config() {
        let config = FractalConfig::julia_default();
        assert!((config.center_x - 0.0).abs() < 0.001);
    }
}
