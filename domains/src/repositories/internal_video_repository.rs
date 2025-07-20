use crate::entities::video::VideoEntity;
use crate::value_objects::video_id::VideoId;

#[cfg_attr(test, mockall::automock)]
#[async_trait::async_trait]
pub trait InternalVideoRepository {
    async fn add_video_entity(&self, video_entity: &VideoEntity) -> Result<(), String>;

    async fn add_video_entities(&self, video_entities: &[VideoEntity]) -> Result<(), String>;

    async fn update_video_entity(&self, video_entity: &VideoEntity) -> Result<(), String>;

    async fn update_video_entities(&self, video_entities: &[VideoEntity]) -> Result<(), String>;

    async fn find_video_entity_by_id(&self, video_id: &VideoId) -> Result<bool, String>;

    async fn get_video_entity_by_id(
        &self,
        video_id: &VideoId,
    ) -> Result<Option<VideoEntity>, String>;

    async fn get_all_video_entities(&self) -> Result<Vec<VideoEntity>, String>;

    async fn delete_video_entity_by_id(&self, video_id: &VideoId) -> Result<(), String>;

    async fn delete_all_video_entities(&self) -> Result<(), String>;
}
