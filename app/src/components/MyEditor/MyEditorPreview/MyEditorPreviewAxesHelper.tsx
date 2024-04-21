import React from "react";
import { AxesHelper } from "three";
import { EditorPreviewOption, useEditorContext } from "../../MyEditor";

export const MyEditorPreviewAxesHelper = () => {
  const axesHelperRef = React.useRef<AxesHelper | null>(null);

  const { previewOptions } = useEditorContext();

  React.useEffect(() => {
    if (!axesHelperRef.current) {
      return;
    }

    axesHelperRef.current.visible = previewOptions.includes(
      EditorPreviewOption.ShowAxesHelper
    );
  }, [axesHelperRef, previewOptions]);

  return <axesHelper ref={axesHelperRef} scale={10} />;
};
