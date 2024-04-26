import { createContext, useContext, useState } from "react";
import { MyEditorPreview } from "./MyEditor/MyEditorPreview";
import { MyEditorToolbar } from "./MyEditor/MyEditorToolbar";
import { Grid, Stack } from "@mui/material";
import { MyEditorTabs } from "./MyEditor/MyEditorTabs";

export enum EditorMode {
  TranslateEndEffector = "TranslateEndEffector",
  OrientEndEffector = "OrientEndEffector",
  CircleTracer = "CircleTracer",
}

export enum EditorPreviewOption {
  ShowGridHelper = "ShowGridHelper",
  ShowAxesHelper = "ShowAxesHelper",
}

export interface IEditorContext {
  mode: EditorMode;
  setMode: React.Dispatch<React.SetStateAction<EditorMode>>;
  previewOptions: EditorPreviewOption[];
  setPreviewOptions: React.Dispatch<
    React.SetStateAction<EditorPreviewOption[]>
  >;
}

const EditorContext = createContext<IEditorContext | null>(null);

export const useEditorContext = (): IEditorContext => {
  const context = useContext(EditorContext);

  if (!context) {
    throw new Error(
      "useEditorContext must be used within an EditorContext.Provider"
    );
  }

  return context;
};

export const MyEditor = () => {
  const [mode, setMode] = useState<EditorMode>(EditorMode.TranslateEndEffector);
  const [previewOptions, setPreviewOptions] = useState<EditorPreviewOption[]>([
    EditorPreviewOption.ShowGridHelper,
  ]);

  return (
    <EditorContext.Provider
      value={{
        mode,
        setMode,
        previewOptions,
        setPreviewOptions,
      }}
    >
      <Grid container={true} height={"100vh"} width={"100vw"}>
        <Grid item={true} xs={4}>
          <Stack direction={"column"} height={"100%"}>
            <MyEditorTabs />
          </Stack>
        </Grid>
        <Grid item={true} xs={8}>
          <Stack direction={"column"} height={"100%"}>
            <MyEditorPreview />
            <MyEditorToolbar />
          </Stack>
        </Grid>
      </Grid>
    </EditorContext.Provider>
  );
};
