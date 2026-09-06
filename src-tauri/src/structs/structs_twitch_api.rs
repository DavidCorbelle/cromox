use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseChatters {
    pub data: Vec<ChatterList>,
    pub pagination: PaginationTwitch,
    pub total: u128,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PaginationTwitch {
    cursor: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatterList {
    pub user_id: String,
    pub user_login: String,
    pub user_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct AccessTokenResponseTwitch {
    pub access_token: String,
    pub expires_in: u32,
    pub refresh_token: String,
    pub scope: Vec<String>,
    pub token_type: String,
}

#[derive(Serialize, Deserialize)]
pub struct GetUsersTwitch {
    pub data: Vec<UsersTwitchApi>,
}

#[derive(Serialize, Deserialize)]
pub struct UsersTwitchApi {
    pub id: String,
    login: String,
    display_name: String,
    broadcaster_type: String,
    description: String,
    profile_image_url: String,
    offline_image_url: String,
    view_count: u32,
    created_at: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageTwitchEvent {
    pub broadcaster_user_id: String,
    pub broadcaster_user_login: String,
    pub broadcaster_user_name: String,
    pub source_broadcaster_user_id: Option<String>,
    pub source_broadcaster_user_login: Option<String>,
    pub source_broadcaster_user_name: Option<String>,
    pub chatter_user_id: String,
    pub chatter_user_login: String,
    pub chatter_user_name: String,
    pub message_id: String,
    pub source_message_id: Option<String>,
    pub is_source_only: Option<String>,
    pub message: MessageEventData,
    pub color: String,
    pub badges: Vec<MessageEventBadges>,
    pub source_badges: Option<String>,
    pub message_type: String,
    pub cheer: Option<String>,
    pub reply: Option<String>,
    pub channel_points_custom_reward_id: Option<String>,
    pub channel_points_animation_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]

pub struct MessageEventData {
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageEventBadges {
    pub set_id: String,
    pub id: String,
    pub info: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RedeemTwitchEvent {
    pub broadcaster_user_id: String,
    pub broadcaster_user_login: String,
    pub broadcaster_user_name: String,
    pub id: String,
    pub user_id: String,
    pub user_login: String,
    pub user_name: String,
    pub user_input: String,
    pub status: String,
    pub redeemed_at: String,
    pub reward: RedeemTwitchEventReward,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct RedeemTwitchEventReward {
    pub id: String,
    pub title: String,
    pub prompt: String,
    pub cost: u128,
}
