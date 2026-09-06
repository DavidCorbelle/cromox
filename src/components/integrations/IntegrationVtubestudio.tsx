import { invoke } from "@tauri-apps/api/core";
import { useState } from "react";

async function getConfig() {
    let config_string = await invoke('get_config_vtubestudio');
    return "";
}

const IntegrationVtubestudio = ({ }) => {
   // const [configVtubestudio, setConfigVtubestudio] = useState(getConfig());
    return (<div>

    </div >)
}

export default IntegrationVtubestudio