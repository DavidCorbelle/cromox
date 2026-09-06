export type ChatMessage = {
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

export type messageEvent = {
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

export type redeemEvent = {
    broadcaster_user_id: string,
    broadcaster_user_login: string,
    broadcaster_user_name: string,
    id: string,
    redeemed_at: string,
    reward: redeemEventData,
    status: string,
    user_id: string,
    user_input: string,
    user_login: string,
    user_name: string
}
export type redeemEventData = {
    cost: Number
    id: string,
    prompt: string,
    title: string
}


export type Command = {
    command_id: Number,
    command_name: string,
    trigger: string,
    redeem_points_name: string,
    content_type: CommandStructContent,
    response_text: string | null,
    sound: CommandStructSoundData | null,
    permits: CommandStructPermitType,
    integration: CommandStrucIntegrationType | null,
    cooldown: CommandStrucCooldownType | null,
    point_cost: number,
    enabled: boolean
}

export type CommandStructPermitType = {
    content_type: string,
    rol_permit: Array<string> | null,
    user_permit: Array<string> | null
}
export type CommandStrucIntegrationType = {
    http_endpoint: string,
    use_integration: string | null,
    data_integration: null
}
export type CommandStrucCooldownType = {
    units: number,
    type_unit: "SECONDS",
    type_cooldown: COOLDOWN_TYPE;
}

export enum COOLDOWN_TYPE {
    GENERAL = "GENERAL",
    USER = "USER"
}

export enum PERMISSION_TYPE {
    ALL = "ALL",
    ROLE = "ROLE",
    USERNAME = "USERNAME"
}

export enum PERMISION_ROLE_TYPE {
    moderator = "moderator",
    broadcaster = "broadcaster",
    subscriber = "subscriber",
    vip = "vip",
    founder = "founder"

}


export type CommandStructContent = {
    content_type: String,
    position_data: Array<CommandStructContentPositionData> | null,
}
export type CommandStructContentPositionData = {
    position: String,
    param_name: String
}

type CommandStructSoundData = {
    sound_dir: String,
    sound_volume: Number
}

export type CommandUses = {
    userId: String,
    lastTimeUsed: Date,
    commandIdUsed: Number
}



export type PayloadViewers = {
    user_id: String,
    user_login: String,
    user_name: String,
}