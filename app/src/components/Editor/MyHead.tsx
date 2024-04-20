import { SelectTabData, SelectTabEvent, Tab, TabList } from "@fluentui/react-components";
import { Grid24Regular, Settings24Regular } from "@fluentui/react-icons";
import { useState } from "react";

export enum MyHeadTab {
  Home = "Home",
  Settings = "Settings",
}

export const MyHead = () => {
  const [selectedValue, setSelectedValue] = useState<MyHeadTab>(MyHeadTab.Home);

  const onTabSelect = (_event: SelectTabEvent, data: SelectTabData) => {
    setSelectedValue(data.value as MyHeadTab);
  };

  return (
    <>
      <TabList appearance="subtle" selectedValue={selectedValue} onTabSelect={onTabSelect}>
        <Tab value={MyHeadTab.Home} icon={<Grid24Regular />}>
          Home
        </Tab>
        <Tab value={MyHeadTab.Settings} icon={<Settings24Regular />}>
          Settings
        </Tab>
      </TabList>
    </>
  );
};
