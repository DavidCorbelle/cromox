use crate::structs_vtubestudio::*;
pub fn get_access_token_url() -> String {
    return String::from("https://id.twitch.tv/oauth2/token");
}

pub const SCOPES_BOT: &'static [&'static str] = &[
    "moderator:read:chatters",
    "channel:bot",
    "user:write:chat",
    "user:read:chat",
];
pub const SUSCRIBERS_TWITCH: &'static [&'static str] = &[
    "channel.chat.message",
    "channel.channel_points_custom_reward_redemption.add",
];
pub const SCOPES_STREAMER: &'static [&'static str] =
    &["user:bot", "channel:read:redemptions", "user:read:chat"];

// VTUBESTUDIO
const VTUBESTUDIO_PLUGIN_NAME: &'static str = "Cromox";
const VTUBESTUDIO_DEVELOPER: &'static str = "Toku_Doku";
const VTUBESTUDIO_API_NAME: &'static str = "VTubeStudioPublicAPI";
const VTUBESTUDIO_API_VERSION: &'static str = "1.0";
pub const STREAMER_TOKEN_TYPE: &str = "STREAMER";
pub const BOT_TOKEN_TYPE: &str = "BOT";
pub const TABLE_VTUVESTUDIO_MODELS: &'static str = "VTubeStudio_models";
pub const TABLE_VTUVESTUDIO_EXPRESSIONS: &'static str = "VTubeStudio_expressions";

pub fn vtubestudio_get_auth_string(token: String) -> String {
    let message_aut: AuthSendVtubestudio = AuthSendVtubestudio {
        apiName: String::from(VTUBESTUDIO_API_NAME),
        apiVersion: String::from(VTUBESTUDIO_API_VERSION),
        requestID: String::from("AuthRequest"),
        messageType: String::from("AuthenticationRequest"),
        data: AuthSendVtubestudioData {
            authenticationToken: token,
            pluginDeveloper: String::from(VTUBESTUDIO_DEVELOPER),
            pluginName: String::from(VTUBESTUDIO_PLUGIN_NAME),
        },
    };
    let string_message = serde_json::to_string(&message_aut).unwrap();
    return string_message;
}

pub fn vtubestudio_get_models() -> String {
    let message_aut: GetDataVtubestudio = GetDataVtubestudio {
        apiName: String::from(VTUBESTUDIO_API_NAME),
        apiVersion: String::from(VTUBESTUDIO_API_VERSION),
        requestID: String::from("GetDataModels"),
        messageType: String::from("AvailableModelsRequest"),
        data: None,
    };
    let string_message = serde_json::to_string(&message_aut).unwrap();
    return string_message;
}

pub fn vtubestudio_set_model(param: String) -> String {
    let message_aut: GetDataVtubestudio = GetDataVtubestudio {
        apiName: String::from(VTUBESTUDIO_API_NAME),
        apiVersion: String::from(VTUBESTUDIO_API_VERSION),
        requestID: String::from("SetModel"),
        messageType: String::from("ModelLoadRequest"),
        data: Some(DataVtubestudio {
            modelID: Some(param),
            availableModels: None,
            numberOdModels: None,
            details: None,
            expressions:None,
            expresionFile:None,
            active:None
        }),
    };
    let string_message = serde_json::to_string(&message_aut).unwrap();
    return string_message;
}

pub fn vtubestudio_get_expressions_current_model() -> String {
    let message_aut: GetDataVtubestudio = GetDataVtubestudio {
        apiName: String::from(VTUBESTUDIO_API_NAME),
        apiVersion: String::from(VTUBESTUDIO_API_VERSION),
        requestID: String::from("GetCurrentExpressions"),
        messageType: String::from("ExpressionStateRequest"),
        data: Some(DataVtubestudio {
            modelID: None,
            availableModels: None,
            numberOdModels: None,
            details: Some(true),
            expressions:None,
            expresionFile:None,
            active:None
        }),
    };
    let string_message = serde_json::to_string(&message_aut).unwrap();
    return string_message;
}

pub fn vtubestudio_set_expression_current_model(params:String) -> String {
    let message_aut: GetDataVtubestudioActivateExpresion = GetDataVtubestudioActivateExpresion {
        apiName: String::from(VTUBESTUDIO_API_NAME),
        apiVersion: String::from(VTUBESTUDIO_API_VERSION),
        requestID: String::from("ActivateExpression"),
        messageType: String::from("ExpressionActivationRequest"),
        data: DataVtubestudioActivateExpresion { 
            expressionFile:params,
            fadeTime: 0.5,
            active:true
        },
    };
    let string_message = serde_json::to_string(&message_aut).unwrap();
    return string_message;
}
