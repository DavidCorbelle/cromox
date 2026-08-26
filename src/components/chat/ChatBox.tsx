const ChatBox = ({ send_message_twitch, messageChatBox, setMessageChatBoxIntermedio, viewers }: ChatBoxComponent) => {

    return (<div>
        <div className="gridChat">
            <div id="chatBox"></div>
            <div id="users_chat">
                {viewers != undefined ? viewers.map((e) => { return (<><div>{e.user_name}</div></>) }) : null}
            </div>
        </div>
        <input id="input_message" onKeyDown={(e) => { if (e.key === "Enter") { send_message_twitch() } }} value={messageChatBox} onInput={(e) => { setMessageChatBoxIntermedio(e) }} ></input>
    </div>)
}

export default ChatBox