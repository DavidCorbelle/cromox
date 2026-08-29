import { Command, PayloadViewers } from "./types.td";

export type ChatBoxComponent = {
    messageChatBox:string,
    send_message_twitch:Function,
    setMessageChatBoxIntermedio:Function
    viewers:Array<PayloadViewers>
}

export type TokenTwitchConfigComponent = {
      get_new_token_bot:Function,
      get_new_token_streamer:Function,
      streamer_token_loaded:boolean,
      bot_token_loaded:boolean,

}

export type ComandListComponent ={
    commands:Array<Command>,
    create_comando:Function,
    delete_command:Function,
    edit_command:Function,
}