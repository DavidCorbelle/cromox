
type ChatBoxComponent = {
    messageChatBox:string,
    send_message_twitch:Function,
    setMessageChatBoxIntermedio:Function
    viewers:Array<PayloadViewers>
}

type TokenTwitchConfigComponent = {
    save_token_twitch_config:Function
}

type ComandListComponent ={
    commands:Array<Command>,
    status_sub_menu: number,
    set_status_sub_menu:Function,
    create_comando:Function,
    delete_command:Function,
    edit_command:Function,
    set_current_command:Function,
    current_command:Command|undefined

}