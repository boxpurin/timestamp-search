import { GridRow } from '../types/gridTypes';
import * as Mui from '@mui/material';
import React from 'react';
import { AgGridReact } from 'ag-grid-react';
import { columnsDefs } from "../types/gridTypes";
import { SearchQueryContext } from '../layouts/timeStampSearch';
import { lightGridTheme , darkGridTheme } from "../../styles/theme";


export interface GridPaginationProps {
  page?: number;
  pageSize?: number;
  onPageChange?: (newPage: number) => void;
  isFetching?: boolean;
};

export interface SearchResultProps {
  rows?: GridRow[];
  onPageChange?: (newPage: number) => void;
  mode: 'light' | 'dark';
};


const GridPagination = (props: GridPaginationProps) => {
  const onPageChange = props.onPageChange || ((_: number) => { });
  const page = React.useMemo(() => props.page || 1, [props.page]);
  const pageSize = props.pageSize || 1;

  return (
    <Mui.Grid2 container columnSpacing={2}>
      <Mui.Grid2 size={{ xs: 1 }}>
        <Mui.Box sx={{ justifyContent: "right", alignItems: "right" }}>
          <Mui.Typography variant="h6">{"ページ"}</Mui.Typography>
        </Mui.Box>
      </Mui.Grid2>
      <Mui.Grid2 size={{ xs: 3 }} />
      <Mui.Grid2 size={{ xs: 1 }}>
        <Mui.Button
          disabled={page === 1 || props.isFetching}
          variant="contained"
          onClick={() => { onPageChange(1) }}
        >
          {"先頭"}
        </Mui.Button>
      </Mui.Grid2>
      <Mui.Grid2 size={{ xs: 2 }}>
        <Mui.Box sx={{ display: "flex", justifyContent: "center", alignItems: "center" }}>
          <Mui.Button
            disabled={page <= 1 || props.isFetching}
            variant="contained"
            onClick={() => { onPageChange(page - 1) }}
          >
            {"<Prev"}
          </Mui.Button>
          <Mui.Typography variant="h6">
            {page}
          </Mui.Typography>
          <Mui.Button
            disabled={page >= pageSize || props.isFetching}
            variant="contained"
            onClick={() => { onPageChange(page + 1) }}
          >
            {"Next>"}
          </Mui.Button>
        </Mui.Box>
      </Mui.Grid2>
      <Mui.Grid2 size={{ xs: 3 }} />
      <Mui.Grid2 size={{ xs: 1 }} />
    </Mui.Grid2>
  );
}

const SearchResultsGrid = (props: SearchResultProps) => {
  const context = React.useContext(SearchQueryContext);
  return (
    <Mui.Box sx={{ py: 4, height: "100vh", justifyContent: "center", alignItems: "center" }}>
      <Mui.Container sx={{ height: "100%", width: "100%" }}>
        <GridPagination
          page={context.searchQuery.page}
          onPageChange={props.onPageChange}
          pageSize={context.searchQuery.totalPages}
          isFetching={context.isFetching}
        />
        <AgGridReact
          theme={props.mode === "light" ? lightGridTheme : darkGridTheme}
          columnDefs={columnsDefs}
          rowData={props.rows}
          loading={context.isFetching}
        />
      </Mui.Container>
    </Mui.Box>
  );
};

export default SearchResultsGrid;