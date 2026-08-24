use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ResponseChatters {
    pub data: Vec<ChatterList>,
    pub pagination: PaginationTwitch,
    pub total: u128,
}

#[derive(Serialize, Deserialize)]
pub struct PaginationTwitch {
    cursor: String,
}

#[derive(Serialize, Deserialize)]
pub struct ChatterList {
    pub user_id: String,
    pub user_login: String,
    pub user_name: String,
}
