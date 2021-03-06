use crate::{
    channel::WebhookType,
    id::{ChannelId, GuildId, WebhookId},
    user::User,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Webhook {
    pub avatar: Option<String>,
    pub channel_id: ChannelId,
    pub guild_id: Option<GuildId>,
    pub id: WebhookId,
    #[serde(default = "WebhookType::default", rename = "type")]
    pub kind: WebhookType,
    pub name: Option<String>,
    pub token: Option<String>,
    pub user: Option<User>,
}
