use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TwitchResponseChatters {
    pub data: Vec<TwitchChatterList>,
    pub pagination: TwitchPagination,
    pub total: u128,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TwitchPagination {
    cursor: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TwitchChatterList {
    pub user_id: String,
    pub user_login: String,
    pub user_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct TwitchAccessTokenResponse {
    pub access_token: String,
    pub expires_in: u32,
    pub refresh_token: String,
    pub scope: Vec<String>,
    pub token_type: String,
}

#[derive(Serialize, Deserialize)]
pub struct TwitchGetUsers {
    pub data: Vec<TwitchUsersApi>,
}

#[derive(Serialize, Deserialize)]
pub struct TwitchUsersApi {
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
pub struct TwitchMessageEvent {
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
    pub message: TwitchMessageEventData,
    pub color: String,
    pub badges: Vec<TwitchMessageEventBadges>,
    pub source_badges: Option<String>,
    pub message_type: String,
    pub cheer: Option<String>,
    pub reply: Option<String>,
    pub channel_points_custom_reward_id: Option<String>,
    pub channel_points_animation_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]

pub struct TwitchMessageEventData {
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TwitchMessageEventBadges {
    pub set_id: String,
    pub id: String,
    pub info: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TwitchRedeemEvent {
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
    pub reward: TwitchRedeemEventReward,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct TwitchRedeemEventReward {
    pub id: String,
    pub title: String,
    pub prompt: String,
    pub cost: u128,
}


#[derive(Serialize, Deserialize, Clone)]
pub struct TwitchPointUserStruct {
    pub points: u32,
    pub user_id: String,
    pub time_watch_mins: u32,
    pub last_known_name: String,
    pub existe_db: bool,
}
