import React from "react";
import { useArmContext } from "../../../providers/MyArmProvider";
import { BufferGeometry, Color, Vector3 } from "three";
import { Html } from "@react-three/drei";
import { Typography } from "@mui/material";

const LINE_COLOR: Color = new Color("#ff4444");
const LINE_WIDTH: number = 2;

const POINT_COLOR: Color = new Color("#ff1111");
const POINT_SIZE: number = 1;

interface IAngleProps {
  angle: number;
  position: Vector3;
}

const Angle = ({ angle, position }: IAngleProps): React.ReactElement => {
  return (
    <mesh position={position}>
      <Html style={{ pointerEvents: "none" }}>
        <Typography margin={1} variant={"overline"} display={"block"}>
          {angle.toPrecision(2)}&deg;
        </Typography>
      </Html>
    </mesh>
  );
};

export const MyEditorPreviewArmModel = () => {
  const lineBufferGeometryRef = React.useRef<BufferGeometry | null>(null);
  const pointsBufferGeometryRef = React.useRef<BufferGeometry | null>(null);

  const { state } = useArmContext();

  const renderAngles = React.useCallback((): React.ReactElement[] => {
    let result: React.ReactElement[] = [];

    const [vertices, angles] = [state.vertices.inner, state.angles.inner];

    for (let i = 0; i < Math.min(vertices.length, angles.length); ++i) {
      result.push(<Angle angle={angles[i]} position={vertices[i]} />);
    }

    return result;
  }, [state]);

  React.useEffect(() => {
    lineBufferGeometryRef.current?.setFromPoints(state.vertices.inner);
    pointsBufferGeometryRef.current?.setFromPoints(
      state.vertices.inner.slice(0, state.angles.inner.length)
    );
  }, [state, lineBufferGeometryRef, pointsBufferGeometryRef]);

  return (
    <group name={"Arm"}>
      {/* The line */}
      <line>
        <bufferGeometry ref={lineBufferGeometryRef} />
        <lineBasicMaterial
          color={LINE_COLOR}
          linewidth={LINE_WIDTH}
          linecap={"round"}
          linejoin={"round"}
          opacity={0.7}
          transparent={true}
        />
      </line>
      {/* The points */}
      <points>
        <bufferGeometry ref={pointsBufferGeometryRef} />
        <pointsMaterial size={POINT_SIZE} color={POINT_COLOR} />
      </points>
      {/* Angles */}
      {...renderAngles()}
    </group>
  );
};
