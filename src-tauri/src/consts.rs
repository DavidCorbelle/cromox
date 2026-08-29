pub fn get_access_token_url() -> String {
    return String::from("https://id.twitch.tv/oauth2/token");
}

pub const SCOPES_BOT: &'static [&'static str] = &["moderator:read:chatters","channel:bot","user:write:chat","user:read:chat"];
pub const SCOPES_STREAMER: &'static [&'static str] = &["user:bot"];
