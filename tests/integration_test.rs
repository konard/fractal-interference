//! Integration tests for fractal-interference.
//!
//! These tests verify the public API works correctly.

use fractal_interference::{
    FractalConfig, FractalGenerator, FractalInterference, FractalRenderer, FractalType,
    InterferenceMode, WaveParameters, VERSION,
};

mod config_tests {
    use super::*;

    #[test]
    fn test_default_config_values() {
        let config = FractalConfig::default();
        assert_eq!(config.width, 800);
        assert_eq!(config.height, 600);
        assert_eq!(config.max_iterations, 256);
        assert!((config.zoom - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_mandelbrot_default_config() {
        let config = FractalConfig::mandelbrot_default();
        assert!((config.center_x - (-0.5)).abs() < 0.001);
        assert!((config.center_y - 0.0).abs() < 0.001);
    }

    #[test]
    fn test_julia_default_config() {
        let config = FractalConfig::julia_default();
        assert!((config.center_x - 0.0).abs() < 0.001);
        assert!((config.center_y - 0.0).abs() < 0.001);
    }
}

mod fractal_generator_tests {
    use super::*;

    fn small_config() -> FractalConfig {
        FractalConfig {
            width: 50,
            height: 50,
            max_iterations: 50,
            ..Default::default()
        }
    }

    #[test]
    fn test_mandelbrot_generates_correct_size() {
        let config = small_config();
        let generator = FractalGenerator::new(FractalType::Mandelbrot, config);
        let data = generator.generate();
        assert_eq!(data.len(), 50 * 50);
    }

    #[test]
    fn test_julia_generates_correct_size() {
        let config = small_config();
        let generator = FractalGenerator::new(
            FractalType::Julia {
                c_real: -0.7,
                c_imag: 0.27015,
            },
            config,
        );
        let data = generator.generate();
        assert_eq!(data.len(), 50 * 50);
    }

    #[test]
    fn test_burning_ship_generates_correct_size() {
        let config = small_config();
        let generator = FractalGenerator::new(FractalType::BurningShip, config);
        let data = generator.generate();
        assert_eq!(data.len(), 50 * 50);
    }

    #[test]
    fn test_tricorn_generates_correct_size() {
        let config = small_config();
        let generator = FractalGenerator::new(FractalType::Tricorn, config);
        let data = generator.generate();
        assert_eq!(data.len(), 50 * 50);
    }

    #[test]
    fn test_all_values_non_negative() {
        let config = small_config();
        let generator = FractalGenerator::new(FractalType::Mandelbrot, config);
        let data = generator.generate();
        assert!(data.iter().all(|&v| v >= 0.0));
    }
}

mod interference_tests {
    use super::*;

    #[test]
    fn test_add_mode() {
        let mut interference = FractalInterference::new();
        interference.set_mode(InterferenceMode::Add);

        let data1 = vec![10.0, 20.0, 30.0];
        let data2 = vec![10.0, 20.0, 30.0];
        let result = interference.interfere(&data1, &data2);

        assert_eq!(result.len(), 3);
        // Add mode averages the values
        assert!((result[0] - 10.0).abs() < 0.001);
        assert!((result[1] - 20.0).abs() < 0.001);
    }

    #[test]
    fn test_subtract_mode() {
        let mut interference = FractalInterference::new();
        interference.set_mode(InterferenceMode::Subtract);

        let data1 = vec![30.0, 20.0, 10.0];
        let data2 = vec![10.0, 20.0, 30.0];
        let result = interference.interfere(&data1, &data2);

        assert_eq!(result.len(), 3);
        assert!((result[0] - 20.0).abs() < 0.001); // |30 - 10| = 20
        assert!((result[1] - 0.0).abs() < 0.001); // |20 - 20| = 0
        assert!((result[2] - 20.0).abs() < 0.001); // |10 - 30| = 20
    }

    #[test]
    fn test_wave_params_can_be_set() {
        let mut interference = FractalInterference::new();
        let params = WaveParameters {
            amplitude: 0.5,
            frequency: 2.0,
            phase: 1.57,
        };
        interference.set_wave_params(params);
        interference.set_mode(InterferenceMode::Wave);

        // Should not panic
        let data1 = vec![100.0; 10];
        let data2 = vec![100.0; 10];
        let result = interference.interfere(&data1, &data2);
        assert_eq!(result.len(), 10);
    }
}

mod renderer_tests {
    use super::*;

    fn small_config() -> FractalConfig {
        FractalConfig {
            width: 100,
            height: 100,
            max_iterations: 50,
            ..Default::default()
        }
    }

    #[test]
    fn test_renderer_produces_rgba_output() {
        let config = small_config();
        let renderer = FractalRenderer::new(config);
        let pixels = renderer.render();

        // 100 * 100 * 4 (RGBA)
        assert_eq!(pixels.len(), 100 * 100 * 4);
    }

    #[test]
    fn test_renderer_version() {
        let config = small_config();
        let renderer = FractalRenderer::new(config);
        assert_eq!(renderer.version(), VERSION);
    }

    #[test]
    fn test_set_fractal_type() {
        let config = small_config();
        let mut renderer = FractalRenderer::new(config);

        // Should not panic
        renderer.set_fractal_type("mandelbrot");
        renderer.set_fractal_type("julia");
        renderer.set_fractal_type("burning_ship");
        renderer.set_fractal_type("tricorn");
        renderer.set_fractal_type("unknown"); // Falls back to Mandelbrot
    }

    #[test]
    fn test_set_julia_params() {
        let config = small_config();
        let mut renderer = FractalRenderer::new(config);
        renderer.set_julia_params(-0.8, 0.156);

        let pixels = renderer.render();
        assert_eq!(pixels.len(), 100 * 100 * 4);
    }

    #[test]
    fn test_enable_disable_second_fractal() {
        let config = small_config();
        let mut renderer = FractalRenderer::new(config);

        renderer.enable_second_fractal("julia");
        let with_second = renderer.render();

        renderer.disable_second_fractal();
        let without_second = renderer.render();

        // Both should produce valid output
        assert_eq!(with_second.len(), 100 * 100 * 4);
        assert_eq!(without_second.len(), 100 * 100 * 4);
    }

    #[test]
    fn test_interference_modes() {
        let config = small_config();
        let mut renderer = FractalRenderer::new(config);
        renderer.enable_second_fractal("julia");

        for mode in &["add", "subtract", "multiply", "wave", "phase"] {
            renderer.set_interference_mode(mode);
            let pixels = renderer.render();
            assert_eq!(pixels.len(), 100 * 100 * 4, "Failed for mode: {mode}");
        }
    }

    #[test]
    fn test_render_3d_heightmap() {
        let config = small_config();
        let renderer = FractalRenderer::new(config);
        let heightmap = renderer.render_3d_heightmap();

        // One byte per pixel (height value)
        assert_eq!(heightmap.len(), 100 * 100);
        // Verify heightmap produces valid output
        assert!(!heightmap.is_empty());
    }

    #[test]
    fn test_update_config() {
        let config = small_config();
        let mut renderer = FractalRenderer::new(config);

        let new_config = FractalConfig {
            width: 50,
            height: 50,
            max_iterations: 100,
            ..Default::default()
        };
        renderer.update_config(new_config);

        let pixels = renderer.render();
        assert_eq!(pixels.len(), 50 * 50 * 4);
    }
}

mod version_tests {
    use super::*;

    #[test]
    fn test_version_is_not_empty() {
        assert!(!VERSION.is_empty());
    }

    #[test]
    fn test_version_format() {
        // Version should be in semver format
        assert!(
            VERSION.split('.').count() >= 2,
            "Version should have at least major.minor"
        );
    }
}
