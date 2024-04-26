import { Save, Speed } from "@mui/icons-material";
import { Button, Grid, Stack, TextField, Typography } from "@mui/material";

export const MyEditorSettingsTab = () => {
  return (
    <Stack direction={"column"} spacing={2}>
      <Typography variant={"h6"}>End-Effector</Typography>
      <TextField
        fullWidth={true}
        InputProps={{
          startAdornment: <Speed color={"primary"} />,
        }}
        size={"small"}
        label={"Speed"}
        type={"number"}
        helperText={"In meters per second"}
      />
      <TextField
        fullWidth={true}
        InputProps={{
          startAdornment: <Speed color={"primary"} />,
        }}
        size={"small"}
        label={"Acceleration"}
        type={"number"}
        helperText={"In meters second squared"}
      />
      <Stack direction={"row"} justifyContent={"end"}>
        <Button startIcon={<Save />} variant={"outlined"}>
          Save
        </Button>
      </Stack>
    </Stack>
  );
};
