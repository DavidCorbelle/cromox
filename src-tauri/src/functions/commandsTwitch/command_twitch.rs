use std::fs::File;
use std::io::BufReader;

use crate::file_controller;
use crate::structs_custom::{self};
use crate::websocket_twitch;

pub async fn execute_command(message_text_command: &str) -> Result<String, String> {
    let comands_list: structs_custom::BotCommandContainer =
        file_controller::get_commands().unwrap_or_default();
    let message_split: Vec<&str> = message_text_command.split(' ').collect();
    let command_trigger: String = message_split[0].replace("!", "");
    let index: usize = comands_list
        .commands
        .iter()
        .position(|r: &structs_custom::CommandStruct| r.trigger == command_trigger)
        .unwrap();
    let command: structs_custom::CommandStruct = comands_list.commands[index].clone();
    if command.response_text != String::from("") {
        let _res: Result<reqwest::Response, reqwest::Error> =
            websocket_twitch::send_message_twitch(&command.response_text).await;
    }
    if command.sound != Default::default() && command.sound.sound_dir != String::from("") {
        tokio::spawn(reproduce_sound(
            command.sound.sound_dir,
            command.sound.sound_volume,
        ));
    }

    Ok(String::from("OK"))
}

async fn reproduce_sound(sound_dir: String, sound_volume: u8) -> Result<String, ()> {
    let sink_handle =
        rodio::DeviceSinkBuilder::open_default_sink().expect("open default audio stream");
    let file = BufReader::new(File::open(sound_dir).unwrap());
    let player = rodio::play(&sink_handle.mixer(), file).unwrap();
    let volume_float: f32 = sound_volume as f32;
    player.set_volume(volume_float / 100.0);
    player.play();
    player.sleep_until_end();
    Ok(String::from("Sound ended"))
}
