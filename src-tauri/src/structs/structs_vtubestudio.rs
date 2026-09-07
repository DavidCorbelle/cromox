#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct AuthResponseVtubestudio {
    pub apiName: String,
    apiVersion: String,
    timestamp: u128,
    messageType: String,
    requestID: String,
    pub data: AuthResponseVtubestudioData,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AuthResponseVtubestudioData {
    pub authenticationToken: String,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct AuthSendVtubestudio {
    pub apiName: String,
    pub apiVersion: String,
    pub requestID: String,
    pub messageType: String,
    pub data: AuthSendVtubestudioData,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct AuthSendVtubestudioData {
    pub pluginName: String,
    pub pluginDeveloper: String,
    pub authenticationToken: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct GetDataVtubestudio {
    pub apiName: String,
    pub apiVersion: String,
    pub requestID: String,
    pub messageType: String,
    pub data: Option<DataVtubestudio>,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct DataVtubestudio {
    pub modelID: Option<String>,
    pub numberOdModels: Option<u32>,
    pub availableModels: Option<Vec<ModelDataVtubestudio>>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ModelDataVtubestudio {
    pub modelLoaded: bool,
    pub modelName: String,
    pub modelID: String,
    pub vtsModelName: String,
    pub vtsModelIconName: String,
}

#[derive(Copy, Clone, Deserialize)]
pub enum VTUBESTUDIO_ACTIONS {
    NONE = 0,
    GET_MODELS = 1,
    SET_MODEL = 2,
}
