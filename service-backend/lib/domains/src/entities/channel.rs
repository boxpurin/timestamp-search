use crate::value_objects::channel_id::ChannelId;
use crate::value_objects::channel_name::ChannelName;
use rand::Rng;
use rand::distr::Alphanumeric;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChannelEntity {
    pub id: ChannelId,
    pub name: ChannelName,
}

impl ChannelEntity {
    pub fn new(id: ChannelId, name: ChannelName) -> Self {
        Self { id, name }
    }

    /// Test utility function.
    #[cfg(feature = "test_util")]
    pub fn with_random_id(name: ChannelName) -> Self {
        let mut rng = rand::rng();
        let v: String = (0..21).map(|_| rng.sample(Alphanumeric) as char).collect();
        Self {
            id: ChannelId::new(&format!("UC_{}", v)).unwrap(),
            name,
        }
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;
    #[test]
    fn random_id_channel() {
        let c = ChannelEntity::with_random_id(ChannelName::new("Channel name 1").unwrap());

        let c2 = ChannelEntity::with_random_id(ChannelName::new("Channel name 2").unwrap());

        assert_ne!(c.id, c2.id);
    }
}
