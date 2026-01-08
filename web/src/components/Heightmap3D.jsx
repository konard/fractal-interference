import React, { useMemo } from 'react';
import * as THREE from 'three';

/**
 * 3D visualization component that renders fractals as a height map.
 */
function Heightmap3D({ heightmap, width, height }) {
  const geometry = useMemo(() => {
    if (!heightmap || !width || !height) return null;

    // Downsample for performance (max 200x200 vertices)
    const maxSize = 200;
    const scaleX = Math.ceil(width / maxSize);
    const scaleY = Math.ceil(height / maxSize);
    const sampledWidth = Math.ceil(width / scaleX);
    const sampledHeight = Math.ceil(height / scaleY);

    const geo = new THREE.PlaneGeometry(
      2, // width in world units
      2 * (sampledHeight / sampledWidth), // height proportional
      sampledWidth - 1,
      sampledHeight - 1
    );

    const positions = geo.attributes.position.array;
    const colors = new Float32Array(positions.length);

    // Apply heightmap and colors
    for (let i = 0; i < sampledHeight; i++) {
      for (let j = 0; j < sampledWidth; j++) {
        const vertexIdx = (i * sampledWidth + j) * 3;

        // Sample from heightmap
        const srcX = Math.min(j * scaleX, width - 1);
        const srcY = Math.min(i * scaleY, height - 1);
        const srcIdx = srcY * width + srcX;

        const heightValue = heightmap[srcIdx] / 255;

        // Set Z (height)
        positions[vertexIdx + 2] = heightValue * 0.5;

        // Set vertex color based on height (gradient from blue to red)
        const hue = (1 - heightValue) * 0.7; // Blue (0.7) to Red (0)
        const rgb = hslToRgb(hue, 0.8, 0.5);
        colors[vertexIdx] = rgb[0];
        colors[vertexIdx + 1] = rgb[1];
        colors[vertexIdx + 2] = rgb[2];
      }
    }

    geo.setAttribute('color', new THREE.BufferAttribute(colors, 3));
    geo.computeVertexNormals();

    return geo;
  }, [heightmap, width, height]);

  if (!geometry) {
    return null;
  }

  return (
    <mesh rotation={[-Math.PI / 2, 0, 0]} position={[0, 0, 0]}>
      <primitive object={geometry} attach="geometry" />
      <meshStandardMaterial
        vertexColors
        side={THREE.DoubleSide}
        flatShading={false}
      />
    </mesh>
  );
}

/**
 * Convert HSL to RGB
 */
function hslToRgb(h, s, l) {
  let r, g, b;

  if (s === 0) {
    r = g = b = l;
  } else {
    const hue2rgb = (p, q, t) => {
      if (t < 0) t += 1;
      if (t > 1) t -= 1;
      if (t < 1/6) return p + (q - p) * 6 * t;
      if (t < 1/2) return q;
      if (t < 2/3) return p + (q - p) * (2/3 - t) * 6;
      return p;
    };

    const q = l < 0.5 ? l * (1 + s) : l + s - l * s;
    const p = 2 * l - q;
    r = hue2rgb(p, q, h + 1/3);
    g = hue2rgb(p, q, h);
    b = hue2rgb(p, q, h - 1/3);
  }

  return [r, g, b];
}

export default Heightmap3D;
