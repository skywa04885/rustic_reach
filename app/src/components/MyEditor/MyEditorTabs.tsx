import { Home } from "@mui/icons-material";
import { Tab, Tabs } from "@mui/material";
import { useState } from "react";

export enum MyEditorTab {
  Home = "Home",
  Settings = "Settings",
}

export const MyEditorTabs = () => {
  const [selectedTab, setSelectedTab] = useState<MyEditorTab>(MyEditorTab.Home);

  return (
    <Tabs variant={"standard"} value={selectedTab}>
      <Tab iconPosition={"start"} value={MyEditorTab.Home} label={"Home"} icon={<Home />} />
      <Tab iconPosition={"start"} value={MyEditorTab.Settings} label={"Settings"} />
    </Tabs>
  );
};
