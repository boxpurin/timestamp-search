import * as React from "react";
import * as Mui from "@mui/material";
import { LocalizationProvider } from "@mui/x-date-pickers";
import { AdapterDateFns } from "@mui/x-date-pickers/AdapterDateFnsV3";
import { AllCommunityModule, ModuleRegistry } from "ag-grid-community";
import TimeStampSearch from "./layouts/timeStampSearch";
import { brown, yellow } from "@mui/material/colors";

function App() {
  ModuleRegistry.registerModules([AllCommunityModule]);
  const theme = React.useMemo(() => Mui.createTheme({
    palette: {
      primary: yellow,
      secondary: brown,
      background: {
        default: "#f5f5f5"
      }
    }
  }), []);

  return (
    <Mui.Box className="App">
      <Mui.ThemeProvider theme={theme}>
        <LocalizationProvider dateAdapter={AdapterDateFns}>
          <TimeStampSearch />
        </LocalizationProvider>
      </Mui.ThemeProvider>
    </Mui.Box>
  );
}

export default App;
