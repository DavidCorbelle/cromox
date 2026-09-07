import { useRef, useState } from "react";
import { AvataresVTubeStudio } from "../types.td";
import { invoke } from "@tauri-apps/api/core";
import { VTUBESTUDIO_ACTIONS } from "../const";
import { listen } from "@tauri-apps/api/event";

const MenuAvatares = ({ }) => {
    const [avataresListado, setListadoAvatares] = useState<Array<AvataresVTubeStudio>>();
    const [forceUpdate, setForceUpdate]= useState(false);
    const avatarSeleccionado = useRef<AvataresVTubeStudio>(undefined)


    async function getListadoAvatares() {
        let action = VTUBESTUDIO_ACTIONS.GET_MODELS;
        let param = "";
        let res = await invoke('actions_vtubestudio', { action, param });
        let avatares = JSON.parse(res as string);
        console.log(avatares);
        setListadoAvatares(avatares)

    }
    async function asociar_atajo(avatar: AvataresVTubeStudio) {
        console.log("intenta asociar");
        invoke('action_new_shorcut');
        avatarSeleccionado.current = avatar;
    }

    listen<string>('hotkey-pressed', (event) => {
        if (avataresListado != undefined && avatarSeleccionado.current != undefined) {
            let new_shorcut = event.payload as string;
            let listadoTmp: Array<AvataresVTubeStudio> = avataresListado;
            listadoTmp.forEach(element => {
                if (avatarSeleccionado.current != undefined && element.model_id == avatarSeleccionado.current.model_id) {
                    element.model_shortcut = new_shorcut;
                    let input = document.getElementById(element.model_id) as HTMLInputElement;
                    input.value = new_shorcut;
                }
            });
            setListadoAvatares(listadoTmp);
            avatarSeleccionado.current = undefined;
        }
        
    });

    return (<div>
        <button onClick={getListadoAvatares}>Recuperar Avatares</button>
        <div>
            {avataresListado != undefined ? avataresListado.map((av) => {
                return (<>
                    <div>
                        <div>{av.model_name}</div>
                        <input id={av.model_id} type="text" disabled value={av.model_shortcut}></input>
                        <button type="button" onClick={() => { asociar_atajo(av) }}>Asociar atajo</button>
                    </div></>)
            }) : null}
        </div>
    </div>)
}
export default MenuAvatares;