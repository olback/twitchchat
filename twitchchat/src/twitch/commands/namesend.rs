use super::*;

/// List current chatters in a channel. (marks the end)
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NamesEnd {
    pub(super) user: String,
    pub(super) channel: Channel,
}

impl NamesEnd {
    /// Your user for this event
    pub fn user(&self) -> &str {
        &self.user
    }
    /// The channel this event happened on
    pub fn channel(&self) -> &Channel {
        &self.channel
    }
}