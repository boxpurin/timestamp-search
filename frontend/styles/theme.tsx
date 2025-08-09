import { brown, yellow } from "@mui/material/colors";
import { extendTheme } from "@mui/material/styles";
import { themeQuartz } from "ag-grid-community";

const theme = extendTheme({
    colorSchemes: {
        light: {
            palette: {
                primary: {
                    main: brown[600],
                },
                secondary: {
                    main: brown[700],
                },
            }
        },
        dark: {
            palette: {
                primary: {
                    main: yellow[400],
                },
                secondary: {
                    main: yellow[500],
                },
            }
        }
    }
});


// to use myTheme in an application, pass it to the theme grid option
export const lightGridTheme = themeQuartz
    .withParams({
        browserColorScheme: "light",
        headerFontSize: 14
    });

// to use myTheme in an application, pass it to the theme grid option
export const darkGridTheme = themeQuartz
    .withParams({
        backgroundColor: "#1f2836",
        browserColorScheme: "dark",
        chromeBackgroundColor: {
            ref: "foregroundColor",
            mix: 0.07,
            onto: "backgroundColor"
        },
        foregroundColor: "#FFF",
        headerFontSize: 14
    });


export default theme;