import {
  ControlCamera,
  GridOn,
  ThreeSixty,
  ViewInAr,
} from "@mui/icons-material";
import {
  Divider,
  Paper,
  Stack,
  ToggleButton,
  ToggleButtonGroup,
  Tooltip,
} from "@mui/material";
import { EditorMode, EditorPreviewOption, useEditorContext } from "../MyEditor";
import React, { useCallback } from "react";

export const MyEditorToolbarEditorMode = () => {
  const { mode, setMode } = useEditorContext();

  const onChange = useCallback(
    (
      _event: React.MouseEvent<HTMLElement, MouseEvent>,
      newMode: EditorMode
    ) => {
      setMode(newMode);
    },
    [setMode]
  );

  return (
    <ToggleButtonGroup
      color={"primary"}
      value={mode}
      onChange={onChange}
      size={"small"}
      exclusive={true}
    >
      {/* Translate end-effector */}
      <Tooltip title={"Translate end-effector"}>
        <ToggleButton value={EditorMode.TranslateEndEffector}>
          <ControlCamera />
        </ToggleButton>
      </Tooltip>
      {/* Orient end-effector */}
      <Tooltip title={"Orient end-effector"}>
        <ToggleButton value={EditorMode.OrientEndEffector}>
          <ThreeSixty />
        </ToggleButton>
      </Tooltip>
    </ToggleButtonGroup>
  );
};

export const MyEditorToolbarEditorPreviewOption = () => {
  const { previewOptions, setPreviewOptions } = useEditorContext();

  const onChange = useCallback(
    (
      _event: React.MouseEvent<HTMLElement>,
      newPreviewOptions: EditorPreviewOption[]
    ) => {
      setPreviewOptions(newPreviewOptions);
    },
    [setPreviewOptions]
  );

  return (
    <ToggleButtonGroup
      value={previewOptions}
      onChange={onChange}
      color={"secondary"}
      size={"small"}
      exclusive={false}
    >
      <Tooltip title={"Show grid helper"}>
        <ToggleButton value={EditorPreviewOption.ShowGridHelper}>
          <GridOn />
        </ToggleButton>
      </Tooltip>
      <Tooltip title={"Show axes helper"}>
        <ToggleButton value={EditorPreviewOption.ShowAxesHelper}>
          <ViewInAr />
        </ToggleButton>
      </Tooltip>
    </ToggleButtonGroup>
  );
};

export const MyEditorToolbar = () => {
  return (
    <Paper elevation={0}>
      <Stack
        padding={1}
        spacing={1}
        direction={"row"}
        divider={<Divider orientation={"vertical"} flexItem={true} />}
      >
        {/* Editor mode */}
        <MyEditorToolbarEditorMode />
        {/* Preview options */}
        <MyEditorToolbarEditorPreviewOption />
      </Stack>
    </Paper>
  );
};
