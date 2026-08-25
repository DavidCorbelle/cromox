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
