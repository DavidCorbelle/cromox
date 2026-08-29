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
pub struct  AccessTokenResponseTwitch {
    pub access_token:String,
    pub expires_in:u32,
    pub refresh_token:String,
    pub scope:Vec<String>,
    pub token_type:String
    
}

#[derive(Serialize, Deserialize)]
pub struct GetUsersTwitch{
    pub data:Vec<UsersTwitchApi>
}

#[derive(Serialize, Deserialize)]
pub struct UsersTwitchApi{
            pub id: String,
            login: String,
            display_name: String,
            broadcaster_type: String,
            description: String,
            profile_image_url: String,
            offline_image_url: String,
            view_count: u32,
            created_at: String
}
