// 外部サービスのビデオへアクセスするためのリポジトリ
// 想定される外部サービス: YouTube
use crate::entities::video::VideoEntity;
use crate::value_objects::channel_id::ChannelId;

#[cfg_attr(test, mockall::automock)]
#[async_trait::async_trait]
pub trait ExternalVideoRepository {
    /// チャンネルIDからビデオ情報を取得する
    ///
    /// # Arguments
    /// * `channel_id` - チャンネルID
    ///
    /// # Returns
    /// * `Result<Vec<VideoEntity>, String>` - チャンネル情報またはエラーメッセージ
    async fn fetch_all_videos_by_channel_id(
        &self,
        channel_id: &ChannelId,
    ) -> Result<Vec<VideoEntity>, String>;

    /// チャンネルIDから直近のビデオ情報を取得する
    /// # Arguments
    /// * `channel_id` - チャンネルID
    /// * `count` - 取得するビデオの数
    /// # Returns
    /// * `Result<Option<VideoEntity>, String>` - ビデオ情報またはエラーメッセージ
    async fn fetch_recent_video_by_channel_id(
        &self,
        channel_id: &ChannelId,
        count: u32,
    ) -> Result<Vec<VideoEntity>, String>;
}