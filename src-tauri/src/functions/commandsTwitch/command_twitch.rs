use crate::file_controller;
use crate::structs_custom::{self};
use crate::structs_twitch_api::{self};
use crate::websocket_twitch;
use std::fs::File;
use std::io::BufReader;
use tokio::time::{sleep, Duration};

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

pub async fn twitch_points() {
    loop {
        let mut changed: bool = false;
        let mut points_file: Vec<structs_custom::PointUserTwitchStruct> =
            file_controller::get_all_points_user().unwrap();
        let chatters: Vec<structs_twitch_api::ChatterList> = get_chatters_list().await;
        let old_chatters_string: String =
            std::env::var("old_chatters").unwrap_or(String::from("None"));
        if old_chatters_string != "None" {
            let old_chatters: Vec<structs_twitch_api::ChatterList> =
                serde_json::from_str(&old_chatters_string).ok().unwrap();
            for n in old_chatters {
                let index: Option<usize> = chatters
                    .iter()
                    .position(|r: &structs_twitch_api::ChatterList| r.user_id == n.user_id);
                if index.is_some() {
                    changed = true;
                    let index_file: Option<usize> =
                        points_file
                            .iter()
                            .position(|r: &structs_custom::PointUserTwitchStruct| {
                                r.user_id == n.user_id
                            });
                    if index_file.is_some() {
                        points_file[index_file.unwrap()].points += 1;
                        points_file[index_file.unwrap()].last_known_name = n.user_name;
                    } else {
                        points_file.push({
                            structs_custom::PointUserTwitchStruct {
                                points: 1,
                                user_id: n.user_id,
                                time_watch_mins: 5,
                                last_known_name: n.user_name,
                            }
                        });
                    }
                }
            }
        }
        if changed {
            file_controller::save_all_points_user(points_file);
        }
        //TODO Save new points
        let chatters_string = serde_json::to_string(&chatters).ok().unwrap();
        std::env::set_var("old_chatters", chatters_string);

        sleep(Duration::from_mins(5)).await
    }
}

async fn get_chatters_list() -> Vec<structs_twitch_api::ChatterList> {
    let mut listado_chatters: Vec<structs_twitch_api::ChatterList> = Vec::new();
    let listado_chatters_result: structs_twitch_api::ResponseChatters =
        websocket_twitch::get_chatters_twitch()
            .await
            .json()
            .await
            .unwrap();
    for c in listado_chatters_result.data {
        listado_chatters.push(structs_twitch_api::ChatterList {
            user_id: c.user_id,
            user_login: c.user_login,
            user_name: c.user_name,
        });
    }
    return listado_chatters;
}
