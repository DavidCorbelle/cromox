import { invoke } from "@tauri-apps/api/core";
import { useState } from "react";

async function getConfig() {
    let _config_string = await invoke('get_config_vtubestudio');
    console.log(_config_string);
    return "";
}

const IntegrationVtubestudio = ({ }) => {
    const [_configVtubestudio, _setConfigVtubestudio] = useState(getConfig());
    return (<div>

    </div >)
}

export default IntegrationVtubestudio