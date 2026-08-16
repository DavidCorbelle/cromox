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


type CommandBot = {
    name: String,
    trigger: String,
    content_type: CommandStructContentFull | CommandStructContentPosition,
    response_text: String,
    sound_dir: String
}

type CommandStructContentFull = {
    content_type: String
}
type CommandStructContentPosition = {
    content_type: String,
    position_data: Array<CommandStructContentPositionData>,
}
type CommandStructContentPositionData = {
    position: String,
    param_name: String
}