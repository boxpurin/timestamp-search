use errors::{AppError, AppResult};
types::impl_string_value!(ChannelId);
impl ChannelId {
    /// Creates a new `ChannelId` from a string slice.
    /// # Arguments
    /// * `id` - A string slice that holds the channel ID.
    /// # Example
    /// ```
    /// use domains::value_objects::ChannelId;
    /// let channel_id = ChannelId::new("UC_x5XG1OV2P6uZZ5FSM9Ttw");
    /// assert!(channel_id.is_ok());
    /// let invalid_channel_id = ChannelId::new("invalid");
    /// assert!(invalid_channel_id.is_err());
    /// ```
    pub fn new(id: &str) -> AppResult<Self> {
        if id.len() != 24 {
            return Err(AppError::InvalidInput(
                "Channel ID must be 24 characters long".to_string(),
            ));
        }
        if !id.starts_with("UC") {
            return Err(AppError::InvalidInput(
                "Channel ID must start with 'UC'".to_string(),
            ));
        }
        if !id
            .chars()
            .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
        {
            return Err(AppError::InvalidInput(
                "Channel ID must contain only alphanumeric characters, hyphens, or underscores"
                    .to_string(),
            ));
        }
        Ok(ChannelId(id.to_string()))
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn valid_channel_ids() {
        assert!(ChannelId::new("UC_x5XG1OV2P6uZZ5FSM9Ttw").is_ok());
        assert!(ChannelId::new("UC-lHJZR3Gqxm24_Vd_AJ5Yw").is_ok());
    }

    #[test]
    fn invalid_channel_ids() {
        // Invalid length
        assert!(ChannelId::new("").is_err());
        assert!(ChannelId::new("short").is_err());
        assert!(ChannelId::new("this is way too long for a channel id").is_err());

        // Invalid prefix
        assert!(ChannelId::new("VC_x5XG1OV2P6uZZ5FSM9Ttw").is_err());

        // Invalid characters
        assert!(ChannelId::new("UC_x5XG1OV2P6uZZ5FSM9Ttw!").is_err());
    }
}
