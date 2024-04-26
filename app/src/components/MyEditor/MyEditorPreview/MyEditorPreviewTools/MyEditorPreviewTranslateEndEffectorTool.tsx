import { Line } from "@react-three/drei";
import React from "react";
import * as THREE from "three";
import { Line2 } from "three-stdlib";
import { useEditorPreviewContext } from "../../MyEditorPreview";

interface IAxisProps {
  origin: THREE.Vector3;
  setOrigin: React.Dispatch<React.SetStateAction<THREE.Vector3>>;
  axis: THREE.Vector3;
  color: THREE.Color;
  scale: number;
}

interface IAxisDragState {
  startOrigin: THREE.Vector3,
}

const Axis = ({ origin, axis, scale, color, setOrigin }: IAxisProps) => {
  const [hoveringPoint, setHoveringPoint] =
    React.useState<THREE.Vector3 | null>(null);

  const { raycaster, isDragging, setIsDragging, camera } =
    useEditorPreviewContext();

  const [startOrigin, setStartOrigin] = React.useState<THREE.Vector3 | null>();

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
    (_event: MouseEvent): void => {
      if (!hoveringPoint) {
        return;
      }

      setIsDragging(true);
      setStartOrigin(origin.clone());
    },
    [setIsDragging, hoveringPoint, setStartOrigin, origin]
  );

  const onMouseUp = React.useCallback(
    (_event: MouseEvent): void => {
      if (!isDragging || !hoveringPoint) {
        return;
      }

      console.log("a");

      if (!camera.current || !hoveringPoint || !startOrigin) {
        return;
      }

        // Create the a new ray in the direction of the axis.
        const ray = new THREE.Ray(origin, axis);
        const closestPointOnRay = ray.closestPointToPoint(
          camera.current.position,
          new THREE.Vector3()
        );
        const planeNormal = camera.current.position
          .clone()
          .sub(closestPointOnRay)
          .normalize();
        const plane = new THREE.Plane().setFromNormalAndCoplanarPoint(
          planeNormal,
          origin
        );

        const intersect = raycaster.ray.intersectPlane(
          plane,
          new THREE.Vector3()
        );
        if (!intersect) {
          return;
        }

        const newPoint = ray.closestPointToPoint(
          intersect,
          new THREE.Vector3()
        );

        const deltaPoint = newPoint.sub(hoveringPoint);

        console.log(deltaPoint);
        setOrigin(startOrigin.clone().add(deltaPoint));

      setIsDragging(false);
    },
    [isDragging, setIsDragging,       isDragging,
      raycaster,
      setHoveringPoint,
      hoveringPoint,
      camera,
      origin,
      axis,
      startOrigin,
      setOrigin]
  );

  const onMouseMove = React.useCallback(
    (_event: MouseEvent): void => {
      // Check if the line reference is set.
      if (!lineRef.current) {
        return;
      }

      if (!isDragging) {
        const intersecting = raycaster.intersectObject(lineRef.current, false);

        if (intersecting.length === 1) {
          setHoveringPoint(intersecting[0].point);
        } else {
          setHoveringPoint(null);
        }
      } else {

      }
    },
    [
      lineRef,
      isDragging,
      raycaster,
      setHoveringPoint,
      hoveringPoint,
      camera,
      origin,
      axis,
      startOrigin,
      setOrigin
    ]
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
        opacity={hoveringPoint !== null ? 1.0 : 0.7}
        transparent={true}
        lineWidth={2}
      />
    </>
  );
};

export interface IMyEditorPreviewTranslateEndEffectorToolProps {
  origin: THREE.Vector3;
  scale: number;
}

export const MyEditorPreviewTranslateEndEffectorTool = ({
  origin: outerOrigin,
  scale,
}: IMyEditorPreviewTranslateEndEffectorToolProps) => {
  const [origin, setOrigin] = React.useState<THREE.Vector3>(outerOrigin);

  const xAxisColor = React.useMemo(() => new THREE.Color("red"), []);
  const yAxisColor = React.useMemo(() => new THREE.Color("green"), []);
  const zAxisColor = React.useMemo(() => new THREE.Color("blue"), []);

  const xAxis = React.useMemo(() => new THREE.Vector3(1, 0, 0), []);
  const yAxis = React.useMemo(() => new THREE.Vector3(0, 1, 0), []);
  const zAxis = React.useMemo(() => new THREE.Vector3(0, 0, 1), []);

  return (
    <group>
      <Axis
        origin={origin}
        setOrigin={setOrigin}
        axis={xAxis}
        scale={scale}
        color={xAxisColor}
      />
      <Axis
        origin={origin}
        setOrigin={setOrigin}
        axis={yAxis}
        scale={scale}
        color={yAxisColor}
      />
      <Axis
        origin={origin}
        setOrigin={setOrigin}
        axis={zAxis}
        scale={scale}
        color={zAxisColor}
      />
    </group>
  );
};
