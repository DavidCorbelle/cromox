// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
// Made by David Corbelle García
use reqwest::{self, Error, Response};
use std::env;

#[path = "functions/files/file_controller.rs"]
mod file_controller;
#[path = "structs/structs_custom.rs"]
mod structs_custom;
#[path = "functions/websocket/websocketTwitch.rs"]
mod websocket_twitch;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn send_message_twitch(message: &str) -> Result<String, ()> {
    let response: Result<Response, reqwest::Error> =
        websocket_twitch::send_message_twitch(message).await;
    let response_string: String;
    if response.is_ok() {
        response_string = String::from("Mensaje enviado con exito");
    } else {
        response_string = String::from("Ha ocurrido un error");
    }
    Ok(response_string)
}

#[tauri::command]
async fn save_new_command(command_data: String) -> Result<String, ()> {
    let response: Result<String, ()> =
        file_controller::save_new_command(command_data).await;
    let response_string: String;
    if response.is_ok() {
        response_string = String::from("Comando guardado con exito");
    } else {
        response_string = String::from("Ha ocurrido un error");
    }
    Ok(response_string)
}

#[tauri::command]
async fn start_data_config() -> Result<String, ()> {
    let _res: String = file_controller::load_config_token().await.unwrap();
    let data_config: String  = std::env::var("configLoaded").ok().unwrap_or(String::from("Error"));
    let response: String;
    if data_config == "S" {
        let data_test: String = std::env::var("tokenBot").unwrap_or(String::from(""));
        if data_test != "" {
            response = String::from("LOADED");
        } else {
            response = String::from("NODATA");
        }
    } else {
        let _res_retry: String = file_controller::save_config().await.unwrap();
        response = String::from("NOTLOADED");
    }

    Ok(response)

}
#[tauri::command]
async fn save_new_data_token(new_config_token:String) -> Result<String, ()> {
    let _res_save: String = file_controller::create_config_token(new_config_token).await.unwrap();
    let _res: String = file_controller::load_config_token().await.unwrap();
    let data_config: String  = std::env::var("configLoaded").ok().unwrap_or(String::from("Error"));
    let response: String;
    if data_config == "S" {
        let data_test: String = std::env::var("tokenBot").unwrap_or(String::from(""));
        if data_test != "" {
            response = String::from("LOADED");
        } else {
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
    let response: Result<Response, Error> =
        websocket_twitch::implement_suscribers2(sessionId).await;
    let response_processed: String = response.ok().unwrap().text().await.ok().unwrap();

    Ok(format!(
        "Hello, {}! You've been greeted from Rust2222222!",
        response_processed
    ))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            implement_suscribers,
            start_data_config,
            send_message_twitch,
            save_new_command,
            save_new_data_token
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
