# Fractal Interference

A WebAssembly-powered fractal interference visualization using Rust and React.js.

[![CI/CD Pipeline](https://github.com/konard/fractal-interference/workflows/CI%2FCD%20Pipeline/badge.svg)](https://github.com/konard/fractal-interference/actions)
[![Deploy to GitHub Pages](https://github.com/konard/fractal-interference/workflows/Deploy%20to%20GitHub%20Pages/badge.svg)](https://github.com/konard/fractal-interference/actions)
[![Rust Version](https://img.shields.io/badge/rust-1.70%2B-blue.svg)](https://www.rust-lang.org/)
[![License: Unlicense](https://img.shields.io/badge/license-Unlicense-blue.svg)](http://unlicense.org/)

## Live Demo

Visit the live demo at: [https://konard.github.io/fractal-interference/](https://konard.github.io/fractal-interference/)

## Overview

This project explores the concept of fractal interference - combining multiple fractals using wave-like interference patterns. The hypothesis is that by interfering fractals like waves, we can create any desired shape or pattern.

### Features

- **Multiple Fractal Types**:
  - Mandelbrot Set
  - Julia Sets (with customizable parameters)
  - Burning Ship Fractal
  - Tricorn (Mandelbar)

- **Interference Modes**:
  - **Add**: Constructive interference (average of two fractals)
  - **Subtract**: Destructive interference (difference between fractals)
  - **Multiply**: Multiplicative combination
  - **Wave**: Sine wave-based interference with amplitude, frequency, and phase control
  - **Phase**: Phase difference-based interference patterns

- **Visualization**:
  - 2D canvas rendering with smooth coloring
  - 3D height map visualization using WebGL/Three.js
  - Real-time pan and zoom
  - Interactive parameter controls

- **Technology Stack**:
  - **Rust + WebAssembly**: High-performance fractal computation
  - **React.js**: Modern UI framework
  - **Three.js**: 3D visualization
  - **Vite**: Fast build tooling

## Quick Start

### Prerequisites

- Rust 1.70+ with `wasm32-unknown-unknown` target
- Node.js 18+
- wasm-pack

### Development Setup

```bash
# Clone the repository
git clone https://github.com/konard/fractal-interference.git
cd fractal-interference

# Install Rust wasm target
rustup target add wasm32-unknown-unknown

# Install wasm-pack
cargo install wasm-pack

# Build WebAssembly module
wasm-pack build --target web --out-dir web/pkg

# Install web dependencies
cd web
npm install

# Start development server
npm run dev
```

### Running Tests

```bash
# Run Rust tests
cargo test

# Run tests with verbose output
cargo test --verbose

# Run WebAssembly tests
wasm-pack test --headless --chrome
```

### Building for Production

```bash
# Build WASM with optimizations
wasm-pack build --target web --release --out-dir web/pkg

# Build web app
cd web
npm run build
```

The production build will be in `web/dist/`.

## Project Structure

```
fractal-interference/
├── src/
│   ├── lib.rs              # Library entry point with WASM bindings
│   ├── main.rs             # CLI tool for testing
│   ├── fractals.rs         # Fractal generation algorithms
│   └── interference.rs     # Wave interference implementation
├── web/
│   ├── src/
│   │   ├── App.jsx         # Main React component
│   │   ├── components/     # UI components
│   │   └── styles.css      # Styling
│   ├── index.html          # HTML entry point
│   ├── package.json        # Node dependencies
│   └── vite.config.js      # Vite configuration
├── tests/
│   └── integration_test.rs # Integration tests
├── examples/
│   └── basic_usage.rs      # Usage examples
├── .github/workflows/
│   ├── release.yml         # CI/CD pipeline
│   └── deploy.yml          # GitHub Pages deployment
├── Cargo.toml              # Rust dependencies
└── README.md               # This file
```

## Usage

### CLI Tool

```bash
# Run the CLI demo
cargo run

# Run with features
cargo run --features cli
```

### Rust Library

```rust
use fractal_interference::{FractalConfig, FractalRenderer};

// Create configuration
let mut config = FractalConfig::mandelbrot_default();
config.width = 800;
config.height = 600;

// Create renderer
let mut renderer = FractalRenderer::new(config);

// Render Mandelbrot set
let pixels = renderer.render();

// Enable interference with Julia set
renderer.enable_second_fractal("julia");
renderer.set_interference_mode("wave");
renderer.set_wave_params(1.0, 2.0, 0.5);

// Render interference pattern
let interference_pixels = renderer.render();
```

### Web Interface

The web interface provides interactive controls for:

1. **Fractal Selection**: Choose primary and secondary fractal types
2. **Julia Parameters**: Adjust c values for Julia sets
3. **View Controls**: Zoom, pan, and center position
4. **Rendering**: Iteration count and resolution
5. **Interference**: Mode selection and wave parameters
6. **Presets**: Quick access to interesting Julia set parameters

## The Concept

Fractals are mathematical objects with self-similar patterns at every scale. This project explores treating fractals as waves that can interfere with each other:

1. **Constructive Interference**: When fractal values align, they amplify
2. **Destructive Interference**: When values oppose, they cancel out
3. **Wave Interference**: Applying sine functions to create ripple-like patterns
4. **Phase Interference**: Using phase differences to create unique patterns

The hypothesis is that through careful selection of fractal types and interference parameters, any desired shape can be approximated - similar to how Fourier series can represent any periodic function.

## API Reference

### FractalConfig

| Field | Type | Description |
|-------|------|-------------|
| width | u32 | Image width in pixels |
| height | u32 | Image height in pixels |
| max_iterations | u32 | Maximum escape iterations |
| zoom | f64 | Zoom level |
| center_x | f64 | X coordinate of center |
| center_y | f64 | Y coordinate of center |

### FractalRenderer Methods

| Method | Description |
|--------|-------------|
| `new(config)` | Create new renderer |
| `set_fractal_type(type)` | Set primary fractal |
| `set_julia_params(real, imag)` | Set Julia c parameter |
| `enable_second_fractal(type)` | Enable interference |
| `set_interference_mode(mode)` | Set interference mode |
| `set_wave_params(amp, freq, phase)` | Set wave parameters |
| `render()` | Generate RGBA pixels |
| `render_3d_heightmap()` | Generate height values |

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

### Development Workflow

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/my-feature`
3. Make your changes and add tests
4. Run quality checks: `cargo fmt && cargo clippy && cargo test`
5. Add a changelog fragment
6. Commit your changes
7. Push and create a Pull Request

## License

[Unlicense](LICENSE) - Public Domain

This is free and unencumbered software released into the public domain.

## Acknowledgments

- Inspired by the mathematical beauty of fractals
- Built on the Rust WebAssembly ecosystem
- React Three Fiber for 3D visualization

## Resources

- [Mandelbrot Set - Wikipedia](https://en.wikipedia.org/wiki/Mandelbrot_set)
- [Julia Set - Wikipedia](https://en.wikipedia.org/wiki/Julia_set)
- [Wave Interference - Wikipedia](https://en.wikipedia.org/wiki/Wave_interference)
- [Rust WebAssembly Book](https://rustwasm.github.io/docs/book/)
