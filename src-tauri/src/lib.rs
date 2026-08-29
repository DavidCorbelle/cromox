// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
// Made by David Corbelle García
use reqwest::{self, Error, Response, StatusCode};
use std::env;
use std::io::{Read, Write};
use std::net::TcpListener;
use tauri::{AppHandle, Emitter};

#[path = "functions/commandsTwitch/command_twitch.rs"]
mod command_twitch;
#[path = "consts.rs"]
mod consts;
#[path = "functions/files/file_controller.rs"]
mod file_controller;
#[path = "migrations.rs"]
mod migrations_db;
#[path = "secret_const.rs"]
mod secret_const;
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
        let response_string2 = response.unwrap().text().await.unwrap();
        println!("{response_string2}");
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
            tokio::spawn(file_controller::check_tokens(app.clone()));
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
async fn implement_suscribers(session_id: &str, app: AppHandle) -> Result<String, String> {
    let response: Result<StatusCode, Error> =
        websocket_twitch::implement_suscribers(session_id).await;
    if response.is_ok() {
        let response_processed: StatusCode = response.unwrap();
        if response_processed.is_client_error() {
            tokio::spawn(file_controller::check_tokens(app));
            Ok(format!("Error al iniciar el suscriber"))
        } else {
            Ok(format!(
                "Hello, {}! You've been greeted from Rust2222222!",
                response_processed
            ))
        }
    } else {
        Err(format!("Error al iniciar el suscriber"))
    }
}

#[tauri::command]
async fn execute_command(message_text_command: &str) -> Result<String, ()> {
    let _res: String = command_twitch::execute_command(message_text_command)
        .await
        .unwrap();
    Ok(_res)
}

#[tauri::command]
async fn get_url_token(token_type: String, app: AppHandle) -> Result<String, ()> {
    tokio::spawn(get_auth_token(token_type.clone(), app));
    Ok(secret_const::get_token_url(token_type))
}

async fn get_auth_token(token_type: String, app: AppHandle) {
    println!("test");
    let listener = TcpListener::bind(("127.0.0.1", 8080));
    if listener.is_ok() {
        println!("open port");
        let listener_tcp: TcpListener = listener.unwrap();
        for stream in listener_tcp.incoming() {
            let mut stream = stream.unwrap();
            let mut buffer = [0; 1024];
            stream.read(&mut buffer).unwrap();
            // Convert buffer to string and extract the request line
            let request = String::from_utf8_lossy(&buffer);
            let request_line = request.lines().next().unwrap();
            let url = request_line.split_whitespace().nth(1).unwrap();

            let url_split: Vec<&str> = url.split("?").collect();
            if url_split.len() > 1 {
                let params_string = url_split.get(1).unwrap();
                let params_unsplit: Vec<&str> = params_string.split("&").collect();
                for p in params_unsplit {
                    let param_split: Vec<&str> = p.split("=").collect();
                    let key: &str = param_split.get(0).unwrap();
                    if key == "code" {
                        let token: &str = param_split.get(1).unwrap();
                        let response = "HTTP/1.1 200 OK\r\nContent-Length: 48\r\nContent-Type: text/html\r\n\r\n<h1>Token Updated, you can close the window</h1>";
                        stream.write(response.as_bytes()).unwrap();
                        let res =
                            file_controller::save_token_auth(token, token_type.as_str()).await;
                        if res.is_ok() {
                            app.emit("token-updated", token_type.clone()).unwrap();
                        }
                        break;
                    }
                }
            }
        }
    }
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
            get_data_commands,
            edit_command,
            delete_command,
            execute_command,
            get_url_token
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
