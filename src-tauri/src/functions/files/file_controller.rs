use crate::structs_custom::{self, CommandStruct, JSONConfig, PointUserTwitchStruct};
use sqlx::{
    sqlite::{SqliteConnectOptions, SqliteQueryResult},
    Row, SqlitePool,
};
use std::str::FromStr;
use tauri::{self};

const DATABASE_PATH: &str = "database.db";

async fn get_connection() -> Result<SqlitePool, ()> {
    let con_options =
        SqliteConnectOptions::from_str(&format!("sqlite://{}", get_config_path(DATABASE_PATH)))
            .unwrap();
    let con: SqlitePool = SqlitePool::connect_with(con_options).await.unwrap();
    Ok(con)
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
    let con: SqlitePool = get_connection().await.unwrap();
    let _delete: Result<SqliteQueryResult, sqlx::Error> =
        sqlx::query("DELETE FROM tokens_bot").execute(&con).await;
    let token_data: JSONConfig = serde_json::from_str(&data_new_env).unwrap();
    let _result: SqliteQueryResult = sqlx::query(
        "INSERT into tokens_bot (client_id, client_secret, redirect_uri, token, boradcaster_id, bot_id ) VALUES ($1,$2,$3,$4,$5,$6)",
    )
    .bind(token_data.client_id)
    .bind(token_data.client_secret)
    .bind(token_data.redirect_uri)
    .bind(token_data.token)
    .bind(token_data.boradcaster_id)
    .bind(token_data.bot_id)
    .execute(&con)
    .await
    .unwrap();
    Ok(String::from(data_new_env))
}

pub async fn load_config_token() -> Result<String, String> {
    let con: SqlitePool = get_connection().await.unwrap();
    let result: Result<Vec<sqlx::sqlite::SqliteRow>, sqlx::Error> =
        sqlx::query("SELECT * FROM tokens_bot")
            .fetch_all(&con)
            .await;
    if result.is_ok() {
        let result_query: Vec<sqlx::sqlite::SqliteRow> = result.unwrap();
        if result_query.len() > 0 {
            let i = result_query.get(0).unwrap();
            let boradcaster_id: &str = i.get("boradcaster_id");
            std::env::set_var("boradcaster_id", boradcaster_id);
            let bot_id: &str = i.get("bot_id");
            std::env::set_var("bot_id", bot_id);
            let client_id: &str = i.get("client_id");
            std::env::set_var("client_id", client_id);
            let client_secret: &str = i.get("client_secret");
            std::env::set_var("client_secret", client_secret);
            let redirect_uri: &str = i.get("redirect_uri");
            std::env::set_var("redirect_uri", redirect_uri);
            let token: &str = i.get("token");
            std::env::set_var("tokenBot", token);
            std::env::set_var("configLoaded", "S");
            con.close().await;
            Ok(String::from("Configuracion Cargada"))
        } else {
            std::env::set_var("configLoaded", "N");
            con.close().await;
            Err(String::from("No se ha encontrado el comando"))
        }
    } else {
        std::env::set_var("configLoaded", "N");
        con.close().await;
        Err(String::from("No se ha encontrado el comando"))
    }
}

pub async fn save_config() -> Result<String, ()> {
    let data_new_env: structs_custom::JSONConfig = config_struct();
    let data_string: String = serde_json::to_string(&data_new_env)
        .ok()
        .unwrap_or(String::from("{}"));
    let _res = create_config_token(data_string).await;
    Ok(String::from("Config saved"))
}

pub async fn get_command_by_trigger(command_trigger: String) -> Result<CommandStruct, String> {
    let con: SqlitePool = get_connection().await.unwrap();
    let result: Result<Vec<sqlx::sqlite::SqliteRow>, sqlx::Error> =
        sqlx::query("SELECT * FROM commands_twitch WHERE trigger=$1")
            .bind(command_trigger)
            .fetch_all(&con)
            .await;
    if result.is_ok() {
        let result_query: Vec<sqlx::sqlite::SqliteRow> = result.unwrap();
        if result_query.len() > 0 {
            let i = result_query.get(0).unwrap();
            let return_data = CommandStruct {
                command_id: i.get("id"),
                command_name: i.get("command_name"),
                trigger: i.get("trigger"),
                content_type: serde_json::from_str(i.get("content_type")).unwrap(),
                response_text: i.get("response_text"),
                sound: serde_json::from_str(i.get("sound")).unwrap(),
                permits: serde_json::from_str(i.get("permits")).unwrap(),
                cooldown: serde_json::from_str(i.get("cooldown")).unwrap(),
                integration: serde_json::from_str(i.get("integration")).unwrap(),
                point_cost: i.get("point_cost"),
                enabled: i.get("enabled"),
            };
            Ok(return_data)
        } else {
            Err(String::from("No se ha encontrado el comando"))
        }
    } else {
        Err(String::from("No se ha encontrado el comando"))
    }
}
pub async fn get_commands() -> Result<Vec<CommandStruct>, ()> {
    let mut return_data: Vec<CommandStruct> = vec![];
    let con: SqlitePool = get_connection().await.unwrap();
    let result: Result<Vec<sqlx::sqlite::SqliteRow>, sqlx::Error> =
        sqlx::query("SELECT * FROM commands_twitch")
            .fetch_all(&con)
            .await;
    if result.is_ok() {
        let result_query: Vec<sqlx::sqlite::SqliteRow> = result.unwrap();
        for i in result_query {
            return_data.push(structs_custom::CommandStruct {
                command_id: i.get("id"),
                command_name: i.get("command_name"),
                trigger: i.get("trigger"),
                content_type: serde_json::from_str(i.get("content_type")).unwrap(),
                response_text: i.get("response_text"),
                sound: serde_json::from_str(i.get("sound")).unwrap(),
                permits: serde_json::from_str(i.get("permits")).unwrap(),
                cooldown: serde_json::from_str(i.get("cooldown")).unwrap(),
                integration: serde_json::from_str(i.get("integration")).unwrap(),
                point_cost: i.get("point_cost"),
                enabled: i.get("enabled"),
            });
        }
    }
    con.close().await;
    Ok(return_data)
}

pub async fn get_commands_string() -> Result<String, ()> {
    let commnads_object = get_commands().await.unwrap();
    let string_return = serde_json::to_string(&commnads_object).unwrap();
    println!("{string_return}");
    Ok(string_return)
}

pub async fn save_new_command(data: String) -> Result<String, ()> {
    let new_command: structs_custom::CommandStruct = serde_json::from_str(&data.as_str()).unwrap();
    println!("intenta insertar");
    let con: SqlitePool = get_connection().await.unwrap();
    let new_command_content_type: String =
        serde_json::to_string(&new_command.content_type.clone()).unwrap();
    let new_command_sound: String = serde_json::to_string(&new_command.sound.clone()).unwrap();
    let new_command_permits: String = serde_json::to_string(&new_command.permits.clone()).unwrap();
    let new_command_cooldown: String =
        serde_json::to_string(&new_command.cooldown.clone()).unwrap();
    let new_command_integration: String =
        serde_json::to_string(&new_command.integration.clone()).unwrap();
    let _result: SqliteQueryResult = sqlx::query(
        "INSERT into commands_twitch (command_name, trigger, content_type, response_text, sound, permits, cooldown, integration, point_cost) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9)",
    )
    .bind(new_command.command_name)
    .bind(new_command.trigger)
    .bind(new_command_content_type)
    .bind(new_command.response_text)
    .bind(new_command_sound)
    .bind(new_command_permits)
    .bind(new_command_cooldown)
    .bind(new_command_integration)
    .bind(new_command.point_cost)
    .execute(&con)
    .await
    .unwrap();
    Ok(String::from("Ok"))
}

pub async fn edit_command(id: u16, data: String) -> Result<String, ()> {
    let new_command: structs_custom::CommandStruct = serde_json::from_str(&data.as_str()).unwrap();
    println!("intenta insertar");
    let con: SqlitePool = get_connection().await.unwrap();
    let new_command_content_type: String =
        serde_json::to_string(&new_command.content_type.clone()).unwrap();
    let new_command_sound: String = serde_json::to_string(&new_command.sound.clone()).unwrap();
    let new_command_permits: String = serde_json::to_string(&new_command.permits.clone()).unwrap();
    let new_command_cooldown: String =
        serde_json::to_string(&new_command.cooldown.clone()).unwrap();
    let new_command_integration: String =
        serde_json::to_string(&new_command.integration.clone()).unwrap();
    let _result: SqliteQueryResult = sqlx::query(
        "UPDATE commands_twitch SET command_name = $1, trigger= $2, content_type= $3, response_text= $4, sound= $5, permits= $6, cooldown= $7, integration= $8, point_cost = $9 WHERE id = $10",
    )
    .bind(new_command.command_name)
    .bind(new_command.trigger)
    .bind(new_command_content_type)
    .bind(new_command.response_text)
    .bind(new_command_sound)
    .bind(new_command_permits)
    .bind(new_command_cooldown)
    .bind(new_command_integration)
    .bind(new_command.point_cost)
    .bind(id)
    .execute(&con)
    .await
    .unwrap();
    Ok(String::from("Ok"))
}

pub async fn delete_command(id: u16) -> Result<String, ()> {
    let con: SqlitePool = get_connection().await.unwrap();
    let _result: SqliteQueryResult = sqlx::query("DELETE FROM commands_twitch WHERE id = $1")
        .bind(id)
        .execute(&con)
        .await
        .unwrap();

    Ok(String::from("Ok"))
}

pub async fn get_all_points_user() -> Result<Vec<PointUserTwitchStruct>, ()> {
    let con: SqlitePool = get_connection().await.unwrap();
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
    let con: SqlitePool = get_connection().await.unwrap();
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
        let _result: SqliteQueryResult = sqlx::query(
            "UPDATE users_twitch SET name = $2, points = $3, time_watch_mins = $4 WHERE id = $1",
        )
        .bind(u.user_id.clone())
        .bind(u.last_known_name.clone())
        .bind(u.points)
        .bind(u.time_watch_mins)
        .execute(&con)
        .await
        .unwrap();
    }
    con.close().await;
}
