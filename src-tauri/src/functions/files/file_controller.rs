use crate::structs_custom::{self};
use std::{
    fs::File,
    io::{Read, Write},
};

pub async fn load_config() -> Result<String, ()> {
    let file: Result<File, std::io::Error> = File::open("config.json");
    if file.is_ok() {
        let mut new_file: File = file.unwrap();
        let mut contents: String = String::new();
        let _readed: Result<usize, std::io::Error> = new_file.read_to_string(&mut contents);
        let data_new_env: structs_custom::JSONConfig =
            serde_json::from_str(contents.as_str()).unwrap();
        std::env::set_var("boradcaster_id", data_new_env.boradcaster_id);
        std::env::set_var("bot_id", data_new_env.bot_id);
        std::env::set_var("client_id", data_new_env.client_id);
        std::env::set_var("client_secret", data_new_env.client_secret);
        std::env::set_var("redirect_uri", data_new_env.redirect_uri);
        std::env::set_var("tokenBot", data_new_env.token);
        std::env::set_var("configLoaded", "S");
    } else {
        std::env::set_var("configLoaded", "N");
    }

    Ok(String::from("loaded"))
}

pub async fn save_config() -> Result<String, ()> {
    println!("test1");
    let data_mew_env: structs_custom::JSONConfig = structs_custom::JSONConfig {
        boradcaster_id: std::env::var("boradcaster_id").ok().unwrap_or(String::from("")),
        bot_id: std::env::var("bot_id").ok().unwrap_or(String::from("")),
        client_id: std::env::var("client_id").ok().unwrap_or(String::from("")),
        client_secret: std::env::var("client_secret").ok().unwrap_or(String::from("")),
        redirect_uri: std::env::var("redirect_uri").ok().unwrap_or(String::from("")),
        token: std::env::var("tokenBot").ok().unwrap_or(String::from("")),
    };
    let data_string: String = serde_json::to_string(&data_mew_env).ok().unwrap_or(String::from("{}"));
    let file: Result<File, std::io::Error> = File::open("config.json");
    println!("test");
    if file.is_ok() {
        let mut file_writable = file.ok().unwrap();
        file_writable.write_all(data_string.as_bytes()).ok();
    } else {
        let mut file_writable = File::create("config.json").ok().unwrap();
        file_writable.write_all(data_string.as_bytes()).ok();
    }
    std::env::set_var("configLoaded", "S");
    Ok(String::from("Config saved"))
}


pub async fn get_commands() -> Result<String, ()> {
    let file: Result<File, std::io::Error> = File::open("commands.json");
    Ok(String::from("loaded"))
}

pub async fn save_commands() -> Result<String, ()> {
    println!("test1");
    /*let array_commands:= Box::<structs_custom::BotCommand>::new_uninit();
    let data_new_commands: structs_custom::BotCommandContainer = structs_custom::BotCommandContainer {
        commands:array_commands
    };
    let data_string: String = serde_json::to_string(&data_new_commands).ok().unwrap_or(String::from("{}"));
    let file: Result<File, std::io::Error> = File::open("commands.json");
    if file.is_ok() {
        let mut file_writable = file.ok().unwrap();
        file_writable.write_all(data_string.as_bytes()).ok();
    } else {
        let mut file_writable = File::create("commands.json").ok().unwrap();
        file_writable.write_all(data_string.as_bytes()).ok();
    }
    std::env::set_var("configLoaded", "S");*/
    Ok(String::from("Commands saved"))
}