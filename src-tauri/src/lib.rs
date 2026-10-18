use crate::consts::{TABLE_VTUBESTUDIO_EXPRESSIONS, TABLE_VTUBESTUDIO_MODELS};
// Made by David Corbelle García
use crate::structs_custom::{CommandStruct};
use reqwest::{self, Error, Response, StatusCode};
use std::env;
use std::io::{Read, Write};
use std::net::TcpListener;
use tauri::{AppHandle, Emitter};
mod toku_custom_modules;
use crate::toku_custom_modules::toku_twitch::structs_twitch::{TwitchMessageEvent, TwitchPointUserStruct, TwitchRedeemEvent};
use crate::toku_custom_modules::toku_twitch::twitch_actions::twitch_points;
use crate::toku_custom_modules::toku_twitch::twitch_commands::execute_command_bot;
use crate::toku_custom_modules::toku_twitch::twitch_websocket::{twitch_implement_subscribers, twitch_send_message};
use crate::toku_custom_modules::toku_vtubestudio::structs_vtubestudio::{VtubestudioDataShorcutFront, VTUBESTUDIO_ACTIONS};
use crate::toku_custom_modules::toku_vtubestudio::vtubestudio::{get_token_vtubestudio, save_expresions_vtubestudio, save_models_vtubestudio, send_websocket_vtubestudio};
use crate::toku_custom_modules::toku_db::db_controller::{db_check_tokens, db_delete_command, db_edit_command, db_get_command_by_redeem_title, db_get_command_by_trigger, db_get_commands_string, db_get_points_user, db_get_vtubestudio_config, db_load_config_token, db_save_new_command, db_save_points_user, db_save_shorcut_multiple, db_save_token_auth};
mod consts;
mod secret_const;
mod migrations;
mod structs_custom;




#[tauri::command]
fn get_bot_id() -> String {
    env::var("bot_id").ok().unwrap_or(String::from(""))
}

#[tauri::command]
async fn send_message_twitch(message: &str) -> Result<String, ()> {
    let response: Result<Response, reqwest::Error> =
        twitch_send_message(message).await;
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
    let response: Result<String, ()> = db_save_new_command(command_data).await;
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
    let response: Result<String, ()> = db_edit_command(command_id, command_data).await;
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
    let response: Result<String, ()> = db_delete_command(command_id).await;
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
    let _res: String = db_load_config_token()
        .await
        .unwrap_or(String::from("Error"));
    let data_config: String = std::env::var("configLoaded")
        .ok()
        .unwrap_or(String::from("Error"));
    let response: String;
    if data_config == "S" {
        let data_test: String = std::env::var("tokenBot").unwrap_or(String::from(""));
        if data_test != "" {
            tokio::spawn(db_check_tokens(app.clone()));
            response = String::from("LOADED");
            let points_started: String = std::env::var("points_started")
                .ok()
                .unwrap_or(String::from("Error"));
            if points_started != "S" {
                tokio::spawn(twitch_points(app));
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
    let res: String = db_get_commands_string().await?;
    Ok(res)
}

#[tauri::command]
async fn implement_subscribers(session_id: &str, app: AppHandle) -> Result<String, String> {
    let response: Result<StatusCode, Error> =
        twitch_implement_subscribers(session_id).await;
    if response.is_ok() {
        let response_processed: StatusCode = response.unwrap();
        if response_processed.is_client_error() {
            tokio::spawn(db_check_tokens(app));
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
async fn execute_command_message(event_string: &str) -> Result<String, ()> {
    let event = serde_json::from_str(event_string);
    if event.is_err() {
        Ok(String::from("Error"))
    } else {
        let event_ok: TwitchMessageEvent = event.unwrap();
        let message_text_command = event_ok.message.text;
        let message_split: Vec<&str> = message_text_command.split(' ').collect();
        let command_trigger: String = message_split[0].replace("!", "");
        let user_bot_container: Vec<TwitchPointUserStruct> =
            db_get_points_user(event_ok.chatter_user_id).await?;
        let user_bot = user_bot_container.get(0).unwrap();

        let command: CommandStruct = db_get_command_by_trigger(command_trigger)
            .await
            .unwrap_or(CommandStruct::default());
        if user_bot.points >= command.point_cost {
            let _res: String =
                execute_command_bot(command.clone(), message_text_command.as_str())
                    .await
                    .unwrap();
            let actual_points = user_bot.points - command.point_cost;
            db_save_points_user(user_bot.user_id.clone(), actual_points).await;
            Ok(_res)
        } else {
            Ok(String::from(
                "No se ha podido lanzar el comando por falta de puntos",
            ))
        }
    }
}

#[tauri::command]
async fn execute_command_redeem(event_string: &str) -> Result<String, ()> {
    let event = serde_json::from_str(event_string);
    if event.is_err() {
        Ok(String::from("Error"))
    } else {
        let event_ok: TwitchRedeemEvent = event.unwrap();
        let command_trigger: String = event_ok.reward.title;
        let command: CommandStruct = db_get_command_by_redeem_title(command_trigger)
            .await
            .unwrap_or(CommandStruct::default());
        let _res: String = execute_command_bot(command, &event_ok.user_input.as_str())
            .await
            .unwrap();
        Ok(_res)
    }
}

#[tauri::command]
async fn get_url_token(token_type: String, app: AppHandle) -> Result<String, ()> {
    tokio::spawn(get_auth_token(token_type.clone(), app));
    Ok(secret_const::get_token_url(token_type))
}


#[tauri::command]
async fn start_config_vtubestudio(app: AppHandle) {
    let _res = get_token_vtubestudio(app).await;
}

#[tauri::command]
async fn action_new_shortcut() {
    std::env::set_var("ListenForNewShorcut", "S");
}

async fn get_auth_token(token_type: String, app: AppHandle) {
    let listener = TcpListener::bind(("127.0.0.1", 8080));
    if listener.is_ok() {
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
                        let res = db_save_token_auth(token, token_type.as_str()).await;
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
//VTUBESTUDIO
#[tauri::command]
async fn vtubestudio_save_shorcuts_models(data_send: String) {
    let data_shorcut: Vec<VtubestudioDataShorcutFront> = serde_json::from_str(&data_send).unwrap();
    tokio::spawn(db_save_shorcut_multiple(
        data_shorcut,
        TABLE_VTUBESTUDIO_MODELS,
    ));
}
#[tauri::command]
async fn vtubestudio_save_shorcuts_expressions(data_send: String) {
    let data_shorcut: Vec<VtubestudioDataShorcutFront> = serde_json::from_str(&data_send).unwrap();
    tokio::spawn(db_save_shorcut_multiple(
        data_shorcut,
        TABLE_VTUBESTUDIO_EXPRESSIONS,
    ));
}

#[tauri::command]
async fn get_config_vtubestudio() -> Result<String, String> {
    let config_vtubestudio = db_get_vtubestudio_config().await;
    if config_vtubestudio.is_ok() {
        let res:String = config_vtubestudio.unwrap();
        Ok(res)
    } else {
        Err(String::from(
            "{\"Error\":\"No se ha podido cargar la configuracion\"}",
        ))
    }
}

#[tauri::command]
async fn actions_vtubestudio(action: VTUBESTUDIO_ACTIONS, param: String) -> Result<String, ()> {
    let mut response = String::from("");

    match action {
        VTUBESTUDIO_ACTIONS::GET_MODELS => {
            let message: String = consts::vtubestudio_get_models();
            let response_raw: Result<String, tokio_tungstenite::tungstenite::Error> =
                send_websocket_vtubestudio(message).await;

            if response_raw.is_ok() {
                let response_string: String = response_raw.unwrap();
                response = save_models_vtubestudio(response_string).await?;
            }
        }
        VTUBESTUDIO_ACTIONS::SET_MODEL => {
            let message: String = consts::vtubestudio_set_model(param);
            response = send_websocket_vtubestudio(message)
                .await
                .unwrap_or(String::from("Error"));
        }
        VTUBESTUDIO_ACTIONS::GET_EXPRESSIONS_MODEL => {
            let message: String = consts::vtubestudio_get_expressions_current_model();
            let response_raw = send_websocket_vtubestudio(message)
                .await;
             if response_raw.is_ok() {
                let response_string: String = response_raw.unwrap();
                response = save_expresions_vtubestudio(response_string).await?;
            }
        
        }
        _ => {}
    }
    Ok(response)
}


//INIT
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    env::set_var("RUST_BACKTRACE", "1");
    let migrations: Vec<tauri_plugin_sql::Migration> = migrations::get_migrations();

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
            implement_subscribers,
            start_data_config,
            send_message_twitch,
            save_new_command,
            get_data_commands,
            edit_command,
            delete_command,
            execute_command_message,
            get_url_token,
            actions_vtubestudio,
            execute_command_redeem,
            start_config_vtubestudio,
            action_new_shortcut,
            vtubestudio_save_shorcuts_models,
            vtubestudio_save_shorcuts_expressions,
            get_config_vtubestudio
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
