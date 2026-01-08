import React, { useRef, useEffect, useState } from 'react';

/**
 * 2D Canvas component for rendering fractal pixel data.
 */
function FractalCanvas({ pixels, width, height, onZoom, onPan }) {
  const canvasRef = useRef(null);
  const [isDragging, setIsDragging] = useState(false);
  const [lastPos, setLastPos] = useState({ x: 0, y: 0 });

  // Render pixels to canvas
  useEffect(() => {
    if (!canvasRef.current || !pixels) return;

    const canvas = canvasRef.current;
    const ctx = canvas.getContext('2d');

    // Create ImageData and set pixel data
    const imageData = new ImageData(pixels, width, height);
    ctx.putImageData(imageData, 0, 0);
  }, [pixels, width, height]);

  // Handle mouse wheel for zoom
  const handleWheel = (e) => {
    e.preventDefault();
    if (onZoom) {
      const rect = canvasRef.current.getBoundingClientRect();
      const x = e.clientX - rect.left;
      const y = e.clientY - rect.top;
      onZoom(e.deltaY > 0 ? -1 : 1, x, y);
    }
  };

  // Handle mouse down for pan
  const handleMouseDown = (e) => {
    setIsDragging(true);
    setLastPos({ x: e.clientX, y: e.clientY });
  };

  // Handle mouse move for pan
  const handleMouseMove = (e) => {
    if (!isDragging) return;

    const dx = e.clientX - lastPos.x;
    const dy = e.clientY - lastPos.y;

    if (onPan) {
      onPan(dx, dy);
    }

    setLastPos({ x: e.clientX, y: e.clientY });
  };

  // Handle mouse up
  const handleMouseUp = () => {
    setIsDragging(false);
  };

  // Handle mouse leave
  const handleMouseLeave = () => {
    setIsDragging(false);
  };

  // Touch handlers for mobile
  const handleTouchStart = (e) => {
    if (e.touches.length === 1) {
      const touch = e.touches[0];
      setIsDragging(true);
      setLastPos({ x: touch.clientX, y: touch.clientY });
    }
  };

  const handleTouchMove = (e) => {
    if (!isDragging || e.touches.length !== 1) return;

    const touch = e.touches[0];
    const dx = touch.clientX - lastPos.x;
    const dy = touch.clientY - lastPos.y;

    if (onPan) {
      onPan(dx, dy);
    }

    setLastPos({ x: touch.clientX, y: touch.clientY });
  };

  const handleTouchEnd = () => {
    setIsDragging(false);
  };

  return (
    <canvas
      ref={canvasRef}
      width={width}
      height={height}
      onWheel={handleWheel}
      onMouseDown={handleMouseDown}
      onMouseMove={handleMouseMove}
      onMouseUp={handleMouseUp}
      onMouseLeave={handleMouseLeave}
      onTouchStart={handleTouchStart}
      onTouchMove={handleTouchMove}
      onTouchEnd={handleTouchEnd}
      style={{
        cursor: isDragging ? 'grabbing' : 'grab',
        touchAction: 'none',
      }}
    />
  );
}

export default FractalCanvas;
