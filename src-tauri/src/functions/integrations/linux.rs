use std::{fs};

use evdev::{AttributeSetRef, Device, EventSummary, KeyCode};
use tauri::{AppHandle, Emitter};

use crate::db_controller;

pub async fn start_check_hotkeys(app: AppHandle) {
    let paths = fs::read_dir("/dev/input/").unwrap();
    for path in paths {
        let path_check = path.unwrap();
        let path_dir = path_check.file_name().display().to_string();
        if !path_check.file_type().unwrap().is_dir() {
            tokio::spawn(key_listener(path_dir, app.clone()));
        }
    }
}

async fn key_listener(path: String, app: AppHandle) {
    let device_test = Device::open(format!("/dev/input/{}", path));
    if device_test.is_ok() {
        let mut device = device_test.unwrap();
        let supported_keys: &AttributeSetRef<KeyCode> = device.supported_keys().unwrap();
        let keys: Vec<KeyCode> = supported_keys.iter().collect();
        let is_keyboard: Vec<&KeyCode> = keys
            .iter()
            .filter(|x| x.code() == KeyCode::KEY_A.code())
            .collect();
        if is_keyboard.len() > 0 {
            let mut keys_pressed: Vec<KeyCode> = vec![];
            loop {
                for event in device.fetch_events().unwrap() {
                    match event.destructure() {
                        EventSummary::Key(_ev, key_type, 1) => {
                            keys_pressed.push(key_type);
                            let mut keys_pressed_vect_string: Vec<String> = vec![];
                            for k in keys_pressed.clone() {
                                let key = format!("{:?}", k);
                                let key_string: String = key.replace("KEY_", "");
                                keys_pressed_vect_string.push(key_string);
                            }
                            keys_pressed_vect_string.sort();
                            keys_pressed_vect_string.sort_by(|a,b| b.len().cmp(&a.len()));
                            let keys_pressed_string = keys_pressed_vect_string.join(" + ");
                            let _r = db_controller::check_shorcut_and_use(keys_pressed_string).await;
                        }
                        EventSummary::Key(_ev, key_type, 0) => {
                            let emit =
                                std::env::var("ListenForNewShorcut").unwrap_or(String::from("N"));
                            if emit == String::from("S") {
                                let mut keys_resp_vect: Vec<String> = vec![];
                                for k in keys_pressed.clone() {
                                    let key = format!("{:?}", k);
                                    let key_string: String = key.replace("KEY_", "");
                                    keys_resp_vect.push(key_string);
                                }
                                keys_resp_vect.sort();
                                keys_resp_vect.sort_by(|a,b| b.len().cmp(&a.len()));
                                let resp = keys_resp_vect.join(" + ");
                                let _r = app.emit("hotkey-pressed", resp);
                                std::env::set_var("ListenForNewShorcut", "N");
                            }
                            
                            let index: Option<usize> = keys_pressed
                                .iter()
                                .position(|r: &KeyCode| r.code() == key_type.code());
                            if index.is_some() {
                                keys_pressed.remove(index.unwrap());
                            }
                            let mut keys_pressed_vect_string: Vec<String> = vec![];
                            for k in keys_pressed.clone() {
                                let key = format!("{:?}", k);
                                let key_string: String = key.replace("KEY_", "");
                                keys_pressed_vect_string.push(key_string);
                            }
                        }
                        EventSummary::AbsoluteAxis(_, _, _) => {

                        }
                        _ => print!(""),
                    }
                }
            }
        }
    }
}
