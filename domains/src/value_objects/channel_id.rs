use errors::{AppResult, AppError};
types::impl_string_value!(ChannelId);

impl ChannelId {
    /// Creates a new `ChannelId` from a string slice.
    /// # Arguments
    /// * `id` - A string slice that holds the channel ID.
    /// # Example
    /// ```
    /// use domains::value_objects::ChannelId;
    /// let channel_id = ChannelId::new("12345");
    /// assert_eq!(channel_id, Ok(Channel_Id("12345".to_string())));
    /// let invalid_channel_id = ChannelId::new("");
    /// assert!(invalid_channel_id.is_err());
    /// ```
    pub fn new(id: &str) -> AppResult<Self> {
        if id.is_empty() {
            return Err(AppError::InvalidInput("Channel ID cannot be empty".to_string()));
        }
        Ok(ChannelId(id.to_string()))
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn test_channel_id_new_valid() {
        let channel_id = ChannelId::new("12345");
        assert!(channel_id.is_ok());
        assert_eq!(channel_id.unwrap().0, "12345");
    }

    #[test]
    fn test_channel_id_new_empty() {
        let channel_id = ChannelId::new("");
        assert!(channel_id.is_err());
    }
}