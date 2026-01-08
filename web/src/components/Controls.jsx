import React from 'react';

/**
 * Control panel for fractal parameters.
 */
function Controls({ config, updateConfig, presets, applyPreset, resetConfig }) {
  return (
    <aside className="controls">
      {/* Fractal Type */}
      <section className="control-section">
        <h3>Fractal Type</h3>
        <div className="control-group">
          <label>Primary Fractal</label>
          <select
            value={config.fractalType}
            onChange={(e) => updateConfig('fractalType', e.target.value)}
          >
            <option value="mandelbrot">Mandelbrot Set</option>
            <option value="julia">Julia Set</option>
            <option value="burning_ship">Burning Ship</option>
            <option value="tricorn">Tricorn</option>
          </select>
        </div>

        {/* Julia Parameters */}
        {config.fractalType === 'julia' && (
          <>
            <div className="control-group">
              <label>Julia C (Real): {config.juliaReal.toFixed(4)}</label>
              <input
                type="range"
                min="-2"
                max="2"
                step="0.001"
                value={config.juliaReal}
                onChange={(e) => updateConfig('juliaReal', parseFloat(e.target.value))}
              />
            </div>
            <div className="control-group">
              <label>Julia C (Imaginary): {config.juliaImag.toFixed(4)}</label>
              <input
                type="range"
                min="-2"
                max="2"
                step="0.001"
                value={config.juliaImag}
                onChange={(e) => updateConfig('juliaImag', parseFloat(e.target.value))}
              />
            </div>

            {/* Julia Presets */}
            <div className="control-group">
              <label>Presets</label>
              <div className="presets">
                {presets.map((preset, idx) => (
                  <button
                    key={idx}
                    className="preset-btn"
                    onClick={() => applyPreset(preset)}
                  >
                    {preset.name}
                  </button>
                ))}
              </div>
            </div>
          </>
        )}
      </section>

      {/* View Parameters */}
      <section className="control-section">
        <h3>View</h3>
        <div className="control-group">
          <label>Zoom: {config.zoom.toFixed(2)}x</label>
          <input
            type="range"
            min="0.1"
            max="100"
            step="0.1"
            value={config.zoom}
            onChange={(e) => updateConfig('zoom', parseFloat(e.target.value))}
          />
        </div>
        <div className="control-group">
          <label>Center X: {config.centerX.toFixed(4)}</label>
          <input
            type="range"
            min="-3"
            max="3"
            step="0.01"
            value={config.centerX}
            onChange={(e) => updateConfig('centerX', parseFloat(e.target.value))}
          />
        </div>
        <div className="control-group">
          <label>Center Y: {config.centerY.toFixed(4)}</label>
          <input
            type="range"
            min="-3"
            max="3"
            step="0.01"
            value={config.centerY}
            onChange={(e) => updateConfig('centerY', parseFloat(e.target.value))}
          />
        </div>
      </section>

      {/* Rendering Parameters */}
      <section className="control-section">
        <h3>Rendering</h3>
        <div className="control-group">
          <label>Max Iterations: {config.maxIterations}</label>
          <input
            type="range"
            min="50"
            max="1000"
            step="10"
            value={config.maxIterations}
            onChange={(e) => updateConfig('maxIterations', parseInt(e.target.value))}
          />
        </div>
        <div className="control-group">
          <label>Width: {config.width}px</label>
          <input
            type="range"
            min="200"
            max="1920"
            step="100"
            value={config.width}
            onChange={(e) => updateConfig('width', parseInt(e.target.value))}
          />
        </div>
        <div className="control-group">
          <label>Height: {config.height}px</label>
          <input
            type="range"
            min="150"
            max="1080"
            step="50"
            value={config.height}
            onChange={(e) => updateConfig('height', parseInt(e.target.value))}
          />
        </div>
      </section>

      {/* Interference */}
      <section className="control-section">
        <h3>Interference</h3>
        <div className="control-group toggle-group">
          <label className="toggle">
            <input
              type="checkbox"
              checked={config.enableSecondFractal}
              onChange={(e) => updateConfig('enableSecondFractal', e.target.checked)}
            />
            <span className="toggle-slider"></span>
          </label>
          <span>Enable Second Fractal</span>
        </div>

        {config.enableSecondFractal && (
          <>
            <div className="control-group">
              <label>Second Fractal</label>
              <select
                value={config.secondFractalType}
                onChange={(e) => updateConfig('secondFractalType', e.target.value)}
              >
                <option value="mandelbrot">Mandelbrot Set</option>
                <option value="julia">Julia Set</option>
                <option value="burning_ship">Burning Ship</option>
                <option value="tricorn">Tricorn</option>
              </select>
            </div>

            <div className="control-group">
              <label>Interference Mode</label>
              <select
                value={config.interferenceMode}
                onChange={(e) => updateConfig('interferenceMode', e.target.value)}
              >
                <option value="add">Add (Constructive)</option>
                <option value="subtract">Subtract (Destructive)</option>
                <option value="multiply">Multiply</option>
                <option value="wave">Wave</option>
                <option value="phase">Phase</option>
              </select>
            </div>

            {(config.interferenceMode === 'wave' || config.interferenceMode === 'phase') && (
              <>
                <div className="control-group">
                  <label>Wave Amplitude: {config.waveAmplitude.toFixed(2)}</label>
                  <input
                    type="range"
                    min="0.1"
                    max="2"
                    step="0.1"
                    value={config.waveAmplitude}
                    onChange={(e) => updateConfig('waveAmplitude', parseFloat(e.target.value))}
                  />
                </div>
                <div className="control-group">
                  <label>Wave Frequency: {config.waveFrequency.toFixed(2)}</label>
                  <input
                    type="range"
                    min="0.1"
                    max="5"
                    step="0.1"
                    value={config.waveFrequency}
                    onChange={(e) => updateConfig('waveFrequency', parseFloat(e.target.value))}
                  />
                </div>
                <div className="control-group">
                  <label>Wave Phase: {config.wavePhase.toFixed(2)}</label>
                  <input
                    type="range"
                    min="0"
                    max="6.28"
                    step="0.1"
                    value={config.wavePhase}
                    onChange={(e) => updateConfig('wavePhase', parseFloat(e.target.value))}
                  />
                </div>
              </>
            )}
          </>
        )}
      </section>

      {/* Actions */}
      <section className="control-section">
        <div className="button-group">
          <button onClick={resetConfig}>Reset All</button>
          <button
            className="secondary"
            onClick={() => {
              // Center on interesting Mandelbrot location
              updateConfig('centerX', -0.7435669);
              updateConfig('centerY', 0.1314023);
              updateConfig('zoom', 50);
              updateConfig('fractalType', 'mandelbrot');
            }}
          >
            Deep Zoom
          </button>
        </div>
      </section>
    </aside>
  );
}

export default Controls;
