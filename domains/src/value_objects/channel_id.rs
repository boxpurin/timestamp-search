use types::impl_string_value;
impl_string_value!(ChannelId);

impl ChannelId {
    pub fn new(id: &str) -> Self {
        ChannelId(id.to_string())
    }
}