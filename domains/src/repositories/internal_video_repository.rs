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

#[cfg(test)]
mod unit_tests{
    use chrono::DateTime;
    use crate::entities::channel::ChannelEntity;
    use crate::value_objects::channel_id::ChannelId;
    use crate::value_objects::channel_name::ChannelName;
    use crate::value_objects::video_description::VideoDescription;
    use crate::value_objects::video_title::VideoTitle;
    use super::*;


    use mockall::predicate::eq;

    #[tokio::test]
    async fn mock_internal_video_repository() {
        let mut mock = MockInternalVideoRepository::new();

        let c = ChannelEntity::new(
            ChannelId::new("UC_1234567890abcdefghijk").unwrap(),
            ChannelName::new("ChannelName").unwrap()
        );

        let v = VideoEntity::new(
            VideoId::new("abc-def-ghi").unwrap(),
            VideoTitle::new("video").unwrap(),
            vec![],
            VideoDescription::new("description").unwrap(),
            c.clone(),
            None,
            DateTime::default(),
            None,
        );

        mock.expect_add_video_entity()
            .with(eq(v.clone()))
            .returning(|_| Ok(()));




        mock.expect_delete_all_video_entities()
            .returning(|| Ok(()));

        let r = mock.add_video_entity(&v).await;
        assert!(r.is_ok());

        let r = mock.delete_all_video_entities().await;
        assert!(r.is_ok());
    }
}