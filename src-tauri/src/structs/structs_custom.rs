use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct ConditionStruct {
    pub broadcaster_user_id: String,
    pub user_id: String,
}
#[derive(Serialize)]
pub struct TransportStruct {
    pub session_id: String,
    pub method: String,
}

#[derive(Serialize)]
pub struct BodyRequestSuscriber {
    pub type_string: String,
    pub version: String,
    pub condition: ConditionStruct,
    pub transport: TransportStruct,
}

#[derive(Serialize, Deserialize)]
pub struct JSONConfig {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
    pub token: String,
    pub boradcaster_id: String,
    pub bot_id: String
}


#[derive(Serialize)]
pub struct BodyRequestSendMessageTwitch {
    pub broadcaster_id: String,
    pub sender_id: String,
    pub message: String,
}
#[derive(Serialize)]
pub struct BotCommandContainer{
    pub commands:Box::<BotCommand>
}

#[derive(Serialize)]
pub struct BotCommand{
    comand_name:String

}