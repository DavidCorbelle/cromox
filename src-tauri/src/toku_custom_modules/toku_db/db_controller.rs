use crate::{
    consts::{
        get_access_token_url, vtubestudio_set_expression_current_model, vtubestudio_set_model,
        BOT_TOKEN_TYPE, STREAMER_TOKEN_TYPE
    },
    secret_const::{self, CLIENT_ID, CLIENT_SECRET},
    structs_custom::{CommandStruct},
    send_websocket_vtubestudio,
};
use reqwest::{Client, Response};
use sqlx::{
    sqlite::{SqliteConnectOptions, SqliteQueryResult},
    Row, SqlitePool,
};
use std::{
    collections::HashMap,
    str::FromStr,
    time::{Duration, SystemTime},
};
use tauri::{self, AppHandle, Emitter};
use crate::consts::{TABLE_VTUBESTUDIO_EXPRESSIONS, TABLE_VTUBESTUDIO_MODELS};
use crate::toku_custom_modules::toku_twitch::structs_twitch::{TwitchAccessTokenResponse, TwitchPointUserStruct};
use crate::toku_custom_modules::toku_twitch::twitch_websocket::twitch_get_id_user;
use crate::toku_custom_modules::toku_vtubestudio::structs_vtubestudio::{VtubestudioApiExpression, VtubestudioDataShorcutFront, VtubestudioExpressionslDataFront, VtubestudioModelData, VtubestudioModelDataFront, VTUBE_STUDIO_CONFIG};

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
    path
}

pub async fn db_load_config_token() -> Result<String, String> {
    let con: SqlitePool = get_connection().await.unwrap();
    let result: Result<Vec<sqlx::sqlite::SqliteRow>, sqlx::Error> =
        sqlx::query("SELECT * FROM tokens_bot")
            .fetch_all(&con)
            .await;
    if result.is_ok() {
        let result_query: Vec<sqlx::sqlite::SqliteRow> = result.unwrap();
        for i in result_query {
            let type_token: String = i.get("type_token");
            if type_token == STREAMER_TOKEN_TYPE {
                let broadcaster_id: &str = i.get("user_id");
                std::env::set_var("broadcaster_id", broadcaster_id);
                let refresh_token: &str = i.get("refresh_token");
                let token = get_access_token(refresh_token).await.unwrap();
                std::env::set_var("tokenStreamer", token);
            } else if type_token == BOT_TOKEN_TYPE {
                let bot_id: &str = i.get("user_id");
                std::env::set_var("bot_id", bot_id);
                let refresh_token: &str = i.get("refresh_token");
                let token = get_access_token(refresh_token).await.unwrap();
                std::env::set_var("tokenBot", token);
                std::env::set_var("configLoaded", "S");
            }
        }
        con.close().await;
        Ok(String::from("Configuracion Cargada"))
    } else {
        std::env::set_var("configLoaded", "N");
        con.close().await;
        Err(String::from("No se podido cargar la configuracion"))
    }
}

fn command_from_sqlite_row (row: &sqlx::sqlite::SqliteRow) -> CommandStruct
{

    let return_data = CommandStruct {
        command_id: row.get("id"),
        command_name: row.get("command_name"),
        redeem_points_name: row.get("redeem_points_name"),
        trigger: row.get("trigger"),
        content_type: serde_json::from_str(row.get("content_type")).unwrap(),
        response_text: row.get("response_text"),
        sound: serde_json::from_str(row.get("sound")).unwrap(),
        permits: serde_json::from_str(row.get("permits")).unwrap(),
        cooldown: serde_json::from_str(row.get("cooldown")).unwrap(),
        integration: serde_json::from_str(row.get("integration")).unwrap(),
        point_cost: row.get("point_cost"),
        enabled: row.get("enabled"),
    };
    return_data

}

fn bd_sqlite_rows_to_twitch_point_user_struct(data: &Vec<sqlx::sqlite::SqliteRow>) -> Vec<TwitchPointUserStruct>{
    let mut vec_return:Vec<TwitchPointUserStruct> = vec![];
    for row in data{
        vec_return.push( TwitchPointUserStruct{
            points: row.get("points"),
            user_id: row.get("id"),
            time_watch_mins: row.get("time_watch_mins"),
            last_known_name: row.get("name"),
            existe_db: true,
        });
    }

    vec_return
}
pub async fn db_get_command_by_trigger(command_trigger: String) -> Result<CommandStruct, String> {
    let con: SqlitePool = get_connection().await.unwrap();
    let result: Result<Vec<sqlx::sqlite::SqliteRow>, sqlx::Error> =
        sqlx::query("SELECT * FROM commands_twitch WHERE trigger=$1")
            .bind(command_trigger)
            .fetch_all(&con)
            .await;
    if result.is_ok() {
        let result_query: Vec<sqlx::sqlite::SqliteRow> = result.unwrap();
        if result_query.len() > 0 {
            let row = result_query.get(0).unwrap();
            let return_data = command_from_sqlite_row(row);
            Ok(return_data)
        } else {
            Err(String::from("No se ha encontrado el comando"))
        }
    } else {
        Err(String::from("No se ha encontrado el comando"))
    }
}
pub async fn db_get_command_by_redeem_title(command_trigger: String) -> Result<CommandStruct, String> {
    let con: SqlitePool = get_connection().await.unwrap();
    let result: Result<Vec<sqlx::sqlite::SqliteRow>, sqlx::Error> =
        sqlx::query("SELECT * FROM commands_twitch WHERE redeem_points_name=$1")
            .bind(command_trigger)
            .fetch_all(&con)
            .await;
    if result.is_ok() {
        let result_query: Vec<sqlx::sqlite::SqliteRow> = result.unwrap();
        if result_query.len() > 0 {
            let row = result_query.get(0).unwrap();
            let return_data = command_from_sqlite_row(row);
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
    let con: SqlitePool = get_connection().await?;
    let result: Result<Vec<sqlx::sqlite::SqliteRow>, sqlx::Error> =
        sqlx::query("SELECT * FROM commands_twitch")
            .fetch_all(&con)
            .await;
    if result.is_ok() {
        let result_query: Vec<sqlx::sqlite::SqliteRow> = result.unwrap();
        for i in result_query {
            return_data.push(command_from_sqlite_row(&i));
        }
    }
    con.close().await;
    Ok(return_data)
}

pub async fn db_get_commands_string() -> Result<String, ()> {
    let commands_object = get_commands().await?;
    let string_return = serde_json::to_string(&commands_object).unwrap();
    Ok(string_return)
}

pub async fn db_save_new_command(data: String) -> Result<String, ()> {
    let new_command: CommandStruct = serde_json::from_str(&data.as_str()).unwrap();
    let con: SqlitePool = get_connection().await?;
    let new_command_content_type: String =
        serde_json::to_string(&new_command.content_type.clone()).unwrap();
    let new_command_sound: String = serde_json::to_string(&new_command.sound.clone()).unwrap();
    let new_command_permits: String = serde_json::to_string(&new_command.permits.clone()).unwrap();
    let new_command_cooldown: String =
        serde_json::to_string(&new_command.cooldown.clone()).unwrap();
    let new_command_integration: String =
        serde_json::to_string(&new_command.integration.clone()).unwrap();
    let _result: SqliteQueryResult = sqlx::query(
        "INSERT into commands_twitch (command_name, trigger, content_type, response_text, sound, permits, cooldown, integration, point_cost, redeem_points_name) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10)",
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
    .bind(new_command.redeem_points_name)
    .execute(&con)
    .await
    .unwrap();
    Ok(String::from("Ok"))
}

pub async fn db_edit_command(id: u16, data: String) -> Result<String, ()> {
    let new_command: CommandStruct = serde_json::from_str(&data.as_str()).unwrap();
    let con: SqlitePool = get_connection().await?;
    let new_command_content_type: String =
        serde_json::to_string(&new_command.content_type.clone()).unwrap();
    let new_command_sound: String = serde_json::to_string(&new_command.sound.clone()).unwrap();
    let new_command_permits: String = serde_json::to_string(&new_command.permits.clone()).unwrap();
    let new_command_cooldown: String =
        serde_json::to_string(&new_command.cooldown.clone()).unwrap();
    let new_command_integration: String =
        serde_json::to_string(&new_command.integration.clone()).unwrap();
    let _result: SqliteQueryResult = sqlx::query(
        "UPDATE commands_twitch SET command_name = $1, trigger= $2, content_type= $3, response_text= $4, sound= $5, permits= $6, cooldown= $7, integration= $8, point_cost = $9,redeem_points_name = $11  WHERE id = $10",
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
    .bind(new_command.redeem_points_name)
    .execute(&con)
    .await
    .unwrap();
    Ok(String::from("Ok"))
}

pub async fn db_delete_command(id: u16) -> Result<String, ()> {
    let con: SqlitePool = get_connection().await?;
    let _result: SqliteQueryResult = sqlx::query("DELETE FROM commands_twitch WHERE id = $1")
        .bind(id)
        .execute(&con)
        .await
        .unwrap();

    Ok(String::from("Ok"))
}

pub async fn db_get_all_points_user() -> Result<Vec<TwitchPointUserStruct>, ()> {
    let con: SqlitePool = get_connection().await?;
    let mut return_data: Vec<TwitchPointUserStruct> = vec![];

    let result: Result<Vec<sqlx::sqlite::SqliteRow>, sqlx::Error> =
        sqlx::query("SELECT * FROM users_twitch")
            .fetch_all(&con)
            .await;
    if result.is_ok() {
        let result_query: Vec<sqlx::sqlite::SqliteRow> = result.unwrap();
        return_data = bd_sqlite_rows_to_twitch_point_user_struct(&result_query);
    }
    con.close().await;
    Ok(return_data)
}

pub async fn db_get_points_user(user_id: String) -> Result<Vec<TwitchPointUserStruct>, ()> {
    let con: SqlitePool = get_connection().await?;
    let mut return_data: Vec<TwitchPointUserStruct> = vec![];

    let result: Result<Vec<sqlx::sqlite::SqliteRow>, sqlx::Error> =
        sqlx::query("SELECT * FROM users_twitch WHERE id=$1")
            .bind(user_id)
            .fetch_all(&con)
            .await;
    if result.is_ok() {
        let result_query: Vec<sqlx::sqlite::SqliteRow> = result.unwrap();
        return_data = bd_sqlite_rows_to_twitch_point_user_struct(&result_query);
    }
    con.close().await;
    Ok(return_data)
}

pub async fn db_save_points_user(user_id: String, current_points: u32) {
    let con: SqlitePool = get_connection().await.unwrap();
    let _result: SqliteQueryResult =
        sqlx::query("UPDATE users_twitch SET  points = $2 WHERE id = $1")
            .bind(user_id)
            .bind(current_points)
            .execute(&con)
            .await
            .unwrap();
}

pub async fn db_save_all_points_user(points_file_data: Vec<TwitchPointUserStruct>) {
    let con: SqlitePool = get_connection().await.unwrap();
    for i in points_file_data.clone() {
        let _result: SqliteQueryResult = sqlx::query(
            "INSERT INTO users_twitch (id , name, points,time_watch_mins ) VALUES ($1,$2,$3,$4) ON CONFLICT(id) DO UPDATE SET name = excluded.name, points = excluded.points, time_watch_mins = excluded.time_watch_mins",
        )
        .bind(i.user_id.clone())
        .bind(i.last_known_name.clone())
        .bind(i.points)
        .bind(i.time_watch_mins)
        .execute(&con)
        .await
        .unwrap();
    }
    con.close().await;
}

pub async fn db_check_tokens(app: AppHandle) {
    let token_bot = std::env::var("tokenBot").ok().unwrap_or(String::from(""));
    let token_streamer = std::env::var("tokenStreamer")
        .ok()
        .unwrap_or(String::from(""));
    let broadcaster_id = std::env::var("broadcaster_id")
        .ok()
        .unwrap_or(String::from(""));
    let bot_id = std::env::var("bot_id").ok().unwrap_or(String::from(""));
    if broadcaster_id == "" || token_streamer == "" {
        let _res1 = app.emit("token-invalid", STREAMER_TOKEN_TYPE);
    } else {
        let _resu1 = app.emit("token-updated", STREAMER_TOKEN_TYPE);
    }
    if bot_id == "" || token_bot == "" {
        let _res2 = app.emit("token-invalid", BOT_TOKEN_TYPE);
    } else {
        let _resu2 = app.emit("token-updated", BOT_TOKEN_TYPE);
    }
}

pub async fn db_save_token_auth(token: &str, type_token: &str) -> Result<String, String> {
    //GET NEW ACCESS TOKEN
    let url_get_token = secret_const::get_refresh_token_url(token.to_string());
    let client: Client = Client::new();
    let response: Result<Response, reqwest::Error> = client.post(url_get_token).send().await;
    if response.is_ok() {
        let response_data = response.unwrap();
        let response_string = response_data.text().await.unwrap();
        let response_object: TwitchAccessTokenResponse =
            serde_json::from_str(&response_string).unwrap();
        if type_token == BOT_TOKEN_TYPE {
            std::env::set_var("tokenBot", response_object.access_token);
        } else if type_token == STREAMER_TOKEN_TYPE {
            std::env::set_var("tokenStreamer", response_object.access_token);
        }
        let user: String = twitch_get_id_user(type_token)
            .await
            .unwrap();
        let con: SqlitePool = get_connection().await.unwrap();
        let result: Result<Vec<sqlx::sqlite::SqliteRow>, sqlx::Error> =
            sqlx::query("SELECT * FROM tokens_bot WHERE type_token = $1")
                .bind(type_token)
                .fetch_all(&con)
                .await;
        if result.is_ok() {
            let expire_date: SystemTime = SystemTime::now();
            let duration_expire = Duration::from_secs(response_object.expires_in as u64);
            expire_date.checked_add(duration_expire);
            let expire_date_unix_epoch =
                expire_date.duration_since(SystemTime::UNIX_EPOCH).unwrap();
            let expire_date_unix = expire_date_unix_epoch.as_secs().to_string();
            let result_query: Vec<sqlx::sqlite::SqliteRow> = result.unwrap();
            if result_query.len() > 0 {
                let _result_u: SqliteQueryResult = sqlx::query("UPDATE tokens_bot SET refresh_token = $2, expires_in = $3, expire_date = $4, user_id = $5 WHERE type_token = $1")
                    .bind(type_token)
        .bind(response_object.refresh_token)
        .bind(response_object.expires_in)
        .bind(expire_date_unix)
        .bind(user)
        .execute(&con)
        .await
        .unwrap();
            } else {
                let _result_u: SqliteQueryResult = sqlx::query("INSERT INTO tokens_bot (type_token,refresh_token, expires_in, expire_date, user_id) VALUES ($1,$2,$3,$4,$5)")
                    .bind(type_token)
        .bind(response_object.refresh_token)
        .bind(response_object.expires_in)
        .bind(expire_date_unix)
        .bind(user)
        .execute(&con)
        .await
        .unwrap();
            }
        }
    }

    Ok(String::from("Token Updated"))
}

async fn get_access_token(refresh_token: &str) -> Result<String, ()> {
    let url: String = get_access_token_url();
    let client = Client::new();
    let mut params = HashMap::new();
    params.insert("client_id", CLIENT_ID);
    params.insert("client_secret", CLIENT_SECRET);
    params.insert("grant_type", "refresh_token");
    params.insert("refresh_token", refresh_token);
    let response: Result<Response, reqwest::Error> = client.post(url).form(&params).send().await;
    let response_data: Response = response.unwrap();
    let response_string: String = response_data.text().await.unwrap();
    let response_object: TwitchAccessTokenResponse =
        serde_json::from_str(&response_string).unwrap();
    Ok(response_object.access_token)
}

pub async fn db_save_token_integration(integration_name: &str, token: String) {
    let con: SqlitePool = get_connection().await.unwrap();
    let _result: SqliteQueryResult = sqlx::query(
        "INSERT INTO integrations (app, token) VALUES ($1, $2) ON CONFLICT(app) DO UPDATE SET token = excluded.token;",
    )
    .bind(integration_name)
    .bind(token)
    .execute(&con)
    .await
    .unwrap();
}

pub async fn db_get_token_integration(integration_name: &str) -> Result<String, ()> {
    let con: SqlitePool = get_connection().await?;
    let result: Result<Vec<sqlx::sqlite::SqliteRow>, sqlx::Error> =
        sqlx::query("SELECT * FROM  integrations  WHERE app = $1")
            .bind(integration_name)
            .fetch_all(&con)
            .await;
    if result.is_ok() {
        let result_query: Vec<sqlx::sqlite::SqliteRow> = result.unwrap();
        if result_query.len() > 0 {
            let mut token: String = String::from("");
            for l in result_query {
                token = l.get("token");
            }
            Ok(token)
        } else {
            Ok(String::from(""))
        }
    } else {
        Ok(String::from(""))
    }
}

//SHORTCUTS
pub async fn db_save_shorcut_multiple(data: Vec<VtubestudioDataShorcutFront>, table: &str) {
    let con: SqlitePool = get_connection().await.unwrap();
    for d in data {
        let shortcut = d.shortcut.unwrap();
        let mut parent = d.parent_id.unwrap_or_default();
        if parent == String::from("") {
            parent = Default::default();
        }
        if shortcut != String::from("") {
            let select = sqlx::query(
                "SELECT shorcut FROM  shorcuts  WHERE id_item = $1 AND table_name = $2 AND parent_id=$3",
            )
            .bind(d.id_item.clone())
            .bind(table)
            .bind(parent.clone())
            .fetch_all(&con)
            .await
            .unwrap();
            if select.len() > 0 {
                let _result: SqliteQueryResult = sqlx::query(
                    "UPDATE shorcuts SET shorcut = $3 WHERE id_item = $1 AND table_name =$2 AND parent_id=$4",
                )
                .bind(d.id_item)
                .bind(table)
                .bind(shortcut)
                .bind(parent)
                .execute(&con)
                .await
                .unwrap();
            } else {
                let _result: SqliteQueryResult = sqlx::query(
                    "INSERT INTO shorcuts (id_item, table_name, shorcut, parent_id) VALUES ($1, $2, $3, $4)",
                )
                .bind(d.id_item)
                .bind(table)
                .bind(shortcut)
                .bind(parent)
                .execute(&con)
                .await
                .unwrap();
            }
        } else {
            let _result: SqliteQueryResult = sqlx::query(
                "DELETE FROM shorcuts WHERE id_item=$1 AND table_name =$2  AND parent_id=$3",
            )
            .bind(d.id_item)
            .bind(table)
            .bind(parent)
            .execute(&con)
            .await
            .unwrap();
        }
    }
}

pub async fn db_check_shorcut_and_use(string_shorcut: String) {
    let con: SqlitePool = get_connection().await.unwrap();
    let select = sqlx::query("SELECT * FROM  shorcuts  WHERE shorcut = $1")
        .bind(string_shorcut)
        .fetch_all(&con)
        .await
        .unwrap();
    if select.len() > 0 {
        for s in select {
            let shorcut: String = s.get("shorcut");
            let table_name: String = s.get("table_name");
            if table_name == TABLE_VTUBESTUDIO_MODELS {
                let model: String = s.get("id_item");
                let message = vtubestudio_set_model(model);
                let _r = send_websocket_vtubestudio(message).await;
            }
            if table_name == TABLE_VTUBESTUDIO_EXPRESSIONS {
                println!("Entra");
                let expression: String = s.get("id_item");
                let message = vtubestudio_set_expression_current_model(expression);
                println!("Entra: {}", message.clone());
                let _r = send_websocket_vtubestudio(message).await;
            }
            println!("Existe shorcut {}", shorcut);
        }
    }
}

// VTUBE_STUDIO
pub async fn db_get_all_models_vtubestudio() -> Result<Vec<VtubestudioModelDataFront>, ()> {
    let con: SqlitePool = get_connection().await?;
    let mut models = vec![];
    let result: Result<Vec<sqlx::sqlite::SqliteRow>, sqlx::Error> =
        sqlx::query("SELECT VTubeStudio_models.model_id as model_id, VTubeStudio_models.model_name as model_name, shorcuts.shorcut as shorcut FROM VTubeStudio_models LEFT JOIN shorcuts ON VTubeStudio_models.model_id = shorcuts.id_item")
            .fetch_all(&con)
            .await;
    if result.is_ok() {
        let result_query: Vec<sqlx::sqlite::SqliteRow> = result.unwrap();
        for i in result_query {
            models.push(VtubestudioModelDataFront {
                model_name: i.get("model_name"),
                model_id: i.get("model_id"),
                shortcut: i.get("shorcut"),
            });
        }
    }
    Ok(models)
}

pub async fn db_save_model_vtubestudio_api(model: VtubestudioModelData) {
    let con: SqlitePool = get_connection().await.unwrap();
    let _result: SqliteQueryResult = sqlx::query(
        "INSERT INTO VTubeStudio_models (model_id, model_name) VALUES ($1, $2) ON CONFLICT(model_id) DO UPDATE SET model_name = excluded.model_name;",
    )
    .bind(model.modelID)
    .bind(model.modelName)
    .execute(&con)
    .await
    .unwrap();
}

pub async fn db_save_expressions_vtubestudio_api(
    expression: VtubestudioApiExpression,
    model_id: String,
) {
    let con: SqlitePool = get_connection().await.unwrap();
    let select = sqlx::query(
        "SELECT * FROM  VTubeStudio_expressions  WHERE expression_file = $1 AND  model_id = $2",
    )
    .bind(expression.file.clone())
    .bind(model_id.clone())
    .fetch_all(&con)
    .await
    .unwrap();
    if select.len() > 0 {
    } else {
        let _result: SqliteQueryResult = sqlx::query(
        "INSERT INTO VTubeStudio_expressions (expression_name, expression_file, model_id) VALUES ($1, $2, $3);",
    )
    .bind(expression.name)
    .bind(expression.file)
    .bind(model_id)
    .execute(&con)
    .await
    .unwrap();
    }
}

pub async fn db_get_all_expressions_vtubestudio() -> Result<Vec<VtubestudioExpressionslDataFront>, ()> {
    let con: SqlitePool = get_connection().await?;
    let mut expressions = vec![];
    let result: Result<Vec<sqlx::sqlite::SqliteRow>, sqlx::Error> =
        sqlx::query("SELECT VTubeStudio_expressions.model_id as model_id, VTubeStudio_expressions.expression_name as expression_name, VTubeStudio_expressions.expression_file as expression_file, shorcuts.shorcut as shorcut FROM VTubeStudio_expressions LEFT JOIN shorcuts ON VTubeStudio_expressions.model_id = shorcuts.parent_id AND VTubeStudio_expressions.expression_file = shorcuts.id_item")
            .fetch_all(&con)
            .await;
    if result.is_ok() {
        println!("Hay Resultados");
        let result_query: Vec<sqlx::sqlite::SqliteRow> = result.unwrap();
        for i in result_query {
            expressions.push(VtubestudioExpressionslDataFront {
                expression_name: i.get("expression_name"),
                expression_file: i.get("expression_file"),
                model_id: i.get("model_id"),
                shortcut: i.get("shorcut"),
            });
        }
    }
    Ok(expressions)
}

pub async fn db_get_vtubestudio_config() -> Result<String, ()> {
    let models_data = db_get_all_models_vtubestudio().await?;
    let data: VTUBE_STUDIO_CONFIG = VTUBE_STUDIO_CONFIG {
        models_data,
    };
    let data_string = serde_json::to_string(&data).unwrap();
    Ok(data_string)
}
