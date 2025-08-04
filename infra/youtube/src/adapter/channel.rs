use domains::entities::channel::ChannelEntity;
use errors::{AppError, AppResult};
use google_youtube3::api::{Channel, Video};

pub struct ChannelToChannelEntityConverter(pub Channel);
pub struct VideoToChannelEntityConverter(pub Video);

impl TryInto<ChannelEntity> for ChannelToChannelEntityConverter {
    type Error = String;
    fn try_into(self) -> Result<ChannelEntity, Self::Error> {
        let inner = self.0;
        let snippet = inner.snippet.ok_or("Channel snippet is missing")?;
        let id = inner.id.ok_or("Channel ID is missing")?;
        let name = snippet.title.ok_or("Channel name is missing")?;
        Ok(ChannelEntity {
            id: id.into(),
            name: name.into(),
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
            id: channel_id.into(),
            name: channel_name.into(),
        })
    }
}
