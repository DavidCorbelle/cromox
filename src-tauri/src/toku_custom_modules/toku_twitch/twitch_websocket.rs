use reqwest::{header::CONTENT_TYPE, Client, Response, StatusCode};
use tauri::http::HeaderMap;

use crate::{
    consts::SUBSCRIBERS_TWITCH,
    secret_const::{ CLIENT_ID},
    structs_custom,
    consts::{BOT_TOKEN_TYPE, STREAMER_TOKEN_TYPE}
};
use crate::toku_custom_modules::toku_twitch::structs_twitch::{TwitchGetUsers, TwitchUsersApi};

fn twitch_get_auth_headers(type_token: &str) -> HeaderMap {
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
pub async fn twitch_implement_subscribers(session_id: &str) -> Result<StatusCode, reqwest::Error> {
    let broadcaster_id: String = std::env::var("broadcaster_id").unwrap_or(String::from(""));
    const URL: &str = "https://api.twitch.tv/helix/eventsub/subscriptions";
    let client: Client = reqwest::Client::new();
    let headers: HeaderMap = twitch_get_auth_headers(STREAMER_TOKEN_TYPE);
    let mut status: StatusCode = StatusCode::ACCEPTED;
    for rw in SUBSCRIBERS_TWITCH {
        let json_fake: structs_custom::BodyRequestSuscriber =
            structs_custom::BodyRequestSuscriber {
                type_string: String::from(rw.to_owned()),
                version: String::from("1"),
                condition: structs_custom::ConditionStruct {
                    broadcaster_user_id: broadcaster_id.clone(),
                    user_id: broadcaster_id.clone(),
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
            .headers(headers.clone())
            .send()
            .await?;

        status = response.status();
        if status.is_client_error(){
            break;
        }
    }

    Ok(status)
}

pub async fn twitch_send_message(message: &str) -> Result<Response, reqwest::Error> {
    let client: Client = reqwest::Client::new();
    let broadcaster_id: String = std::env::var("broadcaster_id").unwrap();
    let bot_id: String = std::env::var("bot_id").unwrap();

    const URL: &str = "https://api.twitch.tv/helix/chat/messages";
    let headers: HeaderMap = twitch_get_auth_headers(BOT_TOKEN_TYPE);
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

pub async fn twitch_get_chatters() -> Response {
    let client: Client = reqwest::Client::new();
    let broadcaster_id: String = std::env::var("broadcaster_id").unwrap();
    let bot_id: String = std::env::var("bot_id").unwrap();
    let url: String = format!("https://api.twitch.tv/helix/chat/chatters?broadcaster_id={broadcaster_id}&moderator_id={bot_id}&first=1000");
    let headers: HeaderMap = twitch_get_auth_headers(BOT_TOKEN_TYPE);
    let response: Response = client.get(url).headers(headers).send().await.unwrap();
    return response;
}

pub async fn twitch_get_id_user(type_token: &str) -> Result<String, ()> {
    let client: Client = reqwest::Client::new();
    let url: String = format!("https://api.twitch.tv/helix/users");
    let headers: HeaderMap = twitch_get_auth_headers(type_token);
    let response: Response = client.get(url).headers(headers).send().await.unwrap();
    let response_string = response.text().await.unwrap();
    let response_object: TwitchGetUsers =
        serde_json::from_str(&response_string).unwrap();
    let user: &TwitchUsersApi = response_object.data.get(0).unwrap();
    let user_id: String = user.id.clone();
    Ok(String::from(user_id))
}
