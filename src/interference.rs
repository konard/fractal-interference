//! Wave interference module for fractal combination.
//!
//! This module provides various methods to combine and interfere fractals
//! like waves, enabling the creation of complex patterns.

use std::f64::consts::PI;

/// Modes for combining two fractals.
#[derive(Debug, Clone, Copy, Default)]
pub enum InterferenceMode {
    /// Add the two fractal values (constructive interference).
    #[default]
    Add,
    /// Subtract the second from the first (destructive interference).
    Subtract,
    /// Multiply the two fractal values.
    Multiply,
    /// Wave-based interference using sine functions.
    Wave,
    /// Phase-based interference.
    Phase,
}

/// Parameters for wave-based interference.
#[derive(Debug, Clone, Copy)]
pub struct WaveParameters {
    /// Amplitude of the wave (0.0 to 1.0).
    pub amplitude: f64,
    /// Frequency multiplier.
    pub frequency: f64,
    /// Phase offset in radians.
    pub phase: f64,
}

impl Default for WaveParameters {
    fn default() -> Self {
        Self {
            amplitude: 1.0,
            frequency: 1.0,
            phase: 0.0,
        }
    }
}

/// Handles interference between two fractals.
pub struct FractalInterference {
    mode: InterferenceMode,
    wave_params: WaveParameters,
}

impl FractalInterference {
    /// Creates a new interference handler with default settings.
    #[must_use]
    pub fn new() -> Self {
        Self {
            mode: InterferenceMode::default(),
            wave_params: WaveParameters::default(),
        }
    }

    /// Sets the interference mode.
    pub fn set_mode(&mut self, mode: InterferenceMode) {
        self.mode = mode;
    }

    /// Sets the wave parameters.
    pub fn set_wave_params(&mut self, params: WaveParameters) {
        self.wave_params = params;
    }

    /// Combines two fractal datasets using the current interference mode.
    ///
    /// Both datasets must have the same length.
    #[must_use]
    pub fn interfere(&self, data1: &[f64], data2: &[f64]) -> Vec<f64> {
        assert_eq!(
            data1.len(),
            data2.len(),
            "Fractal data must have the same dimensions"
        );

        match self.mode {
            InterferenceMode::Add => self.interfere_add(data1, data2),
            InterferenceMode::Subtract => self.interfere_subtract(data1, data2),
            InterferenceMode::Multiply => self.interfere_multiply(data1, data2),
            InterferenceMode::Wave => self.interfere_wave(data1, data2),
            InterferenceMode::Phase => self.interfere_phase(data1, data2),
        }
    }

    /// Additive interference (constructive).
    #[allow(clippy::unused_self)]
    fn interfere_add(&self, data1: &[f64], data2: &[f64]) -> Vec<f64> {
        data1
            .iter()
            .zip(data2.iter())
            .map(|(&a, &b)| (a + b) / 2.0)
            .collect()
    }

    /// Subtractive interference (destructive).
    #[allow(clippy::unused_self)]
    fn interfere_subtract(&self, data1: &[f64], data2: &[f64]) -> Vec<f64> {
        data1
            .iter()
            .zip(data2.iter())
            .map(|(&a, &b)| (a - b).abs())
            .collect()
    }

    /// Multiplicative interference.
    #[allow(clippy::unused_self)]
    fn interfere_multiply(&self, data1: &[f64], data2: &[f64]) -> Vec<f64> {
        // Find max to normalize
        let max1 = data1.iter().copied().fold(f64::NEG_INFINITY, f64::max);
        let max2 = data2.iter().copied().fold(f64::NEG_INFINITY, f64::max);

        if max1 == 0.0 || max2 == 0.0 {
            return vec![0.0; data1.len()];
        }

        data1
            .iter()
            .zip(data2.iter())
            .map(|(&a, &b)| {
                let norm_a = a / max1;
                let norm_b = b / max2;
                norm_a * norm_b * max1.max(max2)
            })
            .collect()
    }

    /// Wave-based interference using sine functions.
    ///
    /// This creates patterns similar to physical wave interference,
    /// where the fractals are treated as wave amplitudes.
    fn interfere_wave(&self, data1: &[f64], data2: &[f64]) -> Vec<f64> {
        let amp = self.wave_params.amplitude;
        let freq = self.wave_params.frequency;
        let phase = self.wave_params.phase;

        data1
            .iter()
            .zip(data2.iter())
            .map(|(&a, &b)| {
                // Convert fractal values to waves (using to_radians for accuracy)
                let wave1 = amp * (a * freq).to_radians().sin();
                let wave2 = amp * ((b * freq).to_radians() + phase).sin();

                // Superposition of waves
                let combined = wave1 + wave2;

                // Map back to positive iteration-like values
                2.0f64.mul_add(amp, combined) * (a.max(b) / (4.0 * amp)).max(1.0)
            })
            .collect()
    }

    /// Phase-based interference.
    ///
    /// Creates interference patterns based on the phase difference
    /// between the two fractal values.
    fn interfere_phase(&self, data1: &[f64], data2: &[f64]) -> Vec<f64> {
        let freq = self.wave_params.frequency;

        data1
            .iter()
            .zip(data2.iter())
            .map(|(&a, &b)| {
                // Calculate phase difference (using to_radians for accuracy)
                let phase1 = (a * freq).to_radians() % (2.0 * PI);
                let phase2 = (b * freq).to_radians() % (2.0 * PI);
                let phase_diff = (phase1 - phase2).abs();

                // Interference based on phase alignment
                let interference_factor = phase_diff.cos().abs();

                // Combine with average value
                ((a + b) / 2.0).mul_add(interference_factor, (a + b) / 4.0)
            })
            .collect()
    }
}

impl Default for FractalInterference {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_interference() {
        let interference = FractalInterference::new();
        let data1 = vec![10.0, 20.0, 30.0];
        let data2 = vec![5.0, 10.0, 15.0];

        let result = interference.interfere(&data1, &data2);

        assert_eq!(result.len(), 3);
        assert!((result[0] - 7.5).abs() < 0.001);
        assert!((result[1] - 15.0).abs() < 0.001);
        assert!((result[2] - 22.5).abs() < 0.001);
    }

    #[test]
    fn test_subtract_interference() {
        let mut interference = FractalInterference::new();
        interference.set_mode(InterferenceMode::Subtract);

        let data1 = vec![10.0, 20.0, 30.0];
        let data2 = vec![5.0, 25.0, 30.0];

        let result = interference.interfere(&data1, &data2);

        assert!((result[0] - 5.0).abs() < 0.001);
        assert!((result[1] - 5.0).abs() < 0.001);
        assert!((result[2] - 0.0).abs() < 0.001);
    }

    #[test]
    fn test_wave_params() {
        let mut interference = FractalInterference::new();
        let params = WaveParameters {
            amplitude: 0.5,
            frequency: 2.0,
            phase: PI / 4.0,
        };
        interference.set_wave_params(params);
        interference.set_mode(InterferenceMode::Wave);

        let data1 = vec![100.0, 200.0];
        let data2 = vec![100.0, 200.0];

        let result = interference.interfere(&data1, &data2);
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_multiply_interference() {
        let mut interference = FractalInterference::new();
        interference.set_mode(InterferenceMode::Multiply);

        let data1 = vec![10.0, 20.0, 30.0];
        let data2 = vec![15.0, 30.0, 15.0];

        let result = interference.interfere(&data1, &data2);
        assert_eq!(result.len(), 3);
    }

    #[test]
    fn test_phase_interference() {
        let mut interference = FractalInterference::new();
        interference.set_mode(InterferenceMode::Phase);

        let data1 = vec![100.0, 200.0, 300.0];
        let data2 = vec![100.0, 200.0, 300.0];

        let result = interference.interfere(&data1, &data2);
        assert_eq!(result.len(), 3);
    }

    #[test]
    fn test_default_wave_params() {
        let params = WaveParameters::default();
        assert!((params.amplitude - 1.0).abs() < 0.001);
        assert!((params.frequency - 1.0).abs() < 0.001);
        assert!((params.phase - 0.0).abs() < 0.001);
    }

    #[test]
    #[should_panic(expected = "Fractal data must have the same dimensions")]
    fn test_mismatched_dimensions() {
        let interference = FractalInterference::new();
        let data1 = vec![1.0, 2.0, 3.0];
        let data2 = vec![1.0, 2.0];

        let _ = interference.interfere(&data1, &data2);
    }
}
