use reqwest::{header::CONTENT_TYPE, Client, Response};
use tauri::http::HeaderMap;

use crate::structs_custom;

pub async fn implement_suscribers2(session_id: &str) -> Result<Response, reqwest::Error> {
    let boradcaster_id: String = std::env::var("boradcaster_id").unwrap();
    let bot_id: String = std::env::var("bot_id").unwrap();
    let client_id: String = std::env::var("client_id").unwrap();
    let token_bot: String = std::env::var("tokenBot").unwrap();
    const URL: &str = "https://api.twitch.tv/helix/eventsub/subscriptions";
    let client: Client = reqwest::Client::new();
    let mut headers: HeaderMap = HeaderMap::new();
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
    headers.insert(
        "Client-Id",
        client_id.parse().unwrap(),
    );
    headers.insert(
        "Authorization",
        format!("Bearer {}", token_bot).parse().unwrap(),
    );
    let json_fake: structs_custom::BodyRequestSuscriber = structs_custom::BodyRequestSuscriber {
        type_string: String::from("channel.chat.message"),
        version: String::from("1"),
        condition: structs_custom::ConditionStruct {
            broadcaster_user_id: boradcaster_id,
            user_id: bot_id,
        },
        transport: structs_custom::TransportStruct {
            session_id: String::from(session_id),
            method: String::from("websocket"),
        },
    };
    let json_string: Option<String> = serde_json::to_string(&json_fake).ok();
    let json_clone: String = String::from(json_string.unwrap()).replace("type_string", "type");
    //let json_patata: &str = json_clone.as_str();
    print!("test");
    let response: Response = client
        .post(URL)
        .body(json_clone)
        .headers(headers)
        .send()
        .await?;
    Ok(response)
}
