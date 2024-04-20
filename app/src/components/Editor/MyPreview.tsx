import { OrbitControls, PerspectiveCamera } from "@react-three/drei";
import { Canvas } from "@react-three/fiber";
import { Color } from "three";

export const MyPreviewGrid = () => {
  const centerLineColor = new Color("#aaaaaa");
  const lineColor = new Color("#eeeeee");

  return <gridHelper scale={1} args={[20, 100, centerLineColor, lineColor]} />;
};

export const MyPreview = () => {
  return (
    <Canvas>
      {/* The camera and it's controls */}
      <PerspectiveCamera position={[2, 2, 2]} makeDefault={true} />
      <OrbitControls dampingFactor={1} />
      {/* The scene */}
      <ambientLight />
      <axesHelper scale={1} />
      <MyPreviewGrid /> 
    </Canvas>
  );
}
