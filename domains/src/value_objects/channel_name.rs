use errors::{AppResult, AppError};
types::impl_string_value!(ChannelName);

impl ChannelName  {
    /// Creates a new `ChannelName` from a string slice.
    /// # Arguments
    /// * `name` - A string slice that holds the name of the channel.
    /// # Example
    /// ```
    /// use domains::value_objects::ChannelName;
    /// let channel_name = ChannelName::new("MyChannel");
    /// assert!(channel_name.is_ok());
    /// let invalid_channel_name = ChannelName::new("");
    /// assert!(invalid_channel_name.is_err());
    /// ```
    pub fn new(name: &str) -> AppResult<Self> {
        if name.is_empty() {
            return Err(AppError::InvalidInput("Channel name cannot be empty".to_string()));
        }

        Ok(ChannelName(name.to_string()))
    }
}


#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn test_channel_name() {
        let valid_name = ChannelName::new("ValidChannel");
        assert!(valid_name.is_ok());

        let invalid_name = ChannelName::new("");
        assert!(invalid_name.is_err());
    }
}