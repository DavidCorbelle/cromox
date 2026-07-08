import { useEffect, useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import { useRef } from "react";

function App() {
  const [socket] = useState(new WebSocket("wss://eventsub.wss.twitch.tv/ws"))
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("test2");
  const [sessionID, setSessionID] = useState(undefined);
  const [dataLoaded, setDataLoaded] = useState(undefined);
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
          //setGreetMsg("Message from server "+ event.data)
        }

      });
    }

  }, [dataLoaded])
  useEffect(() => {
    if (sessionID != undefined) {
      implement_suscribers(sessionID)
    }

  }, [sessionID]);

  async function implement_suscribers(sessionId) {
    console.log(sessionID);
    setGreetMsg(await invoke("implement_suscribers", { sessionId }));
  }
  async function getDataLoaded() {
    setDataLoaded(await invoke("start_data_config"));
  }
  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    setGreetMsg(await invoke("greet", { name }));

  }

  return (
    <main className="container">
      <h1>Welcome to Tauri + React</h1>

      <div className="row">
        <a href="https://vite.dev" target="_blank">
          <img src="/vite.svg" className="logo vite" alt="Vite logo" />
        </a>
        <a href="https://tauri.app" target="_blank">
          <img src="/tauri.svg" className="logo tauri" alt="Tauri logo" />
        </a>
        <a href="https://react.dev" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>
      <p>Click on the Tauri, Vite, and React logos to learn more.</p>

      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          greet();
        }}
      >
        <input
          id="greet-input"
          onChange={(e) => setName(e.currentTarget.value)}
          placeholder="Enter a name..."
        />
        <button type="submit">Greet</button>
      </form>
      <p>{dataLoaded}</p>
      <p>{greetMsg}</p>
    </main>
  );
}

export default App;
