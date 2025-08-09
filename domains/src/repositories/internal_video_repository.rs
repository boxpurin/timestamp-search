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
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};
    use chrono::DateTime;
    use crate::entities::channel::ChannelEntity;
    use crate::value_objects::channel_id::ChannelId;
    use crate::value_objects::channel_name::ChannelName;
    use crate::value_objects::video_description::VideoDescription;
    use crate::value_objects::video_title::VideoTitle;
    use super::*;

    use mockall::predicate::eq;
    use errors::AppError;

    struct InMemoryVideoRepository {
        pub db: Arc<Mutex<HashMap<VideoId, VideoEntity>>>,
    }

    impl InMemoryVideoRepository {
        pub fn new()-> Self{
            Self{ db: Arc::new(Mutex::new(HashMap::new())) }
        }
    }

    #[async_trait::async_trait]
    impl InternalVideoRepository for InMemoryVideoRepository {
        async fn add_video_entity(&self, video_entity: &VideoEntity) -> AppResult<()> {
            if !self.find_video_entity_by_id(&video_entity.id).await? {
                let mut db = self
                    .db
                    .lock()
                    .map_err(|_| AppError::Conflict("".to_string()))?;
                db.insert(video_entity.id.clone(), video_entity.clone());
            }
            Ok(())
        }

        async fn add_video_entities(&self, video_entities: &[VideoEntity]) -> AppResult<()> {
            for v in video_entities{
                self.add_video_entity(v).await?;
            }
            Ok(())
        }

        async fn update_video_entity(&self, video_entity: &VideoEntity) -> AppResult<()> {
            let mut db = self
                .db
                .lock()
                .map_err(|_| AppError::Conflict("".to_string()))?;
            if let Some(mut v) = db.get(video_entity.id.as_str()){
                v = video_entity;
            }
            Ok(())
        }

        async fn update_video_entities(&self, video_entities: &[VideoEntity]) -> AppResult<()> {
            for v in video_entities{
                self.update_video_entity(v).await?;
            }
            Ok(())
        }

        async fn find_video_entity_by_id(&self, video_id: &VideoId) -> AppResult<bool> {
            let mut db = self
                .db
                .lock()
                .map_err(|_| AppError::Conflict("".to_string()))?;
            let b = db.get(video_id);
            Ok(b.is_some())
        }

        async fn get_video_entity_by_id(&self, video_id: &VideoId) -> AppResult<Option<VideoEntity>> {
            let mut db = self
                .db
                .lock()
                .map_err(|_| AppError::Conflict("".to_string()))?;
            Ok(db.get(video_id).map(|v| v.clone()))
        }

        async fn get_all_video_entities(&self) -> AppResult<Vec<VideoEntity>> {
            let mut db = self
                .db
                .lock()
                .map_err(|_| AppError::Conflict("".to_string()))?;
            let vs = db.iter().map(|(_, v)| v.clone()).collect::<Vec<VideoEntity>>();
            Ok(vs)
        }

        async fn delete_video_entity_by_id(&self, video_id: &VideoId) -> AppResult<()> {
            let mut db = self
                .db
                .lock()
                .map_err(|_| AppError::Conflict("".to_string()))?;
            db.remove(video_id);
            Ok(())
        }

        async fn delete_all_video_entities(&self) -> AppResult<()> {
            let mut db = self
                .db
                .lock()
                .map_err(|_| AppError::Conflict("".to_string()))?;
            db.clear();
            Ok(())
        }
    }

    #[tokio::test]
    async fn mock_internal_video_repository() {
        let mut mock = MockInternalVideoRepository::new();

        let c = ChannelEntity::new(
            ChannelId::new("UC_1234567890abcdefghijk").unwrap(),
            ChannelName::new("ChannelName").unwrap()
        );

        let v = VideoEntity::new(
            VideoId::new("abc-def-ghi").unwrap(),
            VideoTitle::new("video title").unwrap(),
            vec![],
            VideoDescription::new("description").unwrap(),
            c.clone(),
            None,
            DateTime::default(),
            None,
        );

        mock.expect_add_video_entity()
            .with(eq(v.clone()))
            .returning(|_| {
                Ok(())
            });

        let r = mock.add_video_entity(&v).await;
        assert!(r.is_ok());

        mock.expect_update_video_entity()
            .with(eq(v.clone()))
            .returning(|_| {
               Ok(())
            });

        let r = mock.update_video_entity(&v).await;
        assert!(r.is_ok());

        mock.expect_delete_all_video_entities()
            .returning(|| Ok(()));

        let r = mock.delete_all_video_entities().await;
        assert!(r.is_ok());
    }


    #[tokio::test]
    async fn in_memory_internal_video_repository() {
        let mut repo = InMemoryVideoRepository::new();

        let c = ChannelEntity::new(
            ChannelId::new("UC_1234567890abcdefghijk").unwrap(),
            ChannelName::new("ChannelName").unwrap()
        );

        let v = VideoEntity::new(
            VideoId::new("abc-def-ghi").unwrap(),
            VideoTitle::new("video title").unwrap(),
            vec![],
            VideoDescription::new("description").unwrap(),
            c.clone(),
            None,
            DateTime::default(),
            None,
        );

        let r = repo.add_video_entity(&v).await;
        assert!(r.is_ok());

        let r = repo.find_video_entity_by_id(&v.id).await;
        assert!(r.is_ok());
        let r = r.unwrap();
        assert_eq!(r, true);

        let r = repo.get_all_video_entities().await;
        assert!(r.is_ok());
        let r = r.unwrap();
        assert_eq!(r.len(), 1);
    }
}