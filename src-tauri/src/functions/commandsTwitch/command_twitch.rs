use crate::structs_custom::{self};
use crate::file_controller;
use crate::websocket_twitch;

pub async fn execute_command(message_text_command:&str) -> Result<String, String>{
    let comands_list:structs_custom::BotCommandContainer = file_controller::get_commands().unwrap_or_default();
    let message_split:Vec<&str> = message_text_command.split(' ').collect();
    let command_trigger: String = message_split[0].replace("!", "");
    let index: usize = comands_list.commands.iter().position(|r: &structs_custom::CommandStruct| r.trigger == command_trigger).unwrap();
    let command: structs_custom::CommandStruct = comands_list.commands[index].clone();
    if command.response_text != String::from("") {
        let _res: Result<reqwest::Response, reqwest::Error> = websocket_twitch::send_message_twitch(&command.response_text).await;
    }
    Ok(String::from("OK"))
}