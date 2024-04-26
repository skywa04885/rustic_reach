import { useArmContext } from "../../../providers/MyArmProvider";
import { EditorMode, useEditorContext } from "../../MyEditor";
import { MyEditorPreviewOrientEndEffectorTool } from "./MyEditorPreviewTools/MyEditorPreviewOrientEndEffectorTool";
import { MyEditorPreviewTranslateEndEffectorTool } from "./MyEditorPreviewTools/MyEditorPreviewTranslateEndEffectorTool";

export const MyEditorPreviewTools = (): React.ReactElement => {
  const { mode } = useEditorContext();
  const { state } = useArmContext();

  switch (mode) {
    case EditorMode.TranslateEndEffector:
      return (
        <MyEditorPreviewTranslateEndEffectorTool
          origin={state.vertices.end_effector}
          scale={10}
        />
      );
    case EditorMode.OrientEndEffector:
      return <MyEditorPreviewOrientEndEffectorTool />;
    case EditorMode.CircleTracer:
      return <></>;
  }
};
