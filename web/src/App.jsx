import React, { useState, useEffect, useRef, useCallback, Suspense } from 'react';
import { Canvas } from '@react-three/fiber';
import { OrbitControls } from '@react-three/drei';
import FractalCanvas from './components/FractalCanvas';
import Controls from './components/Controls';
import Heightmap3D from './components/Heightmap3D';

// Default configuration
const DEFAULT_CONFIG = {
  width: 800,
  height: 600,
  maxIterations: 256,
  zoom: 1.0,
  centerX: -0.5,
  centerY: 0.0,
  fractalType: 'mandelbrot',
  juliaReal: -0.7,
  juliaImag: 0.27015,
  enableSecondFractal: false,
  secondFractalType: 'julia',
  interferenceMode: 'add',
  waveAmplitude: 1.0,
  waveFrequency: 1.0,
  wavePhase: 0.0,
};

// Julia set presets
const JULIA_PRESETS = [
  { name: 'Dendrite', real: -0.7, imag: 0.27015 },
  { name: 'Siegel Disk', real: -0.8, imag: 0.156 },
  { name: 'Douady Rabbit', real: 0.285, imag: 0.01 },
  { name: 'San Marco', real: -0.4, imag: 0.6 },
  { name: 'Spiral', real: -0.75, imag: 0.11 },
  { name: 'Galaxy', real: 0.285, imag: 0.535 },
];

function App() {
  const [config, setConfig] = useState(DEFAULT_CONFIG);
  const [viewMode, setViewMode] = useState('2d'); // '2d' or '3d'
  const [wasmModule, setWasmModule] = useState(null);
  const [renderer, setRenderer] = useState(null);
  const [pixels, setPixels] = useState(null);
  const [heightmap, setHeightmap] = useState(null);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState(null);
  const [renderTime, setRenderTime] = useState(0);

  // Load WASM module
  useEffect(() => {
    async function loadWasm() {
      try {
        setIsLoading(true);
        // In production, the WASM would be loaded from the pkg directory
        // For now, we'll use a fallback pure JS implementation
        console.log('Initializing fractal renderer...');

        // Simulate WASM loading - in production this would be:
        // const wasm = await import('../pkg/fractal_interference.js');
        // await wasm.default();

        // Using fallback JS implementation for demo
        setWasmModule({ loaded: true, fallback: true });
        setIsLoading(false);
      } catch (err) {
        console.error('Failed to load WASM:', err);
        setError('Failed to load WebAssembly module. Using fallback renderer.');
        setWasmModule({ loaded: true, fallback: true });
        setIsLoading(false);
      }
    }
    loadWasm();
  }, []);

  // Render fractal when config changes
  const renderFractal = useCallback(() => {
    if (!wasmModule) return;

    const startTime = performance.now();

    try {
      // Generate fractal using JS fallback or WASM
      const { width, height, maxIterations, zoom, centerX, centerY, fractalType } = config;
      const data = new Float64Array(width * height);

      const scale = 4.0 / (zoom * Math.min(width, height));

      for (let py = 0; py < height; py++) {
        for (let px = 0; px < width; px++) {
          const x = (px - width / 2) * scale + centerX;
          const y = (py - height / 2) * scale + centerY;

          let iterations;
          if (fractalType === 'julia') {
            iterations = calculateJulia(x, y, config.juliaReal, config.juliaImag, maxIterations);
          } else if (fractalType === 'burning_ship') {
            iterations = calculateBurningShip(x, y, maxIterations);
          } else if (fractalType === 'tricorn') {
            iterations = calculateTricorn(x, y, maxIterations);
          } else {
            iterations = calculateMandelbrot(x, y, maxIterations);
          }

          data[py * width + px] = iterations;
        }
      }

      // Apply interference if enabled
      let finalData = data;
      if (config.enableSecondFractal) {
        const data2 = new Float64Array(width * height);
        for (let py = 0; py < height; py++) {
          for (let px = 0; px < width; px++) {
            const x = (px - width / 2) * scale + centerX;
            const y = (py - height / 2) * scale + centerY;

            if (config.secondFractalType === 'julia') {
              data2[py * width + px] = calculateJulia(x, y, config.juliaReal, config.juliaImag, maxIterations);
            } else {
              data2[py * width + px] = calculateMandelbrot(x, y, maxIterations);
            }
          }
        }
        finalData = applyInterference(data, data2, config);
      }

      // Colorize
      const pixelData = colorize(finalData, maxIterations, width, height);
      setPixels(pixelData);

      // Generate heightmap for 3D view
      const hmap = new Uint8Array(width * height);
      for (let i = 0; i < finalData.length; i++) {
        hmap[i] = Math.floor((finalData[i] / maxIterations) * 255);
      }
      setHeightmap(hmap);

      const endTime = performance.now();
      setRenderTime(Math.round(endTime - startTime));
      setError(null);
    } catch (err) {
      console.error('Render error:', err);
      setError('Failed to render fractal: ' + err.message);
    }
  }, [config, wasmModule]);

  // Render when config changes
  useEffect(() => {
    if (wasmModule) {
      renderFractal();
    }
  }, [wasmModule, renderFractal]);

  const updateConfig = (key, value) => {
    setConfig(prev => ({ ...prev, [key]: value }));
  };

  const applyPreset = (preset) => {
    setConfig(prev => ({
      ...prev,
      fractalType: 'julia',
      juliaReal: preset.real,
      juliaImag: preset.imag,
    }));
  };

  const resetConfig = () => {
    setConfig(DEFAULT_CONFIG);
  };

  if (isLoading) {
    return (
      <div className="loading-overlay">
        <div className="loading-spinner"></div>
        <p>Loading Fractal Interference...</p>
      </div>
    );
  }

  return (
    <div className="app">
      <header className="header">
        <h1>Fractal Interference</h1>
        <span className="version">v0.1.0 {wasmModule?.fallback ? '(JS Fallback)' : '(WASM)'}</span>
      </header>

      <div className="main-content">
        <Controls
          config={config}
          updateConfig={updateConfig}
          presets={JULIA_PRESETS}
          applyPreset={applyPreset}
          resetConfig={resetConfig}
        />

        <div className="canvas-container">
          <div className="stats-bar">
            <div className="stat">
              <span className="stat-label">Size:</span>
              <span className="stat-value">{config.width}x{config.height}</span>
            </div>
            <div className="stat">
              <span className="stat-label">Iterations:</span>
              <span className="stat-value">{config.maxIterations}</span>
            </div>
            <div className="stat">
              <span className="stat-label">Zoom:</span>
              <span className="stat-value">{config.zoom.toFixed(2)}x</span>
            </div>
            <div className="stat">
              <span className="stat-label">Render Time:</span>
              <span className="stat-value">{renderTime}ms</span>
            </div>
          </div>

          <div className="canvas-wrapper">
            {viewMode === '2d' ? (
              <FractalCanvas
                pixels={pixels}
                width={config.width}
                height={config.height}
                onZoom={(delta, x, y) => {
                  const newZoom = config.zoom * (delta > 0 ? 1.2 : 0.8);
                  updateConfig('zoom', Math.max(0.1, Math.min(1000, newZoom)));
                }}
                onPan={(dx, dy) => {
                  const scale = 4.0 / (config.zoom * Math.min(config.width, config.height));
                  updateConfig('centerX', config.centerX - dx * scale);
                  updateConfig('centerY', config.centerY - dy * scale);
                }}
              />
            ) : (
              <div style={{ width: '100%', height: '100%' }}>
                <Canvas camera={{ position: [0, 2, 3], fov: 60 }}>
                  <ambientLight intensity={0.5} />
                  <directionalLight position={[5, 5, 5]} intensity={1} />
                  <Suspense fallback={null}>
                    <Heightmap3D
                      heightmap={heightmap}
                      width={config.width}
                      height={config.height}
                    />
                  </Suspense>
                  <OrbitControls enablePan enableZoom enableRotate />
                </Canvas>
              </div>
            )}
          </div>

          <div className="view-tabs">
            <button
              className={`view-tab ${viewMode === '2d' ? 'active' : ''}`}
              onClick={() => setViewMode('2d')}
            >
              2D View
            </button>
            <button
              className={`view-tab ${viewMode === '3d' ? 'active' : ''}`}
              onClick={() => setViewMode('3d')}
            >
              3D View
            </button>
          </div>
        </div>
      </div>

      {error && (
        <div className="error-message">
          {error}
        </div>
      )}
    </div>
  );
}

// Fractal calculation functions (JS fallback)
function calculateMandelbrot(x, y, maxIter) {
  let zr = 0, zi = 0;
  let zr2 = 0, zi2 = 0;
  let i = 0;

  while (i < maxIter && zr2 + zi2 <= 4) {
    zi = 2 * zr * zi + y;
    zr = zr2 - zi2 + x;
    zr2 = zr * zr;
    zi2 = zi * zi;
    i++;
  }

  if (i === maxIter) return maxIter;

  // Smooth coloring
  const logZn = Math.log(zr2 + zi2) / 2;
  const nu = Math.log(logZn / Math.LN2) / Math.LN2;
  return i + 1 - nu;
}

function calculateJulia(x, y, cr, ci, maxIter) {
  let zr = x, zi = y;
  let zr2 = zr * zr, zi2 = zi * zi;
  let i = 0;

  while (i < maxIter && zr2 + zi2 <= 4) {
    zi = 2 * zr * zi + ci;
    zr = zr2 - zi2 + cr;
    zr2 = zr * zr;
    zi2 = zi * zi;
    i++;
  }

  if (i === maxIter) return maxIter;

  const logZn = Math.log(zr2 + zi2) / 2;
  const nu = Math.log(logZn / Math.LN2) / Math.LN2;
  return i + 1 - nu;
}

function calculateBurningShip(x, y, maxIter) {
  let zr = 0, zi = 0;
  let i = 0;

  while (i < maxIter && zr * zr + zi * zi <= 4) {
    const temp = zr * zr - zi * zi + x;
    zi = Math.abs(2 * zr * zi) + y;
    zr = temp;
    i++;
  }

  if (i === maxIter) return maxIter;

  const logZn = Math.log(zr * zr + zi * zi) / 2;
  const nu = Math.log(logZn / Math.LN2) / Math.LN2;
  return i + 1 - nu;
}

function calculateTricorn(x, y, maxIter) {
  let zr = 0, zi = 0;
  let i = 0;

  while (i < maxIter && zr * zr + zi * zi <= 4) {
    const temp = zr * zr - zi * zi + x;
    zi = -2 * zr * zi + y;
    zr = temp;
    i++;
  }

  if (i === maxIter) return maxIter;

  const logZn = Math.log(zr * zr + zi * zi) / 2;
  const nu = Math.log(logZn / Math.LN2) / Math.LN2;
  return i + 1 - nu;
}

function applyInterference(data1, data2, config) {
  const result = new Float64Array(data1.length);
  const { interferenceMode, waveAmplitude, waveFrequency, wavePhase } = config;

  for (let i = 0; i < data1.length; i++) {
    switch (interferenceMode) {
      case 'subtract':
        result[i] = Math.abs(data1[i] - data2[i]);
        break;
      case 'multiply': {
        const max1 = Math.max(...data1);
        const max2 = Math.max(...data2);
        if (max1 === 0 || max2 === 0) {
          result[i] = 0;
        } else {
          result[i] = (data1[i] / max1) * (data2[i] / max2) * Math.max(max1, max2);
        }
        break;
      }
      case 'wave': {
        const wave1 = waveAmplitude * Math.sin(data1[i] * waveFrequency * Math.PI / 180);
        const wave2 = waveAmplitude * Math.sin(data2[i] * waveFrequency * Math.PI / 180 + wavePhase);
        const combined = wave1 + wave2;
        result[i] = (combined + 2 * waveAmplitude) * Math.max(data1[i], data2[i]) / (4 * waveAmplitude);
        break;
      }
      case 'phase': {
        const phase1 = (data1[i] * waveFrequency * Math.PI / 180) % (2 * Math.PI);
        const phase2 = (data2[i] * waveFrequency * Math.PI / 180) % (2 * Math.PI);
        const phaseDiff = Math.abs(phase1 - phase2);
        const factor = Math.abs(Math.cos(phaseDiff));
        result[i] = (data1[i] + data2[i]) / 2 * factor + (data1[i] + data2[i]) / 4;
        break;
      }
      default: // 'add'
        result[i] = (data1[i] + data2[i]) / 2;
    }
  }

  return result;
}

function colorize(data, maxIter, width, height) {
  const pixels = new Uint8ClampedArray(width * height * 4);

  for (let i = 0; i < data.length; i++) {
    const value = data[i];
    const idx = i * 4;

    if (value >= maxIter) {
      pixels[idx] = 0;
      pixels[idx + 1] = 0;
      pixels[idx + 2] = 0;
    } else {
      const normalized = value / maxIter;
      const hue = normalized * 360;
      const [r, g, b] = hsvToRgb(hue, 0.8, 1 - normalized * 0.3);
      pixels[idx] = r;
      pixels[idx + 1] = g;
      pixels[idx + 2] = b;
    }
    pixels[idx + 3] = 255;
  }

  return pixels;
}

function hsvToRgb(h, s, v) {
  h = h % 360;
  const c = v * s;
  const x = c * (1 - Math.abs((h / 60) % 2 - 1));
  const m = v - c;

  let r, g, b;
  if (h < 60) { r = c; g = x; b = 0; }
  else if (h < 120) { r = x; g = c; b = 0; }
  else if (h < 180) { r = 0; g = c; b = x; }
  else if (h < 240) { r = 0; g = x; b = c; }
  else if (h < 300) { r = x; g = 0; b = c; }
  else { r = c; g = 0; b = x; }

  return [
    Math.round((r + m) * 255),
    Math.round((g + m) * 255),
    Math.round((b + m) * 255)
  ];
}

export default App;
