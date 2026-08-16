import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import { useRef } from "react";
import { MENU_ACTUAL, message_types, suscription_types } from "./consts";
import ChatBox from "./components/chat/ChatBox";
import TokenTwitchConfig from "./components/configMenu/TokenTwitchConfig";

const sleep = (ms: number) => new Promise((r) => setTimeout(r, ms));
let intentos = 0;
const initializeSocket = () => {
  try {
    return new WebSocket("wss://eventsub.wss.twitch.tv/ws")
  } catch {
    if (intentos < 10) {
      console.log("reintento");
      sleep(2000);
      intentos++;
      return initializeSocket()
    } else {
      throw new Error("No se ha podido inicializar el socket");
    }
  }

}

function App() {
  const [socket] = useState(initializeSocket)
  const [greetMsg, setGreetMsg] = useState("");
  const [messageChatBox, setMessageChatBox] = useState("");
  const [sessionID, setSessionID] = useState(undefined);
  const [dataLoaded, setDataLoaded] = useState(undefined);
  const [currentMenu, setCurrentMenu] = useState(MENU_ACTUAL.CHAT);
  const [messages, setMessages] = useState<Array<ChatMessage>>([]);
  const sessionIDtmp = useRef(undefined)



  useEffect(() => {
    getDataLoaded();
  }, []);

  useEffect(() => {
    if (dataLoaded != undefined) {
      // Connection opened
      socket.addEventListener("open", _event => {
      });
      // Listen for messages
      socket.addEventListener("message", event => {
        if (sessionIDtmp.current == undefined) {
          let session_id = JSON.parse(event.data).payload.session.id;
          sessionIDtmp.current = session_id;
          setSessionID(session_id);
        } else {
          let data = JSON.parse(event.data);
          if (data.metadata.message_type == message_types.NOTIFICATION) {
            if (data.metadata.subscription_type == suscription_types.CHAT_MESSAGE) {
              //check_listener(data.payload.event)
              add_message(data.payload.event);
            }
          }

          setGreetMsg("Message from server " + event.data)

        }

      });
    }

  }, [dataLoaded])
  useEffect(() => {
    if (sessionID != undefined) {
      implement_suscribers(sessionID)
    }

  }, [sessionID]);

  function add_message(i: messageEvent) {
    let newMessage: ChatMessage = {
      chatter_name: i.chatter_user_name,
      message: i.message.text
    }
    let arrayMessages = messages;
    arrayMessages.push(newMessage);
    setMessages(arrayMessages);
    let chatBox = document.getElementById("chatBox");
    const newNode = document.createElement("div");
    newNode.textContent = newMessage.chatter_name + ": " + newMessage.message
    chatBox?.appendChild(newNode)
  }
  async function implement_suscribers(sessionId: string) {
    console.log(sessionID);
    setGreetMsg(await invoke("implement_suscribers", { sessionId }));
  }
  async function getDataLoaded() {
    setDataLoaded(await invoke("start_data_config"));
  }

  async function send_message_twitch() {
    let message = messageChatBox;
    await invoke("send_message_twitch", { message });
    setMessageChatBox("");
  }

  function setMessageChatBoxIntermedio(e: any) {
    setMessageChatBox(e.target.value)
  }

  function save_token_twitch_config() {
    var client_id_container = document.getElementById("client_id") as unknown;
    var client_id = client_id_container as HTMLInputElement;
    var client_secret_container = document.getElementById("client_secret") as unknown;
    var client_secret = client_secret_container as HTMLInputElement;
    var redirect_uri_container = document.getElementById("redirect_uri") as unknown;
    var redirect_uri = redirect_uri_container as HTMLInputElement;
    var token_container = document.getElementById("token") as unknown;
    var token = token_container as HTMLInputElement;
    var boradcaster_id_container = document.getElementById("boradcaster_id") as unknown;
    var boradcaster_id = boradcaster_id_container as HTMLInputElement;
    var bot_id_container = document.getElementById("bot_id") as unknown;
    var bot_id = bot_id_container as HTMLInputElement;

    var dataJSON = {
      client_id: client_id.value,
      client_secret: client_secret.value,
      redirect_uri: redirect_uri.value,
      token: token.value,
      boradcaster_id: boradcaster_id.value,
      bot_id: bot_id.value
    };
    var newConfigToken = JSON.stringify(dataJSON);

    invoke("save_new_data_token", { newConfigToken }).then(() => getDataLoaded());
  }

  function renderCurrentView() {
    switch (currentMenu) {
      case MENU_ACTUAL.CONFIGURACION:
        return (<TokenTwitchConfig
          save_token_twitch_config={save_token_twitch_config}
        ></TokenTwitchConfig>)
      default:
        break;
    }
  }

  return (
    <main className="container">

      <div className="botoneraNav">
        <button onClick={() => setCurrentMenu(MENU_ACTUAL.CHAT)}>Chat</button>
        <button onClick={() => setCurrentMenu(MENU_ACTUAL.COMANDOS)}>Comandos</button>
        <button onClick={() => setCurrentMenu(MENU_ACTUAL.CONFIGURACION)}>CONFIGURACION</button>
        <p>{dataLoaded}</p>
        <p className="debug-data">{greetMsg}</p>
      </div>
      <div hidden={currentMenu != MENU_ACTUAL.CHAT}>
        <ChatBox
          messageChatBox={messageChatBox}
          send_message_twitch={send_message_twitch}
          setMessageChatBoxIntermedio={setMessageChatBoxIntermedio}
        >

        </ChatBox>
      </div>
      {renderCurrentView()}

    </main>
  );
}

export default App;
