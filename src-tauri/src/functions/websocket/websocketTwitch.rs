use reqwest::{header::CONTENT_TYPE, Client, Response, StatusCode};
use tauri::http::HeaderMap;

use crate::{
    secret_const::{BOT_TOKEN_TYPE, CLIENT_ID, STREAMER_TOKEN_TYPE},
    structs_custom, structs_twitch_api,
};

fn get_auth_headers(type_token: &str) -> HeaderMap {
    let mut token: String = String::from("");
    if type_token == STREAMER_TOKEN_TYPE {
        token = std::env::var("tokenStreamer").unwrap_or(String::from(""));
    } else if type_token == BOT_TOKEN_TYPE {
        token = std::env::var("tokenBot").unwrap_or(String::from(""));
    }
    let mut headers: HeaderMap = HeaderMap::new();
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
    headers.insert("Client-Id", CLIENT_ID.parse().unwrap());
    headers.insert(
        "Authorization",
        format!("Bearer {}", token).parse().unwrap(),
    );
    return headers;
}
pub async fn implement_suscribers(session_id: &str) -> Result<StatusCode, reqwest::Error> {
    let broadcaster_id: String = std::env::var("broadcaster_id").unwrap_or(String::from(""));
    let bot_id: String = std::env::var("bot_id").unwrap_or(String::from(""));
    const URL: &str = "https://api.twitch.tv/helix/eventsub/subscriptions";
    let client: Client = reqwest::Client::new();
    let headers: HeaderMap = get_auth_headers(BOT_TOKEN_TYPE);
    let json_fake: structs_custom::BodyRequestSuscriber = structs_custom::BodyRequestSuscriber {
        type_string: String::from("channel.chat.message"),
        version: String::from("1"),
        condition: structs_custom::ConditionStruct {
            broadcaster_user_id: broadcaster_id,
            user_id: bot_id,
        },
        transport: structs_custom::TransportStruct {
            session_id: String::from(session_id),
            method: String::from("websocket"),
        },
    };
    let json_string: Option<String> = serde_json::to_string(&json_fake).ok();
    let json_clone: String = String::from(json_string.unwrap()).replace("type_string", "type");
    let response: Response = client
        .post(URL)
        .body(json_clone)
        .headers(headers)
        .send()
        .await?;
    
    let status = response.status();
    Ok(status)
}

pub async fn send_message_twitch(message: &str) -> Result<Response, reqwest::Error> {
    let client: Client = reqwest::Client::new();
    let broadcaster_id: String = std::env::var("broadcaster_id").unwrap();
    let bot_id: String = std::env::var("bot_id").unwrap();

    const URL: &str = "https://api.twitch.tv/helix/chat/messages";
    let headers: HeaderMap = get_auth_headers(BOT_TOKEN_TYPE);
    let json_data: structs_custom::BodyRequestSendMessageTwitch =
        structs_custom::BodyRequestSendMessageTwitch {
            broadcaster_id: String::from(broadcaster_id),
            sender_id: bot_id,
            message: String::from(message),
        };
    let json_string: Option<String> = serde_json::to_string(&json_data).ok();
    let json_clone: String = String::from(json_string.unwrap()).replace("type_string", "type");
    let response: Response = client
        .post(URL)
        .body(json_clone)
        .headers(headers)
        .send()
        .await?;
    Ok(response)
}

pub async fn get_chatters_twitch() -> Response {
    let client: Client = reqwest::Client::new();
    let broadcaster_id: String = std::env::var("broadcaster_id").unwrap();
    let bot_id: String = std::env::var("bot_id").unwrap();
    let url: String = format!("https://api.twitch.tv/helix/chat/chatters?broadcaster_id={broadcaster_id}&moderator_id={bot_id}&first=1000");
    let headers: HeaderMap = get_auth_headers(BOT_TOKEN_TYPE);
    let response: Response = client.get(url).headers(headers).send().await.unwrap();
    return response;
}

pub async fn get_id_user_twitch(type_token: &str) -> Result<String, ()> {
    let client: Client = reqwest::Client::new();
    let url: String = format!("https://api.twitch.tv/helix/users");
    let headers: HeaderMap = get_auth_headers(type_token);
    let response: Response = client.get(url).headers(headers).send().await.unwrap();
    let response_string = response.text().await.unwrap();
    let response_object: structs_twitch_api::GetUsersTwitch =
        serde_json::from_str(&response_string).unwrap();
    let user: &structs_twitch_api::UsersTwitchApi = response_object.data.get(0).unwrap();
    let user_id: String = user.id.clone();
    Ok(String::from(user_id))
}
