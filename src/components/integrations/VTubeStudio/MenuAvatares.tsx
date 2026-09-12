import { useRef, useState } from "react";
import { AvataresVTubeStudio, MenuAvataresVtubestudio, ShorcutData } from "../types.td";
import { invoke } from "@tauri-apps/api/core";
import { VTUBESTUDIO_ACTIONS } from "../const";
import { listen } from "@tauri-apps/api/event";

const MenuAvatares = ({avatares}:MenuAvataresVtubestudio) => {
    const [avataresListado, setListadoAvatares] = useState<Array<AvataresVTubeStudio>|undefined>(avatares);
    const avatarSeleccionado = useRef<AvataresVTubeStudio>(undefined)


    async function getListadoAvatares() {
        let action = VTUBESTUDIO_ACTIONS.GET_MODELS;
        let param = "";
        let res = await invoke('actions_vtubestudio', { action, param });
        let avatares_tmp = JSON.parse(res as string);
        console.log(avatares_tmp);
        setListadoAvatares(avatares_tmp)

    }
    async function asociar_atajo(avatar: AvataresVTubeStudio) {
        console.log("intenta asociar");
        invoke('action_new_shorcut');
        avatarSeleccionado.current = avatar;
    }
    async function save_shorcuts_models(e: React.SubmitEvent<HTMLFormElement>) {
        e.preventDefault();
        if (avataresListado != undefined) {
            let data: Array<ShorcutData> = [];
            avataresListado.forEach(element => {
                data.push({ id_item: element.model_id, parent_id:undefined, shortcut: element.shortcut })
            });
            let dataSend = JSON.stringify(data);
            invoke('vtubestudio_save_shorcuts_models', { dataSend })
            console.log(avataresListado);
        }

    }

    listen<string>('hotkey-pressed', (event) => {
        if (avataresListado != undefined && avatarSeleccionado.current != undefined) {
            let new_shorcut = event.payload as string;
            let listadoTmp: Array<AvataresVTubeStudio> = [...avataresListado];
            listadoTmp.forEach(element => {
                if (avatarSeleccionado.current != undefined && element.model_id == avatarSeleccionado.current.model_id) {
                    element.shortcut = new_shorcut;
                }
            });
            setListadoAvatares(listadoTmp);
            avatarSeleccionado.current = undefined;
        }

    });

    return (<div>
        <button onClick={getListadoAvatares}>Recuperar Avatares</button>
        <div>
            <form onSubmit={(e) => { save_shorcuts_models(e) }}>
                {avataresListado != undefined ? avataresListado.map((av) => {
                    return (<>
                        <div>
                            <div>{av.model_name}</div>
                            <input id={av.model_id} type="text" disabled value={av.shortcut}></input>
                            <button type="button" onClick={() => { asociar_atajo(av) }}>Asociar atajo</button>
                        </div></>)
                }) : null}
                <button>Guardar Ajustes</button>
            </form>
        </div>
    </div>)
}
export default MenuAvatares;