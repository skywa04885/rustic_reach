import { createContext, useContext, useState } from 'react';
import { MyToolbar } from './Editor/MyToolbar';
import { MyPreview } from './Editor/MyPreview';
import { Divider, makeStyles } from '@fluentui/react-components';
import { MyHead } from './Editor/MyHead';
import { MyFooter } from './Editor/MyFooter';

export interface IEditorContext {
  mode: EditorMode,
  setMode: React.Dispatch<React.SetStateAction<EditorMode>>,
}

const EditorContext = createContext<IEditorContext | null>(null);

export enum EditorMode {
  TranslateEndEffector = "TranslateEndEffector",
  OrientEndEffector = "OrientEndEffector",
}

export const useEditorContext = (): IEditorContext => {
  const context = useContext(EditorContext);

  if (!context) {
    throw new Error('useEditorContext must be used within an EditorContext.Provider');
  }

  return context;
}

const useStyles = makeStyles({
  wrapper: {
    display: 'flex',
    flexDirection: 'column',
    height: '100vh',
  }
});

export const Editor = () => {
  const [mode, setMode] = useState<EditorMode>(EditorMode.TranslateEndEffector);

  const styles = useStyles();

  return (
    <EditorContext.Provider value={{
      mode,
      setMode
    }}>
      <div className={styles.wrapper}>
        <MyHead />
        <Divider />
        <MyToolbar />
        <MyPreview />
        <MyFooter />
      </div>
    </EditorContext.Provider>
  );
};
