import { Line, Plane } from "@react-three/drei";
import React from "react";
import * as THREE from "three";
import { Line2 } from "three-stdlib";
import { useEditorPreviewContext } from "../../MyEditorPreview";

interface IAxisProps {
  origin: THREE.Vector3;
  axis: THREE.Vector3;
  ortho: THREE.Vector3;
  color: THREE.Color;
  scale: number;
}

const Axis = ({ origin, axis, ortho, scale, color }: IAxisProps) => {
  const [isHovering, setIsHovering] = React.useState(false);
  const [dragStartVector, setDragStartVector] =
    React.useState<THREE.Vector2 | null>(null);

  const { raycaster, setIsDragging, camera } = useEditorPreviewContext();

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

  const onMouseDown = React.useCallback(
    (event: MouseEvent): void => {
      if (isHovering) {
        setDragStartVector(new THREE.Vector2(event.clientX, event.clientY));
        setIsDragging(true);
      }
    },
    [isHovering]
  );

  const onMouseUp = React.useCallback(
    (event: MouseEvent): void => {
      if (dragStartVector === null) {
        return;
      }

      console.log("a");

      setDragStartVector(null);
      setIsDragging(false);
    },
    [dragStartVector, setDragStartVector]
  );

  const onMouseMove = React.useCallback(
    (event: MouseEvent): void => {
      // Check if the line reference is set.
      if (!lineRef.current) {
        return;
      }

      if (dragStartVector == null) {
        const intersecting =
          raycaster.intersectObject(lineRef.current, false).length > 0;

        if (intersecting && !isHovering) {
          setIsHovering(true);
        } else if (!intersecting && isHovering) {
          setIsHovering(false);
        }
      } else {
        if (!camera.current) {
          return;
        }

        // Create the a new ray in the direction of the axis.
        const ray = new THREE.Ray(origin, axis);
        const closestPointOnRay = ray.closestPointToPoint(camera.current.position, new THREE.Vector3());
        const planeNormal = camera.current.position.clone().sub(closestPointOnRay).normalize();
        const plane = new THREE.Plane().setFromNormalAndCoplanarPoint(planeNormal, origin);

        const intersect =  raycaster.ray.intersectPlane(plane, new THREE.Vector3());
        console.log(intersect);
      }
    },
    [isHovering, setIsHovering, lineRef, raycaster, dragStartVector, origin, camera]
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
  }, [onMouseDown, onMouseUp, onMouseMove]);

  return (
    <>
      <Line
      position={origin}
        ref={lineRef}
        points={points}
        color={color}
        opacity={isHovering ? 1.0 : 0.7}
        transparent={true}
        lineWidth={2}
      />
    </>
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

  const xAxisOrtho = React.useMemo(() => new THREE.Vector3(0, 1, 0), []);
  const yAxisOrtho = React.useMemo(() => new THREE.Vector3(1, 0, 0), []);
  const zAxisOrtho = React.useMemo(() => new THREE.Vector3(0, 1, 0), []);

  return (
    <group>
      <Axis origin={position} axis={xAxis} ortho={xAxisOrtho} scale={scale} color={xAxisColor} />
      <Axis origin={position} axis={yAxis} ortho={yAxisOrtho} scale={scale} color={yAxisColor} />
      <Axis origin={position} axis={zAxis} ortho={zAxisOrtho} scale={scale} color={zAxisColor} />
    </group>
  );
};
