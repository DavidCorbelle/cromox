
import { Key } from "react";
import { MENU_COMANDOS } from "../../consts";
import { open } from '@tauri-apps/plugin-dialog';

const CommandList = ({ commands, status_sub_menu, set_status_sub_menu, create_comando, delete_command, edit_command, set_current_command, current_command }: ComandListComponent) => {
    async function test() {
        let input = document.getElementById("command_form_sound_dir") as HTMLInputElement;
        input.value = await open() as string;
    }
    function renderStatus() {
        switch (status_sub_menu) {
            case MENU_COMANDOS.BASE:
                return (<><div><button onClick={() => { set_current_command(undefined); set_status_sub_menu(MENU_COMANDOS.CREAR_COMANDO) }}>+</button>
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
                                <div key={com.command_id as Key} className="commandListLine">
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
                                    <button onClick={() => { set_current_command(com); set_status_sub_menu(MENU_COMANDOS.CREAR_COMANDO) }}> Editar</button>
                                    <button onClick={() => delete_command(com.command_id)}> Borrar</button>
                                </div>)
                        }) : null}
                    </div></>);
            case MENU_COMANDOS.CREAR_COMANDO:
                return (<>
                    <form onSubmit={(e) => { if (current_command == undefined) { create_comando(e) } else { edit_command(current_command.command_id, e) } }}>
                        <div className="form_add_command">
                            <label>Nombre Comando
                                <input name="command_name" type="text" defaultValue={current_command != undefined ? current_command.command_name : ""}></input>
                            </label>
                            <label>Activador Comando
                                <input name="trigger" type="text" defaultValue={current_command != undefined ? current_command.trigger : ""}></input>
                            </label>
                            <label>Respuesta
                                <input name="response_text" type="text" defaultValue={current_command != undefined ? current_command.response_text as string : ""}></input>
                            </label>
                            <label>Ruta Sonido
                                <input name="sound_dir" id="command_form_sound_dir" type="text" defaultValue={current_command != undefined ? current_command.sound_dir as string : ""}></input>
                                {/*Revisar como conseguir la ruta del archivo*/}
                                <button type="button" onClick={() => test()}>Elegir Ruta</button>
                            </label>
                            <label>Tiempo Cooldown
                                <input name="cooldown" type="number" defaultValue={current_command != undefined ? current_command.cooldown?.units : ""}></input>
                            </label>
                            <label>Permisos
                                <input name="permits" type="text" defaultValue={current_command != undefined ? current_command.permits.content_type : ""}></input>
                            </label>
                            <label>Coste Puntos
                                <input name="point_cost" type="number" defaultValue={current_command != undefined ? current_command.point_cost : 0}></input>
                            </label>
                            <label>Activado
                                <input name="enabled" defaultChecked={current_command != undefined ? current_command.enabled : true} type="checkbox" ></input>
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