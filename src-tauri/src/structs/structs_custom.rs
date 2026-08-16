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
    pub bot_id: String,
}

#[derive(Serialize)]
pub struct BodyRequestSendMessageTwitch {
    pub broadcaster_id: String,
    pub sender_id: String,
    pub message: String,
}

// Comandos
#[derive(Serialize,Deserialize)]
pub struct BotCommandContainer {
    pub commands: Vec<CommandStruct>,
}


#[derive(Serialize, Deserialize)]
pub struct CommandStruct {
    pub command_id: u16,
    pub command_name: String,
    pub trigger: String,
    pub content_type: CommandStructContent,
    pub response_text: String,
    pub sound_dir:String,
    pub permits:CommandStructPerms,
    pub integration: Option<CommandStructIntegration>,
    pub cooldown: Option<CommandStructCooldown>,
    pub point_cost: Option<u32>,
    pub enabled: bool
}
#[derive(Serialize, Clone, Deserialize)]
pub struct CommandStructContentPositionData {
    pub position: String,
    pub param_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct CommandStructContent {
    pub content_type: String,
    pub position_data: Option<Vec<CommandStructContentPositionData>>
}

#[derive(Serialize, Deserialize)]
pub struct CommandStructPerms {
    pub content_type: String,
    pub rol_permit: Option<Vec<String>>,
    pub user_permit: Option<Vec<String>>
}

#[derive(Serialize, Deserialize)]
pub  struct CommandStructIntegration{
    pub http_endpoint: Option<String>,
    pub use_integration: Option<String>,
    pub data_integration: Option<String>
}

#[derive(Serialize, Deserialize)]
pub  struct CommandStructCooldown{
    pub units: u16,
    pub type_unit: String
}