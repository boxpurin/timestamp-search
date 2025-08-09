use domains::entities::channel::ChannelEntity;
use errors::AppError;
use google_youtube3::api::{Channel, Video};
use domains::value_objects::channel_id::ChannelId;
use domains::value_objects::channel_name::ChannelName;

pub struct ChannelToChannelEntityConverter(pub Channel);
pub struct VideoToChannelEntityConverter(pub Video);

impl TryInto<ChannelEntity> for ChannelToChannelEntityConverter {
    type Error = AppError;
    fn try_into(self) -> Result<ChannelEntity, Self::Error> {
        let inner = self.0;
        let snippet = inner.snippet.ok_or(AppError::InvalidInput(
            "Video snippet is missing".to_string()
        ))?;
        let id = inner.id.ok_or(AppError::InvalidInput(
            "Channel ID is missing".to_string()
        ))?;
        let name = snippet.title.ok_or(AppError::InvalidInput
            ("Channel name is missing".to_string())
        )?;
        Ok(ChannelEntity {
            id: ChannelId::new(&id)?,
            name: ChannelName::new(&name)?,
        })
    }
}

impl TryInto<ChannelEntity> for VideoToChannelEntityConverter {
    type Error = AppError;
    fn try_into(self) -> Result<ChannelEntity, Self::Error> {
        let inner = self.0;
        let snippet = inner.snippet.ok_or(AppError::InvalidInput(
            "Video snippet is missing".to_string(),
        ))?;
        let channel_id = snippet.channel_id.ok_or(AppError::InvalidInput(
            "Channel ID is missing in video snippet".to_string(),
        ))?;
        let channel_name = snippet.channel_title.ok_or(AppError::InvalidInput(
            "Channel name is missing in video snippet".to_string(),
        ))?;
        Ok(ChannelEntity {
            id: ChannelId::new(&channel_id)?,
            name: ChannelName::new(&channel_name)?,
        })
    }
}


#[cfg(test)]
mod unit_tests{
    use super::*;
    use domains::entities::channel::ChannelEntity;
    use google_youtube3::api::{Channel, Video};

    #[test]
    fn channel_to_channel_entity_converter_test(){
        let channel = Channel::default();
        let c = ChannelToChannelEntityConverter(channel);

        let r: Result<ChannelEntity, AppError> = c.try_into();
        assert!(r.is_err());
    }
}