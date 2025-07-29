use types::impl_string_value;
impl_string_value!(ChannelName);

impl ChannelName  {
    /// Creates a new `ChannelName` from a string slice.
    pub fn new(name: &str) -> Self {
        ChannelName(name.to_string())
    }

    pub fn is_valid(&self) -> bool {
        // A valid channel name must be between 1 and 100 characters long
        let len = self.0.chars().count();
        len > 0 && len <= 100
    }

}