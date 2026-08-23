import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import { useRef } from "react";
import { MENU_ACTUAL, MENU_COMANDOS, message_types, suscription_types, TIPO_COMANDO_TEXTO } from "./consts";
import ChatBox from "./components/chat/ChatBox";
import TokenTwitchConfig from "./components/configMenu/TokenTwitchConfig";
import CommandList from "./components/configMenu/CommandList";
import { canIUseCommand } from "./functions/function_commands";

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
  const [socket, setSocket] = useState(initializeSocket)
  const [greetMsg, setGreetMsg] = useState("");
  const [messageChatBox, setMessageChatBox] = useState("");
  const [sessionID, setSessionID] = useState(undefined);
  const [dataLoaded, setDataLoaded] = useState<any>(undefined);
  const [currentMenu, setCurrentMenu] = useState(MENU_ACTUAL.CHAT);
  const [statusSubMenu, setStatusSubMenu] = useState(0);
  const [messages, setMessages] = useState<Array<ChatMessage>>([]);
  const commands = useRef<Array<Command>>([])
  const [currentCommand, setCurrentCommand] = useState<undefined | Command>(undefined);
  const [forceUpdate, setForceUpdate] = useState<boolean>(true);
  const commandUses = useRef<Array<CommandUses>>([]);
  const sessionIDtmp = useRef(undefined)
  const botIdChat = useRef("")



  useEffect(() => {
    getDataLoaded();
  }, []);

  useEffect(() => {
    if (dataLoaded != undefined) {
      // Connection opened
      socket.addEventListener("open", _event => {

      });
      // Listen for messages
      socket.addEventListener("message", (event) => process_message(event));
    }

  }, [dataLoaded])
  useEffect(() => {
    if (sessionID != undefined) {
      implement_suscribers(sessionID)
    }

  }, [sessionID]);

  function process_message(event: MessageEvent<any>) {
    if (sessionIDtmp.current == undefined) {
      let session_id = JSON.parse(event.data).payload.session.id;
      sessionIDtmp.current = session_id;
      setSessionID(session_id);
    } else {
      let data = JSON.parse(event.data);
      if (data.metadata.message_type == message_types.NOTIFICATION) {
        if (data.metadata.subscription_type == suscription_types.CHAT_MESSAGE) {
          let event: messageEvent = data.payload.event;
          console.log(commands.current);
          if (event.message.text.startsWith("!") && event.chatter_user_id != botIdChat.current) {
            try_command(event);
          } else {
            add_message(event);
          }

        }
      }

      setGreetMsg("Message from server " + event.data)

    }

  }
  async function update_commands() {
    let dataCommandsString = await invoke("get_data_commands")
    if (dataCommandsString != "") {
      let dataCommands = JSON.parse(dataCommandsString as string);
      let dataUpdate = dataCommands.commands;
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
      //TEST if command used  
      let usable = canIUseCommand(command.command_id, i.chatter_user_id, command.cooldown, commandUses.current);
      if (usable) {
        commandUses.current.push({commandIdUsed: command.command_id, lastTimeUsed: new Date(), userId: i.chatter_user_id});
        invoke('execute_command', { messageTextCommand });

      }

    }
    add_message(i);
  }

  async function implement_suscribers(sessionId: string) {
    console.log(sessionID);
    setGreetMsg(await invoke("implement_suscribers", { sessionId }));
    botIdChat.current = await invoke("get_bot_id");
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

  function setStatusSubMenuInter(e: number) {
    setStatusSubMenu(e);
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

    invoke("save_new_data_token", { newConfigToken }).then(() => setSocket(initializeSocket()));
  }


  function prepare_data_command(data: FormData): Command {
    let commands_data = commands.current;
    let command_id = (commands != undefined && commands_data.length > 0 ? commands_data[commands_data.length - 1].command_id as number + 1 : 1);
    console.log("dataCooldown", (Number.parseInt(data.get("cooldown") as string)));
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
        units: isNaN(Number.parseInt(data.get("cooldown") as string)) ? 0:  (Number.parseInt(data.get("cooldown") as string))  ,
        type_unit: "SECONDS"
      },
      point_cost: Number.parseInt(data.get("point_cost") as string),
      enabled: data.get("enabled") != null
    };
    return new_command;


  }
  function create_comando(e: React.SubmitEvent<HTMLFormElement>) {
    e.preventDefault();
    var formData = new FormData(e.target);
    console.log(formData.get("enabled"));
    console.log(commands);
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
    setStatusSubMenu(MENU_COMANDOS.BASE);
  }

  function delete_command(commandId: Number) {
    invoke('delete_command', { commandId }).then(() => { update_commands() })

  }

  function edit_command(commandId: Number, e: React.SubmitEvent<HTMLFormElement>) {
    e.preventDefault();

    var formData = new FormData(e.target);
    console.log(formData.get("cooldown"));
    let new_command = prepare_data_command(formData);
    console.log(new_command.cooldown?.units);
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
    setStatusSubMenu(MENU_COMANDOS.BASE);
    invoke('edit_command', { commandId, commandData }).then(() => { update_commands() })
  }


  function set_current_command(command: Command) {
    setCurrentCommand(command);
  }

  function renderCurrentView() {
    switch (currentMenu) {
      case MENU_ACTUAL.TOKENS:
        return (<TokenTwitchConfig
          save_token_twitch_config={save_token_twitch_config}
        ></TokenTwitchConfig>)
      case MENU_ACTUAL.COMANDOS:
        return (<CommandList
          commands={commands.current}
          status_sub_menu={statusSubMenu}
          set_status_sub_menu={setStatusSubMenuInter}
          create_comando={create_comando}
          delete_command={delete_command}
          edit_command={edit_command}
          set_current_command={set_current_command}
          current_command={currentCommand}
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
