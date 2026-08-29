
import { Key, useState } from "react";
import { MENU_COMANDOS } from "../../consts";
import { open } from '@tauri-apps/plugin-dialog';
import { Command, COOLDOWN_TYPE } from "../../custom-types/types.td";
import { ComandListComponent } from "../../custom-types/components.td";

const CommandList = ({ commands, create_comando, delete_command, edit_command }: ComandListComponent) => {
    const [statusMenu, SetStatusMenu] = useState<number>(0);
    const [currentCommand, SetCurrentCommand] = useState<Command | undefined>(undefined);
    async function test() {
        let input = document.getElementById("command_form_sound_dir") as HTMLInputElement;
        input.value = await open() as string;
    }

    function renderStatus() {
        switch (statusMenu) {
            case MENU_COMANDOS.BASE:
                return (<><div><button onClick={() => { SetCurrentCommand(undefined); SetStatusMenu(MENU_COMANDOS.CREAR_COMANDO) }}>+</button>
                </div>

                    <div className="commandListShow">
                        <div className="commandListHeader">
                            <div >
                                ID
                            </div>
                            <div >
                                Nombre
                            </div>
                            <div >
                                Activador
                            </div>
                            <div >
                                Coste
                            </div>
                            <div >
                                Activado
                            </div>
                            <div> Acciones</div>
                        </div>
                        {commands != undefined ? commands.map((com: Command) => {
                            return (
                                <div key={"command_" + com.command_id as Key} className="commandListLine">
                                    <div >
                                        {com.command_id.toString()}
                                    </div>
                                    <div >
                                        {com.command_name}
                                    </div>
                                    <div >
                                        {com.trigger}
                                    </div>
                                    <div >
                                        {com.point_cost == null ? 'Sin Coste' : com.point_cost}
                                    </div>
                                    <div >
                                        {com.enabled ? 'Activo' : 'Inactivo'}
                                    </div>
                                    <button onClick={() => { SetCurrentCommand(com); SetStatusMenu(MENU_COMANDOS.CREAR_COMANDO) }}> Editar</button>
                                    <button onClick={() => delete_command(com.command_id)}> Borrar</button>
                                </div>)
                        }) : null}
                    </div></>);
            case MENU_COMANDOS.CREAR_COMANDO:
                return (<>
                    <form onSubmit={(e) => { if (currentCommand == undefined) { create_comando(e); SetStatusMenu(MENU_COMANDOS.BASE) } else { edit_command(currentCommand.command_id, e); SetStatusMenu(MENU_COMANDOS.BASE) } }}>
                        <div className="form_add_command">

                            <label>Nombre Comando
                                <input name="command_name" type="text" defaultValue={currentCommand != undefined ? currentCommand.command_name : ""}></input>
                            </label>
                            <label>Activador Comando
                                <input name="trigger" type="text" defaultValue={currentCommand != undefined ? currentCommand.trigger : ""}></input>
                            </label>

                            <label>Respuesta
                                <textarea name="response_text" defaultValue={currentCommand != undefined ? currentCommand.response_text as string : ""}></textarea>
                            </label>
                            <label>Ruta Sonido
                                <input name="sound_dir" id="command_form_sound_dir" type="text" defaultValue={currentCommand != undefined ? currentCommand.sound?.sound_dir as string : ""}></input>
                                {/*Revisar como conseguir la ruta del archivo*/}
                                <button type="button" onClick={() => test()}>Elegir Ruta</button>
                            </label>
                            <label>Volumen Sonido
                                <input name="sound_dir" id="command_form_sound_dir" type="number" min={0} max={200} defaultValue={currentCommand != undefined ? currentCommand.sound?.sound_volume as number : 100}></input>
                            </label>
                            <label>Tiempo Cooldown
                                <input name="cooldown" type="number" defaultValue={currentCommand != undefined ? currentCommand.cooldown?.units : ""}></input>
                            </label>
                            <label>Tipo Cooldown
                                <select name="type_cooldown" defaultValue={currentCommand != undefined ? currentCommand.cooldown?.type_cooldown : COOLDOWN_TYPE.GENERAL}>
                                    {Object.values(COOLDOWN_TYPE).map((e) => { return (<option value={e}> {e}</option>) })}
                                </select>
                            </label>
                            <label>Permisos
                                <input name="permits" type="text" defaultValue={currentCommand != undefined ? currentCommand.permits.content_type : ""}></input>
                            </label>
                            <label>Coste Puntos
                                <input name="point_cost" type="number" defaultValue={currentCommand != undefined ? currentCommand.point_cost : 0}></input>
                            </label>
                            <label>Activado
                                <input name="enabled" defaultChecked={currentCommand != undefined ? currentCommand.enabled : true} type="checkbox" ></input>
                            </label>

                            <button>Guardar</button>
                        </div>
                    </form >
                </>)
            default:
                break;
        }

    }
    return (<>
        {renderStatus()}
    </>)
}
export default CommandList