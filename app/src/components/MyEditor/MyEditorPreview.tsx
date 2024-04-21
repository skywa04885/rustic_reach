import { OrbitControls, PerspectiveCamera } from "@react-three/drei";
import { Canvas } from "@react-three/fiber";
import * as THREE from "three";
import { MyEditorPreviewArmModel } from "./MyEditorPreview/MyEditorPreviewArmModel";
import { MyEditorPreviewGridHelper } from "./MyEditorPreview/MyEditorPreviewGridHelper";
import { MyEditorPreviewAxesHelper } from "./MyEditorPreview/MyEditorPreviewAxesHelper";
import React from "react";
import { MyEditorPreviewTools } from "./MyEditorPreview/MyEditorPreviewTools";
import { useRaycaster } from "../../hooks/useRaycaster";

export interface IMyEditorPreviewContext {
  camera: React.MutableRefObject<THREE.Camera | null>;
  canvas: React.MutableRefObject<HTMLCanvasElement | null>;
  isDragging: boolean;
  setIsDragging: React.Dispatch<React.SetStateAction<boolean>>;
  raycaster: THREE.Raycaster;
}

export const MyEditorPreviewContext =
  React.createContext<IMyEditorPreviewContext | null>(null);

export const useEditorPreviewContext = (): IMyEditorPreviewContext => {
  const context = React.useContext(MyEditorPreviewContext);

  if (!context) {
    throw new Error(
      "useEditorPreviewContext() must be called in MyEditorPreviewContext.Provider"
    );
  }

  return context;
};

export const MyEditorPreview = () => {
  const cameraPosition = React.useMemo(() => new THREE.Vector3(80, 80, 80), []);

  const [isDragging, setIsDragging] = React.useState<boolean>(false);

  const cameraRef = React.useRef<THREE.PerspectiveCamera | null>(null);
  const canvasRef = React.useRef<HTMLCanvasElement | null>(null);

  const raycaster = useRaycaster(canvasRef, cameraRef);

  return (
    <Canvas ref={canvasRef}>
      {/* The camera and it's controls */}
      <PerspectiveCamera
        ref={cameraRef}
        position={cameraPosition}
        makeDefault={true}
      />
      <OrbitControls enabled={!isDragging} dampingFactor={1} />
      {/* The scene */}
      <ambientLight />
      <MyEditorPreviewGridHelper />
      <MyEditorPreviewAxesHelper />
      {/* The contents */}
      <MyEditorPreviewContext.Provider
        value={{
          raycaster,
          isDragging,
          setIsDragging,
          camera: cameraRef,
          canvas: canvasRef,
        }}
      >
        <MyEditorPreviewArmModel />
        <MyEditorPreviewTools />
      </MyEditorPreviewContext.Provider>
    </Canvas>
  );
};
