use std::sync::Mutex;

use futures_util::{SinkExt, StreamExt};
use tauri::{AppHandle, Manager};
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;

use crate::{
    consts::{self},
    structs_custom::AppRunnigConfig,
    structs_vtubestudio::AuthResponseVtubestudio,
};

pub async fn get_token_vtubestudio(app: AppHandle) {
    let url: &str = "ws://0.0.0.0:8001";
    match connect_async(url).await {
        Ok((mut ws, _)) => {
            let _ = ws.send(Message::text("{\"apiName\": \"VTubeStudioPublicAPI\",\"apiVersion\": \"1.0\",\"requestID\": \"SomeID\",\"messageType\": \"AuthenticationTokenRequest\",\"data\": {\"pluginName\": \"Cromox\",\"pluginDeveloper\": \"Toku_Doku\",\"pluginIcon\": \"\"}}")).await;

            let msg_response = ws.next().await;
            print!("existe");
            if msg_response.is_some() {
                let msg = msg_response.unwrap().unwrap();
                println!("received: {msg}");
                let data: AuthResponseVtubestudio = serde_json::from_str(&msg.to_string()).unwrap();
                let token_string = data.data.authenticationToken.clone();
                let token = token_string.clone();
                let state = app.state::<Mutex<AppRunnigConfig>>();
                // Lock the mutex to get mutable access:
                let mut state = state.lock().unwrap();
                // Modify the state:
                state.token_vtubestudio = token;
            }
            //TODO: quitar esta llamada ya que es de prueba solo para probar el token con otra funcion
        }
        Err(e) => println!("connect failed: {e}"),
    }
}


pub async fn send_websocket_vtubestudio(message: String, app: AppHandle) -> Result<String,tokio_tungstenite::tungstenite::Error> {
    let url: &str = "ws://0.0.0.0:8001";
    //TODO poner este struct en una const que lo devuelva con el auth token
    let string_message = consts::vtubestudio_get_auth_string(app);
    match connect_async(url).await {
        Ok((mut ws, _)) => {
            let _a = ws.send(Message::text(string_message)).await;

            let msg_response = ws.next().await;
            if msg_response.is_some() {
                let msg = msg_response.unwrap().unwrap();
                println!("received: {msg}");
            }
            let _b = ws.send(Message::text(message)).await;
            let msg_response = ws.next().await;
            if msg_response.is_some() {
                let msg = msg_response.unwrap().unwrap();
                println!("{msg}");
                Ok(msg.to_string())
            }else {
                Ok(String::from("Respuesta Vacia"))
            }
        }
        Err(e) => Err(e)
    }
}
