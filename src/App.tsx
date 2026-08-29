import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import { useRef } from "react";
import { MENU_ACTUAL, message_types, suscription_types, TIPO_COMANDO_TEXTO, TYPE_TOKEN } from "./consts";
import ChatBox from "./components/chat/ChatBox";
import TokenTwitchConfig from "./components/configMenu/TokenTwitchConfig";
import CommandList from "./components/configMenu/CommandList";
import { canIUseCommand } from "./functions/function_commands";
import { listen } from '@tauri-apps/api/event';
import { openUrl } from '@tauri-apps/plugin-opener';
import { ChatMessage, Command, CommandUses, COOLDOWN_TYPE, messageEvent, PayloadViewers } from "./custom-types/types.td";

let didInit = false;
let socketStarted = false;

const initializeSocket = () => {
  try {
    return new WebSocket("wss://eventsub.wss.twitch.tv/ws")
  } catch {
    throw new Error("No se ha podido inicializar el socket");
  }

}

function App() {
  const [socket] = useState(initializeSocket)
  const [greetMsg] = useState("");
  const [messageChatBox, setMessageChatBox] = useState("");
  const [broadcasterTokenLoaded, setBroadcasterTokenLoaded] = useState(false);
  const [botTokenLoaded, setBotTokenLoaded] = useState(false);
  const [suscribersStarted, setSuscribersStarted] = useState(false);
  const [dataLoaded, setDataLoaded] = useState<any>(undefined);
  const [currentMenu, setCurrentMenu] = useState(MENU_ACTUAL.CHAT);
  const [messages, setMessages] = useState<Array<ChatMessage>>([]);
  const commands = useRef<Array<Command>>([])
  const [forceUpdate, setForceUpdate] = useState<boolean>(true);
  const commandUses = useRef<Array<CommandUses>>([]);
  const sessionIDtmp = useRef(undefined)
  const botIdChat = useRef("")
  const viewers = useRef<Array<PayloadViewers>>([])


  listen<string>('refresh-viewers', (event) => {
    let viewers_parse = JSON.parse(event.payload);
    viewers.current = viewers_parse;
    setForceUpdate(!forceUpdate);

  });
  listen<string>('token-invalid', (event) => {
    if (event.payload == TYPE_TOKEN.BOT) {
      setBotTokenLoaded(false);
    } else if (event.payload == TYPE_TOKEN.STREAMER) {
      setBroadcasterTokenLoaded(false);
    }
  });
  listen<string>('token-updated', (event) => {
    if (event.payload == TYPE_TOKEN.BOT) {
      setBotTokenLoaded(true);
    } else if (event.payload == TYPE_TOKEN.STREAMER) {
      setBroadcasterTokenLoaded(true);
    }
  });

  useEffect(() => {
    if (didInit == false) {
      didInit = true
      getDataLoaded();
    }

  }, []);

  useEffect(() => {
    if (botTokenLoaded == true && broadcasterTokenLoaded == true && suscribersStarted == false && sessionIDtmp.current != undefined) {
      tryStartSuscribers();

    }
  }, [botTokenLoaded, broadcasterTokenLoaded, sessionIDtmp]);

  async function tryStartSuscribers() {
    if (suscribersStarted == false) {
      botIdChat.current = await invoke("get_bot_id");
      let sessionId = sessionIDtmp.current;
      let tryStart: string = await invoke("implement_suscribers", { sessionId });
      if (!tryStart.includes("Error")) {
        setSuscribersStarted(true);
      }
    }

  }
  useEffect(() => {
    if (socket != undefined && socketStarted == false) {
      socketStarted = true;
      // Connection opened
      socket.addEventListener("open", _event => {
      });
      // Listen for messages
      socket.addEventListener("message", (event) => process_message(event));
    }

  }, [socket])

  function process_message(event: MessageEvent<any>) {
    if (sessionIDtmp.current == undefined) {
      let session_id = JSON.parse(event.data).payload.session.id;
      sessionIDtmp.current = session_id;

    } else {
      let data = JSON.parse(event.data);
      if (data.metadata.message_type == message_types.NOTIFICATION) {
        if (data.metadata.subscription_type == suscription_types.CHAT_MESSAGE) {
          let event: messageEvent = data.payload.event;
          if (event.message.text.startsWith("!") && event.chatter_user_id != botIdChat.current) {
            try_command(event);
          } else {
            add_message(event);
          }

        }
      }

      //setGreetMsg("Message from server " + event.data)

    }

  }
  async function update_commands() {
    let dataCommandsString = await invoke("get_data_commands")
    if (dataCommandsString != "") {
      let dataCommands = JSON.parse(dataCommandsString as string);
      let dataUpdate = dataCommands;
      commands.current = dataUpdate;
      setForceUpdate(!forceUpdate);
    }

  }
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

  function try_command(i: messageEvent) {
    let messageTextCommand = i.message.text;
    let command_string: string = messageTextCommand.split(" ")[0].replace("!", "");
    let find = commands.current.filter((e) => { return (e.trigger == command_string) })
    if (find.length > 0) {
      let command = find[0];
      let usable = canIUseCommand(command.command_id, i.chatter_user_id, command.cooldown, commandUses.current);
      if (usable && command.enabled == true) {
        commandUses.current.push({ commandIdUsed: command.command_id, lastTimeUsed: new Date(), userId: i.chatter_user_id });
        invoke('execute_command', { messageTextCommand });

      }

    }
    add_message(i);
  }


  async function getDataLoaded() {
    setDataLoaded(await invoke("start_data_config"));
    update_commands();
  }

  async function send_message_twitch() {
    let message = messageChatBox;
    await invoke("send_message_twitch", { message });
    setMessageChatBox("");
  }

  function setMessageChatBoxIntermedio(e: any) {
    setMessageChatBox(e.target.value)
  }


  function save_token_twitch_config(tokenType: string) {
    invoke("get_url_token", { tokenType }).then((res) => { openUrl(res as string) });
  }


  function prepare_data_command(data: FormData): Command {
    let commands_data = commands.current;
    let command_id = (commands != undefined && commands_data.length > 0 ? commands_data[commands_data.length - 1].command_id as number + 1 : 1);
    let new_command: Command = {
      command_id: command_id,
      command_name: data.get("command_name") as string,
      trigger: data.get("trigger") as string,
      content_type: {
        content_type: TIPO_COMANDO_TEXTO.FULL_TEXT,
        position_data: null
      },
      response_text: data.get("response_text") as string,
      sound: {
        sound_dir: data.get("sound_dir") as string,
        sound_volume: isNaN(Number.parseInt(data.get("sound_volume") as string)) ? 100 : Number.parseInt(data.get("sound_volume") as string) * 100,
      },
      permits: {
        content_type: "AllAccess",
        rol_permit: null,
        user_permit: null
      },
      integration: null,
      cooldown: {
        units: isNaN(Number.parseInt(data.get("cooldown") as string)) ? 0 : (Number.parseInt(data.get("cooldown") as string)),
        type_unit: "SECONDS",
        type_cooldown: data.get("type_cooldown") as COOLDOWN_TYPE
      },
      point_cost: Number.parseInt(data.get("point_cost") as string),
      enabled: data.get("enabled") != null
    };
    return new_command;


  }
  function create_comando(e: React.SubmitEvent<HTMLFormElement>) {
    e.preventDefault();
    var formData = new FormData(e.target);
    let new_command = prepare_data_command(formData);
    let tmpCommands: Array<Command>
    if (commands != undefined) {
      tmpCommands = commands.current;
      tmpCommands.push(new_command);
    } else {
      tmpCommands = [new_command];
    }
    let commandData = JSON.stringify(new_command);
    invoke('save_new_command', { commandData }).then(() => {
      update_commands();
    });
    commands.current = tmpCommands;
  }

  function delete_command(commandId: Number) {
    invoke('delete_command', { commandId }).then(() => { update_commands() })

  }

  function edit_command(commandId: Number, e: React.SubmitEvent<HTMLFormElement>) {
    e.preventDefault();

    var formData = new FormData(e.target);
    let new_command = prepare_data_command(formData);
    new_command.command_id = commandId;
    let tmpCommands: Array<Command>
    if (commands != undefined) {
      tmpCommands = commands.current;
      let tmp = tmpCommands.filter((e) => { return (e.command_id == commandId) });
      if (tmp.length > 0) {
        let index = tmpCommands.indexOf(tmp[0]);
        tmpCommands[index] = new_command;
      }
    } else {
      tmpCommands = [new_command];
    }
    let commandData = JSON.stringify(new_command);
    commands.current = tmpCommands;
    invoke('edit_command', { commandId, commandData }).then(() => { update_commands() })
  }

  function renderCurrentView() {
    switch (currentMenu) {
      case MENU_ACTUAL.TOKENS:
        return (<TokenTwitchConfig
          get_new_token_bot={save_token_twitch_config}
          get_new_token_streamer={save_token_twitch_config}
          bot_token_loaded={botTokenLoaded}
          streamer_token_loaded={broadcasterTokenLoaded}
        ></TokenTwitchConfig>)
      case MENU_ACTUAL.COMANDOS:
        return (<CommandList
          commands={commands.current}
          create_comando={create_comando}
          delete_command={delete_command}
          edit_command={edit_command}
        ></CommandList>)

      default:
        break;
    }
  }

  return (
    <main className="container">

      <div className="botoneraNav">
        <button onClick={() => setCurrentMenu(MENU_ACTUAL.CHAT)}>Chat</button>
        <button onClick={() => setCurrentMenu(MENU_ACTUAL.COMANDOS)}>Comandos</button>
        <button onClick={() => setCurrentMenu(MENU_ACTUAL.TOKENS)}>TOKENS</button>
        <p>{dataLoaded}</p>
        <p className="debug-data">{greetMsg}</p>
      </div>
      <div hidden={currentMenu != MENU_ACTUAL.CHAT}>
        <ChatBox
          messageChatBox={messageChatBox}
          viewers={viewers.current}
          send_message_twitch={send_message_twitch}
          setMessageChatBoxIntermedio={setMessageChatBoxIntermedio}
        >

        </ChatBox>
      </div>
      <div>
        {renderCurrentView()}
      </div>


    </main>
  );
}

export default App;
