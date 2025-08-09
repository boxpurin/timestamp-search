export class TimeStampData {
    videoId: string = "";
    description: string = "";
    seconds: number = 0;
    videoDetail?: TimeStampVideoDetail;
  }
  
export class TimeStampVideoDetail {
    videoTitle: string = "";
    thumbnailUrl: string = "";
    publishedAt: Date = new Date();
    actualStartTime: Date = new Date();
  }
  
export class TimeStampSearchResult {
    items: TimeStampData[] = [];
    page: number = 0;
    perPage: number = 0;
    totalPages: number = 0;
    totalHits: number = 0;
    estimatedTotalHits: number = 0;
    offset: number = 0;
    limit: number = 0;
  }