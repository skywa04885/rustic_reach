import React from "react";
import * as THREE from "three";

/**
 * Custom hook that creates and configures a memoized instance of THREE.Raycaster.
 * The raycaster is updated based on the mouse movement within a canvas element.
 *
 * @param canvasRef - A mutable ref object that holds the reference to the canvas element.
 * @param cameraRef - A mutable ref object that holds the reference to the THREE.PerspectiveCamera.
 * @returns The configured THREE.Raycaster instance.
 */
export const useRaycaster = (
  canvasRef: React.MutableRefObject<HTMLCanvasElement | null>,
  cameraRef: React.MutableRefObject<THREE.PerspectiveCamera | null>
): THREE.Raycaster => {
  // This code block uses the React.useMemo hook to create a memoized instance of THREE.Raycaster.
  // The raycaster is configured with a custom parameter for Line2, setting the threshold to 2.
  // The useMemo hook ensures that the raycaster is only created once, even if the component re-renders.
  const raycaster = React.useMemo(() => {
    const raycaster = new THREE.Raycaster();

    raycaster.params.Line2 = {
      threshold: 2,
    };

    return raycaster;
  }, []);

  // This callback function is triggered when the mouse moves within the canvas.
  // It calculates the position of the mouse pointer relative to the canvas and updates the raycaster accordingly.
  const onMouseMove = React.useCallback(
    (event: MouseEvent): void => {
      if (!cameraRef.current || !canvasRef.current) {
        return;
      }

      // Calculate the position of the mouse pointer relative to the canvas
      const canvasBounds = canvasRef.current.getBoundingClientRect();
      const pointer = new THREE.Vector2(
        ((event.clientX - canvasBounds.left) / canvasBounds.width) * 2 - 1,
        -((event.clientY - canvasBounds.top) / canvasBounds.height) * 2 + 1
      );

      // Update the raycaster with the new pointer position
      raycaster.setFromCamera(pointer, cameraRef.current);
    },
    [cameraRef, canvasRef, raycaster]
  );

  // This useEffect hook adds an event listener to the window object for the "mousemove" event.
  // When the mouse moves, it calls the onMouseMove callback function defined above.
  // The event listener is added when the component mounts and removed when the component unmounts.
  React.useEffect((): (() => void) => {
    window.addEventListener("mousemove", onMouseMove);

    return () => {
      window.removeEventListener("mousemove", onMouseMove);
    };
  });

  return raycaster;
};
