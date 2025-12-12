use errors::AppError;
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
    pub fn new(id: &str) -> Result<Self, AppError> {
        if id.len() != 24 {
            return Err(AppError::DomainParseError(String::from(
                "Channel ID must be 24 characters long",
            )));
        }
        if !id.starts_with("UC") {
            return Err(AppError::DomainParseError(String::from(
                "Channel ID must start with 'UC'",
            )));
        }
        if !id
            .chars()
            .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
        {
            return Err(AppError::DomainParseError(String::from(
                "Channel ID must contain only alphanumeric characters, hyphens, or underscores",
            )));
        }
        Ok(ChannelId(id.to_string()))
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[rstest::rstest]
    #[test]
    #[case("UC_x5XG1OV2P6uZZ5FSM9Ttw")]
    #[case("UC-lHJZR3Gqxm24_Vd_AJ5Yw")]
    fn valid_channel_ids(#[case] id: &str) {
        assert!(ChannelId::new(id).is_ok());
    }

    #[rstest::rstest]
    #[test]
    #[case::empty("")]
    #[case::short("short")]
    #[case::long("this is long long long long id")]
    fn invalid_channel_ids(#[case] id: &str) {
        assert!(ChannelId::new(id).is_err());
    }
}
