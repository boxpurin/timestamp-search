use crate::adapter::video::VideoEntityConverter;
use crate::config::YOUTUBE_CLIENT;
use domains::entities::video::VideoEntity;
use domains::repositories::external_video_repository::ExternalVideoRepository;
use domains::value_objects::channel_id::ChannelId;
use errors::{AppError, AppResult};
use google_youtube3::{YouTube, hyper_rustls, hyper_util, yup_oauth2, Result as YouTubeResult, common};
use google_youtube3::api::{Channel as YouTubeChannel, Video as YouTubeVideo};
use google_youtube3::hyper::StatusCode;
use hyper_rustls::HttpsConnector;
use hyper_util::client::legacy::connect::HttpConnector;

pub struct YoutubeVideoRepository {
    api_client: YouTubeApi,
}

pub async fn create_youtube_video_repository() -> YoutubeVideoRepository {
    tracing::info!("read_application_secret");
    let secret = yup_oauth2::read_application_secret(&YOUTUBE_CLIENT.google_client_secret_path)
        .await
        .expect("TSSEARCH_GOOGLE_CLIENT_SECRET_PATH FILE NOT FOUND");

    tracing::info!("read_client_secret");
    let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
        secret,
        yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
    )
    .persist_tokens_to_disk(&YOUTUBE_CLIENT.persistent_token_path)
    .build()
    .await
    .expect("InstalledFlowAuthenticator: builder");

    let client = hyper_util::client::legacy::Client::builder(hyper_util::rt::TokioExecutor::new())
        .build(
            hyper_rustls::HttpsConnectorBuilder::new()
                .with_native_roots()
                .unwrap()
                .https_or_http()
                .enable_http1()
                .build(),
        );

    tracing::info!("Create youtube client");
    let hub = YouTube::new(client, auth);
    let api_impl = YouTubeApi { hub };
    YoutubeVideoRepository::new(api_impl)
}

#[async_trait::async_trait]
impl ExternalVideoRepository for YoutubeVideoRepository {
    /// YouTube APIを使用して、指定されたチャンネルIDのすべてのビデオを取得します。
    ///
    /// # Arguments
    /// * `channel_id` - チャンネルID
    ///
    /// # Returns
    /// * `Result<Vec<VideoEntity>, String>` - ビデオ情報のリストまたはエラーメッセージ
    async fn fetch_all_videos_by_channel_id(
        &self,
        channel_id: &ChannelId,
    ) -> AppResult<Vec<VideoEntity>> {
        tracing::debug!("fetch_all_videos_by_channel_id");

        let mut videos: Vec<VideoEntity> = Vec::new();

        let c = self.api_client.fetch_channel(channel_id).await?;
        if c.is_none() {
            AppError::InvalidInput(format!("Channel not found: {}", channel_id));
        }

        let uploads = c.unwrap_or_default().content_details.unwrap_or_default().related_playlists.unwrap_or_default().uploads.unwrap_or_default();

        let (mut ids, mut next) = self.api_client.fetch_playlist_video_ids(&uploads, 50, None).await?;
        let v = self.api_client.fetch_videos(ids).await?;
        videos.extend(v.into_iter().map(|v| VideoEntityConverter(v).try_into().unwrap()));

        while next.is_some() {
            (ids, next) = self.api_client.fetch_playlist_video_ids(&uploads, 50, next).await?;
            let v = self.api_client.fetch_videos(ids).await?;
            videos.extend(v.into_iter().map(|v| VideoEntityConverter(v).try_into().unwrap()));
        }
        Ok(videos)
    }

    /// YouTube APIを使用して、指定されたチャンネルIDの最新のビデオを取得します。
    ///
    /// * `channel_id` - チャンネルID
    /// * `count` - 取得するビデオの数
    ///
    /// # Returns
    /// * `Result<Vec<VideoEntity>, String>` - ビデオ情報のリストまたはエラーメッセージ
    async fn fetch_recent_video_by_channel_id(
        &self,
        channel_id: &ChannelId,
        max_results: u32,
    ) -> AppResult<Vec<VideoEntity>> {
        tracing::debug!("fetching recent video from api");
        let c = self.api_client.fetch_channel(channel_id).await?;
        if c.is_none() {
            AppError::InvalidInput(format!("Channel not found: {}", channel_id));
        }

        let uploads = c.unwrap_or_default().content_details.unwrap_or_default().related_playlists.unwrap_or_default().uploads.unwrap_or_default();
        let (ids, _) = self.api_client.fetch_playlist_video_ids(&uploads, max_results, None).await?;
        let v = self.api_client.fetch_videos(ids).await?;

        Ok(v.into_iter().map(|v| VideoEntityConverter(v).try_into().unwrap()).collect())
    }
}

impl YoutubeVideoRepository {
    pub fn new(api_client: YouTubeApi) -> Self {
        YoutubeVideoRepository { api_client }
    }
}

pub struct YouTubeApi {
    hub: YouTube<HttpsConnector<HttpConnector>>,
}

impl YouTubeApi {
    ///
    /// YouTubeData APIへ、再送処理を含めて特定のリクエストを送るためのラッパ
    ///
    async fn try_req<R, Fut, F>(&self, req: F) -> YouTubeResult<R>
    where
        F: Fn() -> Fut,
        Fut: Future<Output = YouTubeResult<(common::Response, R)>>,
    {
        // 失敗時の待ち時間
        let mut elapsed = 1;

        // リクエストを送る
        tokio::time::sleep(tokio::time::Duration::from_secs(elapsed)).await;
        let ret = req().await;
        let (mut res, mut v) = ret?;

        // エラーが出てるなら再送処理
        loop {
            match res.status() {
                StatusCode::OK => break,
                _ => {
                    // 再試行のための待ち時間の倍増
                    elapsed *= 2;
                    tokio::time::sleep(tokio::time::Duration::from_secs(elapsed)).await;
                }
            };
            // 1分を超えたら再試行をやめて終了
            if elapsed > 60 {
                break;
            }
            // 指定時間待ったうえでリクエストを送信する
            tracing::debug!("Request retry : wait time {elapsed} sec.");
            (res, v) = req().await?;
        }

        Ok(v)
    }

    pub async fn fetch_channel(&self, channel_id: &ChannelId) -> YouTubeResult<Option<YouTubeChannel>> {
        let res = self.try_req(|| {
            self
            .hub
            .channels()
            .list(&vec!["contentDetails".to_string()])
            .add_id(channel_id)
            .doit()
        }).await?;
        let items = res.items.unwrap_or_default();
        let c = items.first().cloned();
        Ok(c)
    }

    pub async fn fetch_playlist_video_ids(&self, playlist_id: &str, max_results: u32, next_page_token: Option<String>) -> YouTubeResult<(Vec<String>, Option<String>)> {
        let res = self.try_req(||{
            let mut req = self.hub.playlist_items()
                .list(&vec!["snippet".to_string()])
                .playlist_id(playlist_id)
                .max_results(max_results);
                if next_page_token.is_some() {
                    req = req.page_token(&next_page_token.clone().unwrap());
                }
                req.doit()
        }).await?;

        let items = res.items.unwrap_or_default();

        let video_ids = items
            .into_iter()
            .map(|i| i.snippet.unwrap_or_default().resource_id.unwrap_or_default().video_id.unwrap_or_default())
            .collect();
        Ok((video_ids, res.next_page_token))
    }

    pub async fn fetch_videos(&self, video_ids: Vec<String>) -> YouTubeResult<Vec<YouTubeVideo>> {
        let res = self.try_req(||{
            let mut req = self.hub
                .videos()
                .list(&vec![
                    "snippet".to_string(),
                    "contentDetails".to_string(),
                    "liveStreamingDetails".to_string()]
                )
                .max_results(video_ids.len() as u32);

            for v in video_ids.iter() {
                req = req.add_id(v);
            }

            req.doit()
        }).await?;

        Ok(res.items.unwrap_or_default())
    }
}
#[cfg(test)]
mod unit_tests {
}
