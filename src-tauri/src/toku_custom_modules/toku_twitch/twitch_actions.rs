use std::time::Duration;
use tauri::{AppHandle, Emitter};
use tokio::time::sleep;
use crate::toku_custom_modules::toku_db::db_controller::{db_get_all_points_user, db_save_all_points_user};
use crate::toku_custom_modules::toku_twitch::structs_twitch::{TwitchChatterList, TwitchPointUserStruct, TwitchResponseChatters};
use crate::toku_custom_modules::toku_twitch::twitch_websocket::twitch_get_chatters;

pub async fn twitch_points(app: AppHandle) {
    std::env::set_var("points_started", String::from("S"));
    loop {
        let points_file: Vec<TwitchPointUserStruct> =
            db_get_all_points_user().await.unwrap();
        let mut points_update: Vec<TwitchPointUserStruct> = vec![];
        let chatters: Vec<TwitchChatterList> = get_chatters_list().await;
        let old_chatters_string: String =
            std::env::var("old_chatters").unwrap_or(String::from("None"));
        if old_chatters_string != "None" {
            let old_chatters: Vec<TwitchChatterList> =
                serde_json::from_str(&old_chatters_string).ok().unwrap();
            for n in old_chatters {
                let index: Option<usize> = chatters
                    .iter()
                    .position(|r: &TwitchChatterList| r.user_id == n.user_id);
                if index.is_some() {
                    let index_file: Option<usize> =
                        points_file
                            .iter()
                            .position(|r: &TwitchPointUserStruct| {
                                r.user_id == n.user_id
                            });
                    if index_file.is_some() {
                        let mut point_name: TwitchPointUserStruct =
                            points_file[index_file.unwrap()].clone();
                        point_name.points += 1;
                        point_name.last_known_name = n.user_name;
                        point_name.time_watch_mins += 5;
                        points_update.push(point_name);
                    } else {
                        points_update.push({
                            TwitchPointUserStruct {
                                points: 0,
                                user_id: n.user_id,
                                time_watch_mins: 0,
                                last_known_name: n.user_name,
                                existe_db: false,
                            }
                        });
                    }
                }
            }
        }
        db_save_all_points_user(points_update).await;
        let chatters_string = serde_json::to_string(&chatters).ok().unwrap();
        let _emit = app.emit("refresh-viewers", chatters_string.clone());
        std::env::set_var("old_chatters", chatters_string);

        sleep(Duration::from_mins(5)).await;
        println!("Vuelta");
    }
}


async fn get_chatters_list() -> Vec<TwitchChatterList> {
    let mut list_chatters: Vec<TwitchChatterList> = Vec::new();
    let list_chatters_result: String = twitch_get_chatters()
        .await
        .text()
        .await
        .unwrap();
    let list_chatters_json: TwitchResponseChatters =
        serde_json::from_str(&list_chatters_result).unwrap();
    for c in list_chatters_json.data {
        list_chatters.push(TwitchChatterList {
            user_id: c.user_id,
            user_login: c.user_login,
            user_name: c.user_name,
        });
    }
    list_chatters
}