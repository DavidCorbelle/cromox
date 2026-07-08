import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import { useRef } from "react";
import { message_types, suscription_types } from "./consts";

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
  return (
    <main className="container">

      <div className="botoneraNav">
        <button>Chat</button>
        <button>Comandos</button>
        <p>{dataLoaded}</p>
        <p className="debug-data">{greetMsg}</p>
      </div>
      <div>
      <div id="chatBox"></div>
      <input  onKeyDown={(e)=>{ if(e.key==="Enter"){send_message_twitch()}}}  value={messageChatBox} onInput={(e)=>{setMessageChatBox(e.target.value)}} id="chatBoxInpu"></input>
      </div>
    </main>
  );
}

export default App;
