import { Home, Settings } from "@mui/icons-material";
import { Box, Paper, Stack, Tab, Tabs } from "@mui/material";
import React, { useState } from "react";
import { MyEditorSettingsTab } from "./MyEditorTabs/MyEditorSettingsTab";
import { MyEditorHomeTab } from "./MyEditorTabs/MyEditorHomeTab";

export enum MyEditorTab {
  Home = "Home",
  Settings = "Settings",
}

export const MyEditorTabs = () => {
  const [selectedTab, setSelectedTab] = useState<MyEditorTab>(MyEditorTab.Home);

  const onSelectedTabChange = React.useCallback(
    (_event: React.SyntheticEvent<Element, Event>, value: any): void => {
      setSelectedTab(value);
    },
    [setSelectedTab]
  );

  const tab = React.useMemo(() => {
    switch (selectedTab) {
      case MyEditorTab.Settings:
        return <MyEditorSettingsTab />;
      default:
        return <MyEditorHomeTab />;
    }
  }, [selectedTab]);

  return (
    <Stack direction={"column"}>
      <Tabs
        variant={"standard"}
        value={selectedTab}
        onChange={onSelectedTabChange}
      >
        <Tab
          iconPosition={"start"}
          value={MyEditorTab.Home}
          label={"Home"}
          icon={<Home />}
        />
        <Tab
          iconPosition={"start"}
          value={MyEditorTab.Settings}
          label={"Settings"}
          icon={<Settings />}
        />
      </Tabs>
      <Box
        paddingLeft={2}
        paddingRight={2}
        flexGrow={1}
      >
        {tab}
      </Box>
    </Stack>
  );
};
