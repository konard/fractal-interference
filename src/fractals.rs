//! Fractal generation algorithms.
//!
//! This module provides various fractal generation algorithms including:
//! - Mandelbrot set
//! - Julia sets
//! - Burning Ship fractal
//! - Tricorn (Mandelbar)

use crate::FractalConfig;
use num_complex::Complex64;

/// Supported fractal types.
#[derive(Debug, Clone, Copy)]
pub enum FractalType {
    /// The Mandelbrot set: z = z² + c where c varies.
    Mandelbrot,
    /// Julia set: z = z² + c where c is fixed.
    Julia { c_real: f64, c_imag: f64 },
    /// Burning Ship fractal: z = (|Re(z)| + i|Im(z)|)² + c.
    BurningShip,
    /// Tricorn (Mandelbar): z = conj(z)² + c.
    Tricorn,
}

/// Generator for escape-time fractals.
pub struct FractalGenerator {
    fractal_type: FractalType,
    config: FractalConfig,
}

impl FractalGenerator {
    /// Creates a new fractal generator.
    #[must_use]
    pub fn new(fractal_type: FractalType, config: FractalConfig) -> Self {
        Self {
            fractal_type,
            config,
        }
    }

    /// Updates the configuration.
    pub fn update_config(&mut self, config: FractalConfig) {
        self.config = config;
    }

    /// Generates fractal data as iteration counts.
    ///
    /// Returns a flat array of iteration counts (one f64 per pixel).
    #[must_use]
    pub fn generate(&self) -> Vec<f64> {
        let width = self.config.width as usize;
        let height = self.config.height as usize;
        let mut data = Vec::with_capacity(width * height);

        let scale = 4.0
            / (self.config.zoom * f64::from(self.config.width).min(f64::from(self.config.height)));
        let center_x = self.config.center_x;
        let center_y = self.config.center_y;

        for py in 0..height {
            for px in 0..width {
                let x = (px as f64 - f64::from(self.config.width) / 2.0) * scale + center_x;
                let y = (py as f64 - f64::from(self.config.height) / 2.0) * scale + center_y;

                let iterations = self.calculate_pixel(x, y);
                data.push(iterations);
            }
        }

        data
    }

    /// Calculates the escape iteration for a single pixel.
    fn calculate_pixel(&self, x: f64, y: f64) -> f64 {
        match self.fractal_type {
            FractalType::Mandelbrot => self.mandelbrot(x, y),
            FractalType::Julia { c_real, c_imag } => self.julia(x, y, c_real, c_imag),
            FractalType::BurningShip => self.burning_ship(x, y),
            FractalType::Tricorn => self.tricorn(x, y),
        }
    }

    /// Mandelbrot set calculation: z = z² + c.
    fn mandelbrot(&self, x: f64, y: f64) -> f64 {
        let c = Complex64::new(x, y);
        let mut z = Complex64::new(0.0, 0.0);
        let max_iter = self.config.max_iterations;

        for i in 0..max_iter {
            if z.norm_sqr() > 4.0 {
                // Smooth coloring using continuous iteration count
                let log_zn = z.norm_sqr().ln() / 2.0;
                let nu = log_zn.ln() / std::f64::consts::LN_2;
                return f64::from(i) + 1.0 - nu;
            }
            z = z * z + c;
        }

        f64::from(max_iter)
    }

    /// Julia set calculation: z = z² + c (fixed c).
    fn julia(&self, x: f64, y: f64, c_real: f64, c_imag: f64) -> f64 {
        let c = Complex64::new(c_real, c_imag);
        let mut z = Complex64::new(x, y);
        let max_iter = self.config.max_iterations;

        for i in 0..max_iter {
            if z.norm_sqr() > 4.0 {
                let log_zn = z.norm_sqr().ln() / 2.0;
                let nu = log_zn.ln() / std::f64::consts::LN_2;
                return f64::from(i) + 1.0 - nu;
            }
            z = z * z + c;
        }

        f64::from(max_iter)
    }

    /// Burning Ship fractal: z = (|Re(z)| + i|Im(z)|)² + c.
    fn burning_ship(&self, x: f64, y: f64) -> f64 {
        let c = Complex64::new(x, y);
        let mut z = Complex64::new(0.0, 0.0);
        let max_iter = self.config.max_iterations;

        for i in 0..max_iter {
            if z.norm_sqr() > 4.0 {
                let log_zn = z.norm_sqr().ln() / 2.0;
                let nu = log_zn.ln() / std::f64::consts::LN_2;
                return f64::from(i) + 1.0 - nu;
            }
            // Take absolute value of real and imaginary parts
            z = Complex64::new(z.re.abs(), z.im.abs());
            z = z * z + c;
        }

        f64::from(max_iter)
    }

    /// Tricorn (Mandelbar) fractal: z = conj(z)² + c.
    fn tricorn(&self, x: f64, y: f64) -> f64 {
        let c = Complex64::new(x, y);
        let mut z = Complex64::new(0.0, 0.0);
        let max_iter = self.config.max_iterations;

        for i in 0..max_iter {
            if z.norm_sqr() > 4.0 {
                let log_zn = z.norm_sqr().ln() / 2.0;
                let nu = log_zn.ln() / std::f64::consts::LN_2;
                return f64::from(i) + 1.0 - nu;
            }
            // Use complex conjugate
            let z_conj = z.conj();
            z = z_conj * z_conj + c;
        }

        f64::from(max_iter)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_config() -> FractalConfig {
        FractalConfig {
            width: 100,
            height: 100,
            max_iterations: 100,
            zoom: 1.0,
            center_x: 0.0,
            center_y: 0.0,
        }
    }

    #[test]
    fn test_mandelbrot_generation() {
        let generator = FractalGenerator::new(FractalType::Mandelbrot, test_config());
        let data = generator.generate();
        assert_eq!(data.len(), 100 * 100);
    }

    #[test]
    fn test_julia_generation() {
        let generator = FractalGenerator::new(
            FractalType::Julia {
                c_real: -0.7,
                c_imag: 0.27015,
            },
            test_config(),
        );
        let data = generator.generate();
        assert_eq!(data.len(), 100 * 100);
    }

    #[test]
    fn test_burning_ship_generation() {
        let generator = FractalGenerator::new(FractalType::BurningShip, test_config());
        let data = generator.generate();
        assert_eq!(data.len(), 100 * 100);
    }

    #[test]
    fn test_tricorn_generation() {
        let generator = FractalGenerator::new(FractalType::Tricorn, test_config());
        let data = generator.generate();
        assert_eq!(data.len(), 100 * 100);
    }

    #[test]
    fn test_mandelbrot_center_is_in_set() {
        let config = FractalConfig {
            width: 10,
            height: 10,
            max_iterations: 100,
            zoom: 1.0,
            center_x: 0.0,
            center_y: 0.0,
        };
        let generator = FractalGenerator::new(FractalType::Mandelbrot, config);
        // The center pixel at (0, 0) is inside the Mandelbrot set
        let iterations = generator.mandelbrot(0.0, 0.0);
        assert!((iterations - 100.0).abs() < 0.01);
    }

    #[test]
    fn test_point_outside_set() {
        let config = FractalConfig {
            width: 10,
            height: 10,
            max_iterations: 100,
            zoom: 1.0,
            center_x: 0.0,
            center_y: 0.0,
        };
        let generator = FractalGenerator::new(FractalType::Mandelbrot, config);
        // Point (2, 2) is definitely outside the Mandelbrot set
        let iterations = generator.mandelbrot(2.0, 2.0);
        assert!(iterations < 10.0);
    }

    #[test]
    fn test_update_config() {
        let mut generator = FractalGenerator::new(FractalType::Mandelbrot, test_config());
        let new_config = FractalConfig {
            width: 200,
            height: 200,
            max_iterations: 200,
            zoom: 2.0,
            center_x: -0.5,
            center_y: 0.0,
        };
        generator.update_config(new_config);
        let data = generator.generate();
        assert_eq!(data.len(), 200 * 200);
    }
}
