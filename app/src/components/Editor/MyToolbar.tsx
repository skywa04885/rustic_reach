import {
  Toolbar,
  ToolbarButton,
  ToolbarDivider,
  ToolbarGroup,
  ToolbarProps,
  ToolbarRadioButton,
  ToolbarRadioGroup,
  ToolbarToggleButton,
  Tooltip,
} from "@fluentui/react-components";
import {
  ArrowFlowDiagonalUpRight24Regular,
  ArrowMove24Regular,
  CubeRotate20Regular,
  SubGrid24Regular,
} from "@fluentui/react-icons";
import { EditorMode } from "../Editor";
import { ReactElement, useState } from "react";

export interface IMyToolbarEditorModeButtonProps {
  mode: EditorMode;
  icon: ReactElement;
  tooltip: string;
}

export const MyToolbarEditorModeButton = ({
  mode,
  icon,
  tooltip,
}: IMyToolbarEditorModeButtonProps) => {
  return (
    <Tooltip content={tooltip} relationship={"label"}>
      <ToolbarRadioButton
        appearance="subtle"
        name="editorMode"
        value={mode}
        icon={icon}
      />
    </Tooltip>
  );
};

export const MyToolbarEditorMode = () => {
  return (
    <ToolbarRadioGroup>
      <MyToolbarEditorModeButton
        mode={EditorMode.TranslateEndEffector}
        icon={<ArrowMove24Regular />}
        tooltip={"Translate end-effector"}
      />
      <MyToolbarEditorModeButton
        mode={EditorMode.OrientEndEffector}
        icon={<CubeRotate20Regular />}
        tooltip={"Orient end-effector"}
      />
    </ToolbarRadioGroup>
  );
};

export interface IMyToolbarPreviewOptionProps {
  option: string;
  icon: ReactElement;
  tooltip: string;
}

export const MyToolbarPreviewOption = ({
  option,
  icon,
  tooltip,
}: IMyToolbarPreviewOptionProps) => {
  return (
    <Tooltip relationship={"label"} content={tooltip}>
      <ToolbarToggleButton
        appearance={"subtle"}
        name={"previewOptions"}
        value={option}
        icon={icon}
      />
    </Tooltip>
  );
};

export const MyToolbarPreviewOptions = () => {
  return (
    <ToolbarGroup>
      {/* Grid */}
      <MyToolbarPreviewOption
        tooltip={"Show grid in preview"}
        option={"grid"}
        icon={<SubGrid24Regular />}
      />
      {/* Axes */}
      <MyToolbarPreviewOption
        tooltip={"Show axes in preview"}
        option={"axes"}
        icon={<ArrowFlowDiagonalUpRight24Regular />}
      />
    </ToolbarGroup>
  );
};

export const MyToolbar = () => {
  const [checkedValues, setCheckedValues] = useState<Record<string, string[]>>({
    previewOptions: ["grid", "axes"],
    editorMode: [EditorMode.TranslateEndEffector],
  });

  const onChange: ToolbarProps["onCheckedValueChange"] = (
    _e,
    { name, checkedItems }
  ) => {
    console.log(checkedValues);
    setCheckedValues((s) => {
      return s ? { ...s, [name]: checkedItems } : { [name]: checkedItems };
    });
  };

  return (
    <Toolbar onCheckedValueChange={onChange} checkedValues={checkedValues}>
      <MyToolbarEditorMode />
      <ToolbarDivider />
      <MyToolbarPreviewOptions />
    </Toolbar>
  );
};
