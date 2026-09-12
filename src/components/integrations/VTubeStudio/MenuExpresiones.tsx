import { useRef, useState } from "react";
import { AvataresVTubeStudio, ExpresionesVtubeStudio, MenuExpresionesVtubestudio, ShorcutData } from "../types.td";
import { invoke } from "@tauri-apps/api/core";
import { VTUBESTUDIO_ACTIONS } from "../const";
import { listen } from "@tauri-apps/api/event";

const MenuExpresiones = ({ avatares, expresiones }: MenuExpresionesVtubestudio) => {
    const [avatarSeleccionado, setAvatarSeleccionado] = useState<AvataresVTubeStudio | undefined>(avatares ? avatares[0] : undefined);
    const [expresionesListado, setListadoExpresiones] = useState<Array<ExpresionesVtubeStudio> | undefined>(expresiones);
    const expresionSeleccionada = useRef<ExpresionesVtubeStudio>(undefined)


    async function getListadoExpresiones() {
        let action = VTUBESTUDIO_ACTIONS.GET_EXPRESSIONS_MODEL;
        let param = "";
        let res = await invoke('actions_vtubestudio', { action, param });
        console.log(res);
        let expresiones_tmp = JSON.parse(res as string);
        console.log(expresiones_tmp);
        setListadoExpresiones(expresiones_tmp)

    }
    async function asociar_atajo(expresion: ExpresionesVtubeStudio) {
        console.log("intenta asociar");
        invoke('action_new_shorcut');
        expresionSeleccionada.current = expresion;
    }
    async function save_shorcuts_expressions(e: React.SubmitEvent<HTMLFormElement>) {
        e.preventDefault();
        if (expresionesListado != undefined) {
            let data: Array<ShorcutData> = [];
            expresionesListado.forEach(element => {
                data.push({ id_item: element.expression_file, parent_id: element.model_id, shortcut: element.shortcut })
            });
            let dataSend = JSON.stringify(data);
            invoke('vtubestudio_save_shorcuts_expressions', { dataSend })
            console.log(expresionesListado);
        }

    }

    listen<string>('hotkey-pressed', (event) => {
        if (expresionesListado != undefined && expresionSeleccionada.current != undefined) {
            let new_shorcut = event.payload as string;
            let listadoTmp: Array<ExpresionesVtubeStudio> = [...expresionesListado];
            listadoTmp.forEach(element => {
                if (expresionSeleccionada.current != undefined && element.model_id == expresionSeleccionada.current.model_id && element.expression_file == expresionSeleccionada.current.expression_file) {
                    element.shortcut = new_shorcut;
                }
            });
            setListadoExpresiones(listadoTmp);
            expresionSeleccionada.current = undefined;
        }

    });
    function selectAvatar(e: string) {
        console.log(e);
        let avFilter = avatares?.filter((av) => av.model_id == e);
        if (avFilter != undefined && avFilter.length > 0) {
            let avatarSeleccionado = avFilter[0];
            setAvatarSeleccionado(avatarSeleccionado);
            console.log(avFilter)
        }

    }
    return (<div>
        <button onClick={getListadoExpresiones}>Recuperar Expresiones</button>
        <label>Avatar <select onChange={(e) => selectAvatar(e.target.value)}>{avatares ? avatares.map((av) => {
            return (<option id={av.model_id} value={av.model_id}>{av.model_name}</option>)
        }) : null}
        </select></label>
        <div>
            <form onSubmit={(e) => { save_shorcuts_expressions(e) }}>
                {expresionesListado != undefined ? expresionesListado.map((expr) => {
                    if (expr.model_id == avatarSeleccionado?.model_id) {
                        return (<>
                            <div>
                                <div>{expr.expression_name}</div>
                                <input id={expr.expression_name} type="text" disabled value={expr.shortcut}></input>
                                <button type="button" onClick={() => { asociar_atajo(expr) }}>Asociar atajo</button>
                            </div></>)
                    }

                }) : null}
                <button>Guardar Ajustes</button>
            </form>
        </div>
    </div>)
}
export default MenuExpresiones;