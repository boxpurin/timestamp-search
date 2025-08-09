import * as React from 'react';
import { GridRow } from "../types/gridTypes";
import SearchResultGrid from "../components/searchResultsGrid";
import { SearchQuery } from "../types/searchQuery";
import SearchForm from "../components/searchForm";
import { TimeStampData, TimeStampSearchResult } from "../domain/timeStampDataType";
import { styled, useMediaQuery } from '@mui/material';
import { yellow } from '@mui/material/colors';
import { createTheme, ThemeProvider } from '@mui/material/styles';

const ColorModeContext = React.createContext({ toggleColorMode: () => { } });

const defaultSearchQuery: SearchQuery = {
    keyword: "",
    searchPattern: "keyword",
    targetContentType: "",
    allowDateRange: false,
    page: 1,
    perPage: 100,
    totalPages: 1,
    startDate: new Date(),
    endDate: new Date(),
};

export const SearchQueryContext = React.createContext(
    {
        searchQuery: defaultSearchQuery,
        setSearchQuery: (_: SearchQuery) => { },
        isFetching: false,
        error: null as Error | null,
    }
);

const Main = styled('main')(({ theme }) => ({
    flexDirection: 'column',
    alignItems: 'center',
    justifyContent: 'center',
    minHeight: '100vh',
    textAlign: 'center',
}));

interface useFetchDataProps {
    onCompleted?: () => void;
}

const useFetchData = (props?: useFetchDataProps) => {
    const [isFetching, setIsFetching] = React.useState(false);
    const [searchQuery, setSearchQuery] = React.useState<SearchQuery>(defaultSearchQuery);
    const [dataRows, setData] = React.useState<GridRow[]>([]);
    const [error, setError] = React.useState<Error | null>(null);

    const fetchData = React.useCallback(async (query: SearchQuery) => {
        setIsFetching(true);
        setError(null);
        try {
            console.log(query);
            const response = await fetch("http://localhost:7700/api/v1/timestamps/search");
            if (!response.ok) {
                throw new Error("Failed to fetch");
            }
            const result: TimeStampSearchResult = await response.json();
            var rows: GridRow[] = Object.values(result.items).map((ts: TimeStampData, i: number) => {
                var row = new GridRow(i, ts);
                return row;
            });

            setData(rows);
            setIsFetching(false);
            if(props && props.onCompleted){
                props.onCompleted();
            }
        } catch (err) {
            setError(err as Error);
        } finally {
            setIsFetching(false);
        }
    }, []);

    return { dataRows, isFetching, error, searchQuery, setSearchQuery, fetchData };
};

const TimeStampSearch: React.FC<React.PropsWithChildren> = ({ children }) => {
    const [mode, setMode] = React.useState<'light' | 'dark'>('light');
    const colorMode = React.useMemo(
        () => ({
            toggleColorMode: () => {
                setMode((prevMode) => (prevMode === 'light' ? 'dark' : 'light'));
            },
        }),
        [],
    );

    const preferDarkMode = useMediaQuery('(prefers-color-scheme: dark)');

    React.useEffect(() => {
        if (localStorage.getItem('palletMode') === 'dark') {
            setMode('dark');
        } else if (localStorage.getItem('palletMode') === 'light') {
            setMode('light');
        } else if (preferDarkMode) {
            setMode('dark');
        } else {
            setMode('light');
        }
    }, [preferDarkMode]);

    const theme = React.useMemo(
        () =>
            createTheme({
                palette: {
                    primary: yellow,
                    mode,
                },
            }),
        [mode],
    );

    const { dataRows, isFetching, error, searchQuery, setSearchQuery, fetchData } = useFetchData();

    const onSearchSubmit = async () => {
        if (isFetching) {
            return;
        }
        try {
            await fetchData({ ...searchQuery, page: 1 });
        } catch (e) {
            console.log(e);
        }
    };

    const onGridPageChange = async (newPage: number) => {
        if (isFetching) {
            return;
        }
        try {
            await fetchData({ ...searchQuery, page: newPage });
        } catch (e) {
            console.log(e);
        }
    }

    return (
        <Main>
            <ColorModeContext.Provider value={colorMode}>
                <ThemeProvider theme={theme}>
                    <SearchQueryContext.Provider value={{ searchQuery, setSearchQuery, isFetching, error }}>
                        <SearchForm onSubmit={onSearchSubmit} />
                        <SearchResultGrid mode={mode} rows={dataRows} onPageChange={onGridPageChange} />
                    </SearchQueryContext.Provider>
                </ThemeProvider>
            </ColorModeContext.Provider>
        </Main>
    );
}

export default TimeStampSearch;