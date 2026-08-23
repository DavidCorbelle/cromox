type ChatMessage = {
    message: string,
    chatter_name: string
}


type messageEventData = {
    "text": string,
    "fragments": [
        {
            "type": string,
            "text": string,
            "cheermote": any,
            "emote": any,
            "mention": any
        }
    ]
}

type messageEventBadges = {
    "set_id": string,
    "id": string,
    "info": string

}

type messageEvent = {
    "broadcaster_user_id": string,
    "broadcaster_user_login": string,
    "broadcaster_user_name": string,
    "source_broadcaster_user_id": any,
    "source_broadcaster_user_login": any,
    "source_broadcaster_user_name": any,
    "chatter_user_id": string,
    "chatter_user_login": string,
    "chatter_user_name": string,
    "message_id": string,
    "source_message_id": any,
    "is_source_only": any,
    "message": messageEventData,
    "color": string,
    "badges": Array<messageEventBadges>,
    "source_badges": any,
    "message_type": string,
    "cheer": any,
    "reply": any,
    "channel_points_custom_reward_id": any,
    "channel_points_animation_id": any
}

type Command  = {
     command_id: Number,
      command_name: string,
      trigger: string,
      content_type: CommandStructContent,
      response_text: string|null,
      sound: CommandStructSoundData|null,
      permits: CommandStructPermitType,
      integration: CommandStrucIntegrationType|null,
      cooldown: CommandStrucCooldownType|null,
      point_cost: number,
      enabled: boolean
}

type CommandStructPermitType = {
        content_type: string,
        rol_permit: Array<string>|null,
        user_permit: Array<string>|null
}
type CommandStrucIntegrationType = {
        http_endpoint: string,
        use_integration: string|null,
        data_integration: null
}
type CommandStrucCooldownType = {
       units: number,
       type_unit: "SECONDS"
}

type CommandStructContent= {
    content_type: String,
    position_data: Array<CommandStructContentPositionData> | null,
}
type CommandStructContentPositionData = {
    position: String,
    param_name: String
}

type CommandStructSoundData = {
    sound_dir: String,
    sound_volume: Number
}

type CommandUses = {
    userId: String,
    lastTimeUsed: Date,
    commandIdUsed: Number
}