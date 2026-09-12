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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetDataVtubestudio {
    pub apiName: String,
    pub apiVersion: String,
    pub requestID: String,
    pub messageType: String,
    pub data: Option<DataVtubestudio>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DataVtubestudio {
    pub modelID: Option<String>,
    pub numberOdModels: Option<u32>,
    pub availableModels: Option<Vec<ModelDataVtubestudio>>,
    pub details:Option<bool>,
    pub expressions:Option<Vec<VtubestudioApiExpression>>,
    pub expresionFile:Option<String>,
    pub active: Option<bool>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetDataVtubestudioActivateExpresion {
    pub apiName: String,
    pub apiVersion: String,
    pub requestID: String,
    pub messageType: String,
    pub data: DataVtubestudioActivateExpresion,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DataVtubestudioActivateExpresion {
    pub expressionFile:String,
    pub active: bool,
    pub fadeTime:f32
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VtubestudioApiExpression {
    pub name: String,
    pub file: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ModelDataVtubestudio {
    pub modelLoaded: bool,
    pub modelName: String,
    pub modelID: String,
    pub vtsModelName: String,
    pub vtsModelIconName: String,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct DataShorcutFront {
    pub id_item:String, 
    pub parent_id:Option<String>,
    pub shortcut:Option<String>
}

#[derive(Copy, Clone, Deserialize)]
pub enum VTUBESTUDIO_ACTIONS {
    NONE = 0,
    GET_MODELS = 1,
    SET_MODEL = 2,
    GET_EXPRESSIONS_MODEL = 3,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct VTUBE_STUDIO_CONFIG {
    pub models_data:Vec<ModelDataFront>
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ModelDataFront {
    pub model_name:String,
    pub model_id:String,
    pub shortcut:String
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ExpressionslDataFront {
    pub expression_file:String,
    pub expression_name:String,
    pub model_id:String,
    pub shortcut:String
}



