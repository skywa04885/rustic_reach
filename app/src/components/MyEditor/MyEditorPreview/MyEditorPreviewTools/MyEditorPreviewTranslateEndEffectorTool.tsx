import { Line } from "@react-three/drei";
import React from "react";
import * as THREE from "three";
import { Line2 } from "three-stdlib";
import { useEditorPreviewContext } from "../../MyEditorPreview";

interface IAxisProps {
  axis: THREE.Vector3;
  color: THREE.Color;
  scale: number;
}

const Axis = ({ axis, scale, color }: IAxisProps) => {
  const [selected, setSelected] = React.useState(false);

  const { raycaster } = useEditorPreviewContext();

  const lineRef: React.MutableRefObject<Line2 | null> = React.useRef(null);

  // Calculate the points for the line segment representing the axis.
  // The points consist of the origin (0, 0, 0) and the endpoint of the axis, which is obtained by
  // normalizing the axis vector and scaling it by the given scale.
  const points = React.useMemo(
    (): THREE.Vector3[] => [
      new THREE.Vector3(0, 0, 0),
      axis.clone().normalize().multiplyScalar(scale),
    ],
    [axis, scale]
  );

  const onMouseDown = React.useCallback((event: MouseEvent): void => {}, []);

  const onMouseUp = React.useCallback((event: MouseEvent): void => {}, []);

  const onMouseMove = React.useCallback(
    (event: MouseEvent): void => {
      if (!lineRef.current) {
        return;
      }

      const intersections = raycaster.intersectObject(lineRef.current, false);

      const shouldBeSelected = intersections.length > 0;

      if (shouldBeSelected && !selected) {
        setSelected(true);
      } else if (!shouldBeSelected && selected) {
        setSelected(false);
      }
    },
    [selected, setSelected, lineRef, raycaster]
  );

  // Add event listeners for mouse events (mousedown, mouseup, mousemove) when the component mounts.
  React.useEffect((): (() => void) => {
    window.addEventListener("mousedown", onMouseDown);
    window.addEventListener("mouseup", onMouseUp);
    window.addEventListener("mousemove", onMouseMove);

    // Remove event listeners when the component unmounts.
    return () => {
      window.removeEventListener("mousedown", onMouseDown);
      window.removeEventListener("mouseup", onMouseUp);
      window.removeEventListener("mousemove", onMouseMove);
    };
  }, []);

  return (
    <Line
      ref={lineRef}
      points={points}
      color={color}
      opacity={selected ? 1.0 : 0.7}
      transparent={true}
      lineWidth={2}
    />
  );
};

export interface IMyEditorPreviewTranslateEndEffectorToolProps {
  position: THREE.Vector3;
  scale: number;
}

export const MyEditorPreviewTranslateEndEffectorTool = ({
  position,
  scale,
}: IMyEditorPreviewTranslateEndEffectorToolProps) => {
  const xAxisColor = React.useMemo(() => new THREE.Color("red"), []);
  const yAxisColor = React.useMemo(() => new THREE.Color("green"), []);
  const zAxisColor = React.useMemo(() => new THREE.Color("blue"), []);

  const xAxis = React.useMemo(() => new THREE.Vector3(1, 0, 0), []);
  const yAxis = React.useMemo(() => new THREE.Vector3(0, 1, 0), []);
  const zAxis = React.useMemo(() => new THREE.Vector3(0, 0, 1), []);

  return (
    <group position={position}>
      <Axis axis={xAxis} scale={scale} color={xAxisColor} />
      <Axis axis={yAxis} scale={scale} color={yAxisColor} />
      <Axis axis={zAxis} scale={scale} color={zAxisColor} />
    </group>
  );
};
