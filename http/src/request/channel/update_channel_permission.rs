use super::UpdateChannelPermissionConfigured;
use crate::request::prelude::*;
use twilight_model::{
    guild::Permissions,
    id::{ChannelId, RoleId, UserId},
};

/// Update the permissions for a role or a user in a channel.
///
/// # Examples:
///
/// Create permission overrides for a role to view the channel, but not send messages:
///
/// ```rust,no_run
/// use twilight_http::Client;
/// use twilight_model::guild::Permissions;
/// use twilight_model::id::{ChannelId, RoleId};
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
/// let client = Client::new("my token");
///
/// let channel_id = ChannelId(123);
/// let allow = Permissions::VIEW_CHANNEL;
/// let deny = Permissions::SEND_MESSAGES;
/// let role_id = RoleId(432);
///
/// client.update_channel_permission(channel_id, allow, deny)
///     .role(role_id)
///     .await?;
/// # Ok(()) }
/// ```
pub struct UpdateChannelPermission<'a> {
    allow: Permissions,
    channel_id: ChannelId,
    deny: Permissions,
    http: &'a Client,
}

impl<'a> UpdateChannelPermission<'a> {
    pub(crate) fn new(
        http: &'a Client,
        channel_id: ChannelId,
        allow: Permissions,
        deny: Permissions,
    ) -> Self {
        Self {
            allow,
            channel_id,
            deny,
            http,
        }
    }

    /// Specify this override to be for a member.
    pub fn member(self, user_id: impl Into<UserId>) -> UpdateChannelPermissionConfigured<'a> {
        self.configure("member", user_id.into().0)
    }

    /// Specify this override to be for a role.
    pub fn role(self, role_id: impl Into<RoleId>) -> UpdateChannelPermissionConfigured<'a> {
        self.configure("role", role_id.into().0)
    }

    fn configure(
        self,
        kind: impl Into<String>,
        target_id: u64,
    ) -> UpdateChannelPermissionConfigured<'a> {
        UpdateChannelPermissionConfigured::new(
            self.http,
            self.channel_id,
            self.allow,
            self.deny,
            kind,
            target_id,
        )
    }
}
