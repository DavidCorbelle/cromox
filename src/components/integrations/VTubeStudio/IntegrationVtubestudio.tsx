import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";
import { VtubeStudioConfig, VtubeStudioConfigMenu } from "../types.td";
import { MENU_OPTIONS_VTUBESTUDIO } from "../const";
import MenuAvatares from "./MenuAvatares";
import { listen } from "@tauri-apps/api/event";
import MenuExpresiones from "./MenuExpresiones";
const INTEGRATION_NAME = "VtubeStudio";


const IntegrationVtubestudio = ({ integrationStarted }: VtubeStudioConfigMenu) => {
    const [configVtubestudio, setConfigVtubestudio] = useState<VtubeStudioConfig>();
    const [integrated, setIntegratedStatus] = useState<boolean>(integrationStarted);
    const [menuOptions, setMenuOptions] = useState<Number>(MENU_OPTIONS_VTUBESTUDIO.CONFIGURACION);
    async function start_integration() {
        if (!integrated) {
            invoke('start_config_vtubestudio');
        }
    }
    async function getConfig() {
        let config_string = await invoke('get_config_vtubestudio') as string;
        let config: VtubeStudioConfig = JSON.parse(config_string);
        console.log(config_string);
        setConfigVtubestudio(config);

    }

    useEffect(() => {
        if (integrated == true) {
            getConfig();
        }
    }, [integrated])


    listen<string>('integration-started', (event) => {
        let integration_started = event.payload;
        if (integration_started == INTEGRATION_NAME) setIntegratedStatus(true);
    });

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
                return (<MenuAvatares avatares={configVtubestudio?.models_data}>
                </MenuAvatares>)
            case MENU_OPTIONS_VTUBESTUDIO.EXPRESIONES:
                return(<MenuExpresiones avatares={configVtubestudio?.models_data} expresiones={undefined}></MenuExpresiones>)
            default:
                break;
        }

    }

    return (<div>
        {integrated ? subMenuOptions() : <div>
            <button onClick={() => { start_integration(); }} disabled={integrated} >  Iniciar Integracion Vtubestudio</button>

        </div>}
        {menuOptionsRender()}



    </div >)
}

export default IntegrationVtubestudio