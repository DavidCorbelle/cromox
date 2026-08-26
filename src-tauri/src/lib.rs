// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
// Made by David Corbelle García
use reqwest::{self, Error, Response};
use tauri::AppHandle;
use std::env;

#[path = "functions/commandsTwitch/command_twitch.rs"]
mod command_twitch;
#[path = "functions/files/file_controller.rs"]
mod file_controller;
#[path = "migrations.rs"]
mod migrations_db;
#[path = "structs/structs_custom.rs"]
mod structs_custom;
#[path = "structs/structs_twitch_api.rs"]
mod structs_twitch_api;
#[path = "functions/websocket/websocketTwitch.rs"]
mod websocket_twitch;

#[tauri::command]
fn get_bot_id() -> String {
    return std::env::var("bot_id").ok().unwrap_or(String::from(""));
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
    let response: Result<String, ()> = file_controller::save_new_command(command_data).await;
    let response_string: String;
    if response.is_ok() {
        response_string = String::from("Comando guardado con exito");
    } else {
        response_string = String::from("Ha ocurrido un error");
    }
    Ok(response_string)
}

#[tauri::command]
async fn edit_command(command_id: u16, command_data: String) -> Result<String, ()> {
    let response: Result<String, ()> =
        file_controller::edit_command(command_id, command_data).await;
    let response_string: String;
    if response.is_ok() {
        response_string = String::from("Comando guardado con exito");
    } else {
        response_string = String::from("Ha ocurrido un error");
    }
    Ok(response_string)
}

#[tauri::command]
async fn delete_command(command_id: u16) -> Result<String, ()> {
    let response: Result<String, ()> = file_controller::delete_command(command_id).await;
    let response_string: String;
    if response.is_ok() {
        response_string = String::from("Comando guardado con exito");
    } else {
        response_string = String::from("Ha ocurrido un error");
    }
    Ok(response_string)
}

#[tauri::command]
async fn start_data_config(app: AppHandle) -> Result<String, ()> {
    let _res: String = file_controller::load_config_token()
        .await
        .unwrap_or(String::from("Error"));
    let data_config: String = std::env::var("configLoaded")
        .ok()
        .unwrap_or(String::from("Error"));
    let response: String;
    if data_config == "S" {
        let data_test: String = std::env::var("tokenBot").unwrap_or(String::from(""));
        if data_test != "" {
            response = String::from("LOADED");
            let points_started: String = std::env::var("points_started")
                .ok()
                .unwrap_or(String::from("Error"));
            if points_started != "S" {
                tokio::spawn(command_twitch::twitch_points(app));
            }
        } else {
            response = String::from("NODATA");
        }
    } else {
        response = String::from("NOTLOADED");
    }

    Ok(response)
}
#[tauri::command]
async fn get_data_commands() -> Result<String, ()> {
    let res: String = file_controller::get_commands_string().await.unwrap();
    Ok(res)
}

#[tauri::command]
async fn save_new_data_token(new_config_token: String, app: AppHandle) -> Result<String, ()> {
    let _res_save = file_controller::create_config_token(new_config_token).await;
    let _res = file_controller::load_config_token().await;
    let data_config: String = std::env::var("configLoaded")
        .ok()
        .unwrap_or(String::from("Error"));
    let response: String;
    if data_config == "S" {
        let data_test: String = std::env::var("tokenBot").unwrap_or(String::from(""));
        if data_test != "" {
            response = String::from("LOADED");
            let points_started: String = std::env::var("points_started")
                .ok()
                .unwrap_or(String::from("Error"));
            if points_started != "S" {
                tokio::spawn(command_twitch::twitch_points(app));
            }
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
async fn implement_suscribers(session_id: &str) -> Result<String, String> {
    let response: Result<Response, Error> =
        websocket_twitch::implement_suscribers2(session_id).await;
    let response_processed: String = response.ok().unwrap().text().await.ok().unwrap();
    Ok(format!(
        "Hello, {}! You've been greeted from Rust2222222!",
        response_processed
    ))
}

#[tauri::command]
async fn execute_command(message_text_command: &str) -> Result<String, ()> {
    let _res: String = command_twitch::execute_command(message_text_command)
        .await
        .unwrap();
    Ok(_res)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    env::set_var("RUST_BACKTRACE", "1");

    let migrations: Vec<tauri_plugin_sql::Migration> = migrations_db::get_migrations();

    tauri::Builder::default()
        .plugin(
            tauri_plugin_sql::Builder::new()
                .add_migrations("sqlite:database.db", migrations)
                .build(),
        )
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_bot_id,
            implement_suscribers,
            start_data_config,
            send_message_twitch,
            save_new_command,
            save_new_data_token,
            get_data_commands,
            edit_command,
            delete_command,
            execute_command
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
