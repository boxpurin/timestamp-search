import React, { ChangeEvent } from 'react';
import * as Mui from "@mui/material";
import { SearchQuery, searchPattern } from "../types/searchQuery";
import { SearchQueryContext } from '../layouts/timeStampSearch';

export interface SearchFormTabProps {
    query: SearchQuery;
    onSubmit: () => void;
    onQueryUpdate: React.Dispatch<React.SetStateAction<SearchQuery>>;
}

interface TabPanelProps {
    children?: React.ReactNode;
    index: number;
    value: number;
}

const CustomTabPanel : React.FC<React.PropsWithChildren<TabPanelProps>> = (props: TabPanelProps) => {
    const { children, value, index, ...other } = props;

    return (
        <Mui.Box
            role="tabpanel"
            hidden={value !== index}
            id={`simple-tabpanel-${index}`}
            aria-labelledby={`simple-tab-${index}`}
            {...other}
        >
            {value === index && <Mui.Box sx={{ p: 3 }}>{children}</Mui.Box>}
        </Mui.Box>
    );
}

const KeywordSearchFormTab = (props: { query: SearchQuery, onSubmit: () => void, onQueryUpdate: (_: SearchQuery) => void }) => {
    const onSearchClick = async () => {
        await props.onSubmit();
    };
    return (
        <Mui.Stack sx={{ paddingTop: 1 }} direction="column">
            <Mui.Stack sx={{ py: 1 }} direction="column">
                <Mui.TextField
                    fullWidth
                    label="キーワード"
                    value={props.query.keyword}
                    onChange={
                        (e: ChangeEvent<HTMLInputElement>) =>
                            props.onQueryUpdate({ ...props.query, keyword: e.target.value })
                    }
                />
                <Mui.Button type="submit" value="search" onClick={onSearchClick} variant="contained" color="primary" disabled={props.query.keyword.length === 0}>
                    検索
                </Mui.Button>
            </Mui.Stack>
        </Mui.Stack>
    );
}

const DateRangeSearchFormTab = (props: { query: SearchQuery, onSubmit: () => void, onQueryUpdate: (_: SearchQuery) => void }) => {
    const onSearchClick = async () => {
        await props.onSubmit();
    };
    return (
        <Mui.Stack sx={{ paddingTop: 1 }} direction="column">
            <Mui.Stack sx={{ py: 1 }} direction="row">
                <Mui.TextField
                    fullWidth
                    label="開始日"
                    type="date"
                    slotProps={{
                        inputLabel: { shrink: true }
                    }}
                    value={props.query.startDate.toLocaleDateString(`sv-SE`)}
                    onChange={(e: ChangeEvent<HTMLInputElement>) => {
                        props.onQueryUpdate({ ...props.query, startDate: new Date(e.target.value) })
                    }}
                />
                <Mui.TextField
                    fullWidth
                    label="終了日"
                    type="date"
                    slotProps={{
                        inputLabel: { shrink: true }
                    }}
                    value={props.query.endDate.toLocaleDateString(`sv-SE`)}
                    onChange={(e: ChangeEvent<HTMLInputElement>) => {
                        props.onQueryUpdate({ ...props.query, endDate: new Date(e.target.value) })
                    }}
                />
            </Mui.Stack>
            <Mui.Button type="submit" value="search" onClick={onSearchClick} variant="contained" color="primary">
                検索
            </Mui.Button>
        </Mui.Stack>
    );
}

const DateInSearchFormTab = (props: { query: SearchQuery, onSubmit: () => void, onQueryUpdate: (_: SearchQuery) => void }) => {
    const onSearchClick = async () => {
        await props.onSubmit();
    };
    return (
        <Mui.Stack sx={{ paddingTop: 1 }} direction="column">
            <Mui.Stack sx={{ py: 1 }} direction="row">
                <Mui.TextField
                    fullWidth
                    label="日付"
                    type="date"
                    slotProps={{
                        inputLabel: { shrink: true }
                    }}
                    value={props.query.startDate.toLocaleDateString(`sv-SE`)}
                    onChange={(e: ChangeEvent<HTMLInputElement>) => props.onQueryUpdate({ ...props.query, startDate: new Date(e.target.value) })}
                />
            </Mui.Stack>
            <Mui.Button type="submit" value="search" onClick={onSearchClick} variant="contained" color="primary">
                検索
            </Mui.Button>
        </Mui.Stack>
    );
}


// Props for SearchForm
export interface SearchFormProps {
    onSubmit: () => void;
}


const SearchForm: React.FC<SearchFormProps> = (props: SearchFormProps) => {
    const [tabIndex, setTabIndex] = React.useState(0);
    const context = React.useContext(SearchQueryContext);

    return (
        <Mui.Box className="App-Header" sx={{ verticalAlign: "center" }}>
            <Mui.Typography component="h2" variant="h6" color="primary">なんかタイトル</Mui.Typography>
            <Mui.FormControl sx={{ width: "480px" }}>
                <Mui.Tabs sx={
                    {
                        alignItems: "center",
                        justifyContent: "center"
                    }}
                    value={tabIndex}
                    onChange={(_: React.SyntheticEvent, value: number) => {
                        setTabIndex(value);
                        context.setSearchQuery({ ...context.searchQuery, searchPattern: searchPattern[value] });
                    }}
                    aria-label="simple tabs example"
                >
                    <Mui.Tab label="キーワード" />
                    <Mui.Tab label="日付指定" />
                    <Mui.Tab label="日付範囲指定" />
                </Mui.Tabs>
                <Mui.Divider />
                <Mui.Box sx={{ padding: 1 }}>
                    <CustomTabPanel value={tabIndex} index={0}>
                        <KeywordSearchFormTab query={context.searchQuery} onSubmit={props.onSubmit} onQueryUpdate={context.setSearchQuery} />
                    </CustomTabPanel>
                    <CustomTabPanel value={tabIndex} index={1}>
                        <DateInSearchFormTab query={context.searchQuery} onSubmit={props.onSubmit} onQueryUpdate={context.setSearchQuery} />
                    </CustomTabPanel>
                    <CustomTabPanel value={tabIndex} index={2}>
                        <DateRangeSearchFormTab query={context.searchQuery} onSubmit={props.onSubmit} onQueryUpdate={context.setSearchQuery} />
                    </CustomTabPanel>
                </Mui.Box>
            </Mui.FormControl>
        </Mui.Box>
    );
};

export default SearchForm;