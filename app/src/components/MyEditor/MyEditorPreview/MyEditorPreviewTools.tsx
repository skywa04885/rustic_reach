import { useArmContext } from "../../../providers/MyArmProvider";
import { EditorMode, useEditorContext } from "../../MyEditor";
import { MyEditorPreviewTranslateEndEffectorTool } from "./MyEditorPreviewTools/MyEditorPreviewTranslateEndEffectorTool";

export const MyEditorPreviewTools = (): React.ReactElement => {
  const { mode } = useEditorContext();
  const {state } = useArmContext();
  
  switch (mode) {
    case EditorMode.TranslateEndEffector:
      return <MyEditorPreviewTranslateEndEffectorTool position={state.vertices.end_effector} scale={10} />
    case EditorMode.OrientEndEffector:
      return <></>;
  }
};
