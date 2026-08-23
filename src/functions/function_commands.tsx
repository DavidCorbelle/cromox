import { COOLDOWN_MULT } from "../consts";

export function canIUseCommand(command_id: Number, user_id: String, cooldown_command: CommandStrucCooldownType|null, commandUses: Array<CommandUses>): boolean {
    console.log("Entra");
    if (cooldown_command != undefined && cooldown_command.units != 0) {
        console.log("Tiene Cooldown");
        console.log(commandUses);
        console.log(commandUses[0])
        console.log(command_id)
        if(commandUses.length>0){
            console.log(commandUses[0].commandIdUsed == command_id)
        }
        

        let uses_find: Array<CommandUses> = commandUses.filter((e) => { return(e.commandIdUsed == command_id) });
        console.log(uses_find);
        if (uses_find.length > 0) {
            console.log("Encuentra Usos");
            let lastUse = Math.max(...uses_find.map(o => o.lastTimeUsed.getTime()));
            let current_moment = (new Date()).getTime();
            let cooldown_time: number = COOLDOWN_MULT[cooldown_command.type_unit];
            console.log(lastUse + (cooldown_time * cooldown_command.units) > current_moment)
            console.log(lastUse + (cooldown_time * cooldown_command.units) - current_moment)
            if (lastUse + (cooldown_time * cooldown_command.units) > current_moment) {
                console.log("Uso Reciente");
                return false
            }
        }
    }


    return true;
}