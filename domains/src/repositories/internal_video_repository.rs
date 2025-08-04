use crate::entities::video::VideoEntity;
use crate::value_objects::video_id::VideoId;
use errors::AppResult;

#[cfg_attr(test, mockall::automock)]
#[async_trait::async_trait]
pub trait InternalVideoRepository {
    async fn add_video_entity(&self, video_entity: &VideoEntity) -> AppResult<()>;

    async fn add_video_entities(&self, video_entities: &[VideoEntity]) -> AppResult<()>;

    async fn update_video_entity(&self, video_entity: &VideoEntity) -> AppResult<()>;

    async fn update_video_entities(&self, video_entities: &[VideoEntity]) -> AppResult<()>;

    async fn find_video_entity_by_id(&self, video_id: &VideoId) -> AppResult<bool>;

    async fn get_video_entity_by_id(&self, video_id: &VideoId) -> AppResult<Option<VideoEntity>>;

    async fn get_all_video_entities(&self) -> AppResult<Vec<VideoEntity>>;

    async fn delete_video_entity_by_id(&self, video_id: &VideoId) -> AppResult<()>;

    async fn delete_all_video_entities(&self) -> AppResult<()>;
}
