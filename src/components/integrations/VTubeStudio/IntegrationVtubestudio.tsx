import { invoke } from "@tauri-apps/api/core";
import { useState } from "react";
import { VtubeStudioConfig, VtubeStudioConfigMenu } from "../types.td";
import { MENU_OPTIONS_VTUBESTUDIO } from "../const";
import MenuAvatares from "./MenuAvatares";

async function getConfig() {
    let _config_string = await invoke('get_config_vtubestudio');
    console.log(_config_string);
    return "";
}

const IntegrationVtubestudio = ({ integrationStarted }: VtubeStudioConfigMenu) => {
    const [configVtubestudio, _setConfigVtubestudio] = useState<VtubeStudioConfig>();
    const [menuOptions, setMenuOptions] = useState<Number>(MENU_OPTIONS_VTUBESTUDIO.CONFIGURACION);
    async function start_integration() {
        if (!integrationStarted) {
            invoke('start_config_vtubestudio');
        }

    }


    const subMenuOptions = () => {
        return (<div>
            <button onClick={() => setMenuOptions(MENU_OPTIONS_VTUBESTUDIO.CONFIGURACION)}>Configuracion</button>
            <button onClick={() => setMenuOptions(MENU_OPTIONS_VTUBESTUDIO.AVATARES)}>Avatares</button>
            <button onClick={() => setMenuOptions(MENU_OPTIONS_VTUBESTUDIO.EXPRESIONES)}>Expresiones</button>
            <button onClick={() => setMenuOptions(MENU_OPTIONS_VTUBESTUDIO.ASSETS)}>Assets</button>
        </div>)
    }
    const menuOptionsRender = () => {
        switch (menuOptions) {
            case MENU_OPTIONS_VTUBESTUDIO.CONFIGURACION:
                return (<label>VTube Studio Port
                    <input name="vtube_studio_port" type="number" defaultValue={configVtubestudio != undefined ? configVtubestudio.port : 8001}></input>
                </label>)
            case MENU_OPTIONS_VTUBESTUDIO.AVATARES:
                return (<MenuAvatares>
                </MenuAvatares>)

            default:
                break;
        }

    }

    return (<div>
        {integrationStarted ? subMenuOptions() : <div>
            <button onClick={() => { start_integration(); }} disabled={integrationStarted} >  Iniciar Integracion Vtubestudio</button>

        </div>}
        {menuOptionsRender()}



    </div >)
}

export default IntegrationVtubestudio