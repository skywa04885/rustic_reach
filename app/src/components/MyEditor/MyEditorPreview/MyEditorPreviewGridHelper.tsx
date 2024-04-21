import React from "react";
import { Color, GridHelper } from "three";
import { EditorPreviewOption, useEditorContext } from "../../MyEditor";

export const MyEditorPreviewGridHelper = () => {
  const centerLineColor = new Color("#aaaaaa");
  const lineColor = new Color("#eeeeee");

  const gridHelperRef = React.useRef<GridHelper | null>(null);

  const { previewOptions } = useEditorContext();

  React.useEffect(() => {
    if (!gridHelperRef.current) {
      return;
    }

    gridHelperRef.current.visible = previewOptions.includes(
      EditorPreviewOption.ShowGridHelper
    );
  }, [gridHelperRef, previewOptions]);

  return (
    <gridHelper
      ref={gridHelperRef}
      scale={10}
      args={[20, 100, centerLineColor, lineColor]}
    />
  );
};
