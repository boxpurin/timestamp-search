use crate::entities::video_timestamp::VideoTimestampEntity;
use crate::value_objects::video_id::VideoId;
use crate::value_objects::timestamp_id::TimestampId;
use errors::AppResult;

/// # InternalVideoRepository
/// サービス内部における動画のタイムスタンプに関するリポジトリ
/// # features
/// - タイムスタンプの追加
/// - タイムスタンプの修正
/// - タイムスタンプの取得
/// - タイムスタンプの削除
#[cfg_attr(test, mockall::automock)]
#[async_trait::async_trait]
pub trait InternalVideoRepository {
    async fn add_video_timestamp_entity(&self, entity: &VideoTimestampEntity) -> AppResult<()>;

    async fn add_video_timestamp_entities(&self, entities: &[VideoTimestampEntity]) -> AppResult<()>;

    async fn update_video_timestamp_entity(&self, entity: &VideoTimestampEntity) -> AppResult<()>;

    async fn update_video_timestamp_entities(&self, entities: &[VideoTimestampEntity]) -> AppResult<()>;

    async fn find_video_timestamp_entity_by_id(&self, video_id: &TimestampId) -> AppResult<bool>;

    async fn get_video_timestamp_entity_by_id(
        &self,
        video_id: &TimestampId,
    ) -> AppResult<Option<VideoTimestampEntity>>;

    async fn get_all_video_timestamp_entities(&self) -> AppResult<Vec<VideoTimestampEntity>>;

    async fn delete_video_timestamp_entity_by_id(&self, video_id: &VideoId) -> AppResult<()>;

    async fn delete_video_timestamp_entity_by_video_id(&self, video_id: &VideoId) -> AppResult<()>;

    async fn delete_all_video_timestamp_entities(&self) -> AppResult<()>;
}
