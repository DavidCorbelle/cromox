const ChatBox = ({ send_message_twitch, messageChatBox, setMessageChatBoxIntermedio }: ChatBoxComponent) => {

    return (<div>
        <div id="chatBox"></div>
        <input onKeyDown={(e) => { if (e.key === "Enter") { send_message_twitch() } }} value={messageChatBox} onInput={(e) => { setMessageChatBoxIntermedio(e) }} id="chatBoxInpu"></input>
    </div>)
}

export default ChatBox