use futures::future::err;
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use reqwest::{self, Error, Response};
use std::env;

#[path = "functions/files/file_controller.rs"]
mod file_controller;
#[path = "structs/structs_custom.rs"]
mod structs_custom;
#[path = "functions/websocket/websocketTwitch.rs"]
mod websocketTwitch;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn start_data_config() -> Result<String, ()> {
    let _res: String = file_controller::load_config().await.unwrap();
    let data_config = std::env::var("configLoaded").unwrap();
    let response: String;
    if data_config == "S" {
        let data_test: String = std::env::var("tokenBot").unwrap();
        if data_test != ""{
            response = String::from("LOADED");
        }else{
            response = String::from("NODATA");
        }
    } else {
        let _res_retry: String = file_controller::save_config().await.unwrap();
        response = String::from("NOTLOADED");
    }

    Ok(response)
}

#[tauri::command]
async fn implement_suscribers(sessionId: &str) -> Result<String, String> {
    let response: Result<Response, Error> = websocketTwitch::implement_suscribers2(sessionId).await;
    let response_processed = response.ok().unwrap().text().await.ok().unwrap();

    Ok(format!(
        "Hello, {}! You've been greeted from Rust2222222!",
        response_processed
    ))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]

pub fn run() {
    let _res = file_controller::save_config();
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            implement_suscribers,
            start_data_config
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
