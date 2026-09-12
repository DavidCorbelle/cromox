use futures_util::{SinkExt, StreamExt};
use tauri::{AppHandle, Emitter};
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;

use crate::{
    consts::{self}, db_controller, intg_linux::start_check_hotkeys, structs_vtubestudio::{AuthResponseVtubestudio, ExpressionslDataFront, GetDataVtubestudio, ModelDataFront},
};

const INTEGRATION_NAME: &'static str = "VtubeStudio";
pub async fn get_token_vtubestudio(app: AppHandle) {
    let token = db_controller::get_token_integration(INTEGRATION_NAME)
        .await
        .unwrap_or(String::from(""));
    let token_exist = token != String::from("");
    if !token_exist {
        let url: &str = "ws://0.0.0.0:8001";
        match connect_async(url).await {
            Ok((mut ws, _)) => {
                let _ = ws.send(Message::text("{\"apiName\": \"VTubeStudioPublicAPI\",\"apiVersion\": \"1.0\",\"requestID\": \"SomeID\",\"messageType\": \"AuthenticationTokenRequest\",\"data\": {\"pluginName\": \"Cromox\",\"pluginDeveloper\": \"Toku_Doku\",\"pluginIcon\": \"\"}}")).await;

                let msg_response = ws.next().await;
                print!("existe");
                if msg_response.is_some() {
                    let msg = msg_response.unwrap().unwrap();
                    let data: AuthResponseVtubestudio =
                        serde_json::from_str(&msg.to_string()).unwrap();
                    let token_string = data.data.authenticationToken.clone();
                    let token = token_string.clone();
                    let _rdb = db_controller::save_token_integration(INTEGRATION_NAME, token).await;
                    let _r = app.emit("integration-started", INTEGRATION_NAME);
                    tokio::spawn(start_check_hotkeys(app.clone()));
                }
            }
            Err(e) => println!("connect failed: {e}"),
        }
    } else {
        let _r = app.emit("integration-started", INTEGRATION_NAME);
        tokio::spawn(start_check_hotkeys(app.clone()));
    }
}

pub async fn send_websocket_vtubestudio(
    message: String,
) -> Result<String, tokio_tungstenite::tungstenite::Error> {
    let url: &str = "ws://0.0.0.0:8001";
    let token = db_controller::get_token_integration(INTEGRATION_NAME)
        .await
        .unwrap_or(String::from(""));
    let string_message = consts::vtubestudio_get_auth_string(token);
    match connect_async(url).await {
        Ok((mut ws, _)) => {
            let _a = ws.send(Message::text(string_message)).await;

            let _msg_response = ws.next().await;
            let _b = ws.send(Message::text(message)).await;
            let msg_response = ws.next().await;
            if msg_response.is_some() {
                println!("responde");
                let msg = msg_response.unwrap().unwrap();
                println!("{:?}",msg.clone());
                Ok(msg.to_string())
            } else {
                Ok(String::from("Respuesta Vacia"))
            }
        }
        Err(_e) => Ok(String::from("Respuesta Vacia")),
    }
}

pub async fn save_models(data_string: String) -> Result<String, ()> {
    let data_p: GetDataVtubestudio = serde_json::from_str(&data_string).unwrap();
    if data_p.data.is_some() {
        let data = data_p.data.unwrap();
        if data.availableModels.is_some() {
            let models = data.availableModels.unwrap();
            for m in models {
                let _r = db_controller::save_model_vtubestudio_api(m.clone()).await;
            }
            let models_database: Vec<ModelDataFront> =
                db_controller::get_all_models_vtubestudio().await.unwrap();
            let models_response: String = serde_json::to_string(&models_database).unwrap();
            Ok(models_response)
        } else {
            Err(())
        }
    } else {
        Err(())
    }
}

pub async fn save_expresions(data_string: String) -> Result<String, ()> {
    let data_p: GetDataVtubestudio = serde_json::from_str(&data_string).unwrap();
    println!("{:?}", data_p);
    if data_p.data.is_some() {
        let data = data_p.data.unwrap();
        if data.expressions.is_some() {
            let model = data.modelID.unwrap();
            let expressions = data.expressions.unwrap();
            for e in expressions {
                let _r = db_controller::save_expressions_vtubestudio_api(e.clone(), model.clone()).await;
            }
            println!("llega");
            let expressions_database: Vec<ExpressionslDataFront> =
                db_controller::get_all_expressions_vtubestudio().await.unwrap();
            let expressions_response: String = serde_json::to_string(&expressions_database).unwrap();
            Ok(expressions_response)
        } else {
            Err(())
        }
    } else {
        Err(())
    }
}
