use tauri::{self};

use crate::structs_custom::{self, BotCommandContainer, PointUserTwitchStruct};
use sqlx::{
    sqlite::{SqliteConnectOptions, SqliteQueryResult},
    ConnectOptions, Connection, Row, SqlitePool,
};
use std::{
    fs::File,
    io::{Read, Write},
    str::FromStr,
};

const TOKEN_PATH: &str = "token.json";
const COMMANDS_PATH: &str = "commands.json";
const POINTS_PATH: &str = "points.json";
const DATABASE_PATH: &str = "database.db";

fn save_file_if_exist(data_string: String, file_name: &str) {
    let path: String = get_file_path(file_name);
    let file: Result<File, std::io::Error> = File::create(path);
    if file.is_ok() {
        let mut file_writable = file.ok().unwrap();
        file_writable.write_all(data_string.as_bytes()).ok();
    }
}

fn get_config_path(file_name: &str) -> String {
    let app: tauri::Context = tauri::generate_context!();
    let identifier: String = app.config().identifier.clone();
    let mut base_dir_path: std::path::PathBuf = dirs::config_dir().unwrap();
    base_dir_path.push(identifier);
    let base_dir = base_dir_path.as_path().to_str().unwrap();
    let path: String = format!("{base_dir}/{file_name}");
    return path;
}

fn get_file_path(file_name: &str) -> String {
    let app: tauri::Context = tauri::generate_context!();
    let identifier: String = app.config().identifier.clone();
    let mut base_dir_path: std::path::PathBuf = dirs::data_dir().unwrap();
    base_dir_path.push(identifier);
    let base_dir = base_dir_path.as_path().to_str().unwrap();
    let path: String = format!("{base_dir}/{file_name}");
    return path;
}

fn config_struct() -> structs_custom::JSONConfig {
    let data_struct: structs_custom::JSONConfig = structs_custom::JSONConfig {
        boradcaster_id: std::env::var("boradcaster_id")
            .ok()
            .unwrap_or(String::from("")),
        bot_id: std::env::var("bot_id").ok().unwrap_or(String::from("")),
        client_id: std::env::var("client_id").ok().unwrap_or(String::from("")),
        client_secret: std::env::var("client_secret")
            .ok()
            .unwrap_or(String::from("")),
        redirect_uri: std::env::var("redirect_uri")
            .ok()
            .unwrap_or(String::from("")),
        token: std::env::var("tokenBot").ok().unwrap_or(String::from("")),
    };
    return data_struct;
}

pub async fn create_config_token(data_new_env: String) -> Result<String, ()> {
    save_file_if_exist(data_new_env.clone(), TOKEN_PATH);
    Ok(String::from(data_new_env))
}

pub async fn load_config_token() -> Result<String, ()> {
    let file: Result<File, std::io::Error> = File::open(get_file_path(TOKEN_PATH));
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
    let data_new_env: structs_custom::JSONConfig = config_struct();
    let data_string: String = serde_json::to_string(&data_new_env)
        .ok()
        .unwrap_or(String::from("{}"));
    save_file_if_exist(data_string, TOKEN_PATH);
    Ok(String::from("Config saved"))
}

pub fn get_commands() -> Option<structs_custom::BotCommandContainer> {
    let file: Result<File, std::io::Error> = File::open(get_file_path(COMMANDS_PATH));
    let return_data: structs_custom::BotCommandContainer;
    if file.is_ok() {
        let mut new_file: File = file.unwrap();
        let mut contents: String = String::new();
        let _readed: Result<usize, std::io::Error> = new_file.read_to_string(&mut contents);
        let data_new_env: Result<structs_custom::BotCommandContainer, serde_json::Error> =
            serde_json::from_str(contents.as_str());
        if data_new_env.is_ok() {
            return_data = data_new_env.unwrap();
            return Some(return_data);
        }
    }
    return Default::default();
}

pub async fn get_commands_string() -> Result<String, ()> {
    let file: Result<File, std::io::Error> = File::open(get_file_path(COMMANDS_PATH));
    let string_retun: String;
    if file.is_ok() {
        let mut new_file: File = file.unwrap();
        let mut contents: String = String::new();
        let _readed: Result<usize, std::io::Error> = new_file.read_to_string(&mut contents);
        let data_new_env: Result<structs_custom::BotCommandContainer, serde_json::Error> =
            serde_json::from_str(contents.as_str());
        if data_new_env.is_ok() {
            string_retun = contents
        } else {
            string_retun = String::from("Error al cargar los datos");
        }
    } else {
        string_retun = String::from("{ \"response\": \"Error al cargar el archivo\"}");
    }
    Ok(string_retun)
}

pub async fn save_new_command(data: String) -> Result<String, ()> {
    let mut comands_container: structs_custom::BotCommandContainer =
        get_commands().unwrap_or_default();
    let mut new_command: structs_custom::CommandStruct =
        serde_json::from_str(&data.as_str()).unwrap();
    if comands_container != BotCommandContainer::default() {
        let last_id: u16;
        if comands_container.commands.len() > 0 {
            let last_id_command: Option<&structs_custom::CommandStruct> =
                comands_container.commands.last();
            last_id = last_id_command.unwrap().command_id;
        } else {
            last_id = 0;
        }
        new_command.command_id = last_id + 1;
        comands_container.commands.push(new_command);
    } else {
        new_command.command_id = 1;
        comands_container = structs_custom::BotCommandContainer {
            commands: vec![new_command],
        };
    }
    let data_string: String = serde_json::to_string(&comands_container)
        .ok()
        .unwrap_or(String::from("{}"));
    save_file_if_exist(data_string, COMMANDS_PATH);
    Ok(String::from("Ok"))
}

pub async fn edit_command(id: u16, data: String) -> Result<String, ()> {
    let mut commands: structs_custom::BotCommandContainer = get_commands().unwrap_or_default();
    let new_command: structs_custom::CommandStruct = serde_json::from_str(&data.as_str()).unwrap();
    if commands != BotCommandContainer::default() {
        let searcher: Vec<structs_custom::CommandStruct> = commands.commands.clone();
        let index = searcher.iter().position(|r| r.command_id == id).unwrap();
        commands.commands[index] = new_command;
        let data_string: String = serde_json::to_string(&commands)
            .ok()
            .unwrap_or(String::from("{}"));
        save_file_if_exist(data_string, COMMANDS_PATH);
    }
    Ok(String::from("Ok"))
}

pub async fn delete_command(id: u16) -> Result<String, ()> {
    let mut commands: structs_custom::BotCommandContainer = get_commands().unwrap_or_default();

    if commands != BotCommandContainer::default() {
        let searcher: Vec<structs_custom::CommandStruct> = commands.commands.clone();
        let index = searcher.iter().position(|r| r.command_id == id).unwrap();
        commands.commands.remove(index);
        let data_string: String = serde_json::to_string(&commands)
            .ok()
            .unwrap_or(String::from("{}"));
        save_file_if_exist(data_string, COMMANDS_PATH);
    }
    Ok(String::from("Ok"))
}

pub async fn get_all_points_user() -> Result<Vec<PointUserTwitchStruct>, ()> {
    let con_options =
        SqliteConnectOptions::from_str(&format!("sqlite://{}", get_config_path(DATABASE_PATH)))
            .unwrap();
    let con: SqlitePool = SqlitePool::connect_with(con_options).await.unwrap();
    let mut return_data: Vec<PointUserTwitchStruct> = vec![];

    let result: Result<Vec<sqlx::sqlite::SqliteRow>, sqlx::Error> =
        sqlx::query("SELECT * FROM users_twitch")
            .fetch_all(&con)
            .await;
    if result.is_ok() {
        let result_query: Vec<sqlx::sqlite::SqliteRow> = result.unwrap();
        for i in result_query {
            return_data.push(structs_custom::PointUserTwitchStruct {
                points: i.get("points"),
                user_id: i.get("id"),
                time_watch_mins: i.get("time_watch_mins"),
                last_known_name: i.get("name"),
                existe_db: true,
            });
        }
    }
    con.close().await;
    Ok(return_data)
}

pub async fn save_all_points_user(points_file_data: Vec<structs_custom::PointUserTwitchStruct>) {
    let insert: Vec<_> = points_file_data
        .iter()
        .filter(|&x| x.existe_db == false)
        .collect();
    let update: Vec<_> = points_file_data
        .iter()
        .filter(|&x| x.existe_db == true)
        .collect();
    let con_options =
        SqliteConnectOptions::from_str(&format!("sqlite://{}", get_config_path(DATABASE_PATH)))
            .unwrap();
    let con: SqlitePool = SqlitePool::connect_with(con_options).await.unwrap();
    println!("Guarda");
    let data_string = serde_json::to_string(&points_file_data).unwrap();
    println!("{data_string}");
    for i in insert {
        println!("intenta insertar");
        let _result: SqliteQueryResult = sqlx::query(
            "INSERT into users_twitch (id , name, points,time_watch_mins ) VALUES ($1,$2,$3,$4)",
        )
        .bind(i.user_id.clone())
        .bind(i.last_known_name.clone())
        .bind(i.points)
        .bind(i.time_watch_mins)
        .execute(&con)
        .await
        .unwrap();
    }
    for u in update {
        println!("intenta actualizar");
        let _result: SqliteQueryResult  = sqlx::query(
            "UPDATE users_twitch SET   name = $2, points =$3,time_watch_mins =$4  WHERE id = $1",
        )
        .bind(u.user_id.clone())
        .bind(u.last_known_name.clone())
        .bind(u.points)
        .bind(u.time_watch_mins)
        .execute(&con)
        .await.unwrap();
    }
    con.close().await;
}
