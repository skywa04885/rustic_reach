
import { EditorPreviewOption, useEditorContext } from "../../MyEditor";
import { Grid } from "@react-three/drei";

export const MyEditorPreviewGridHelper = () => {
  const { previewOptions } = useEditorContext();

  return (
    <Grid
      scale={10}
      infiniteGrid={true}
      fadeFrom={0}
      visible={previewOptions.includes(EditorPreviewOption.ShowGridHelper)}
    />
  );
};
