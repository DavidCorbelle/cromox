use crate::structs_custom::{CommandStruct};
use crate::toku_custom_modules::toku_twitch::twitch_internal::reproduce_sound;
use crate::toku_custom_modules::toku_twitch::twitch_websocket::twitch_send_message;

pub async fn execute_command_bot(command:CommandStruct, _message_string:&str) -> Result<String, String> {
    if command != CommandStruct::default() && command.enabled == true {
        if command.response_text != String::from("") {
            let _res: Result<reqwest::Response, reqwest::Error> =
                twitch_send_message(&command.response_text).await;
        }
        if command.sound != Default::default() && command.sound.sound_dir != String::from("") {
            tokio::spawn(reproduce_sound(
                command.sound.sound_dir,
                command.sound.sound_volume,
            ));
        }
    }
    Ok(String::from("OK"))
}





