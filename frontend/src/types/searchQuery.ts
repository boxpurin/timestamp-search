
export type SearchQuery = {
    keyword: string;
    targetContentType: string;
    allowDateRange: boolean;
    page: number;
    perPage: number;
    totalPages: number;
    searchPattern: SearchPattern;
    startDate: Date;
    endDate: Date;
  };


export const searchPattern = ["keyword", "dateRange" ,"dateIn"] as const;
export type SearchPattern = typeof searchPattern[number]; // "keyword" | "dateIn"