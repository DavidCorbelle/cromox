import { COOLDOWN_MULT } from "../consts";

export function canIUseCommand(command_id: Number, user_id: String, cooldown_command: CommandStrucCooldownType | null, commandUses: Array<CommandUses>): boolean {
    console.log(user_id);
    if (cooldown_command != undefined && cooldown_command.units != 0) {
        let uses_find: Array<CommandUses> = commandUses.filter((e) => { return (e.commandIdUsed == command_id) });
        if (uses_find.length > 0) {
            let lastUse = Math.max(...uses_find.map(o => o.lastTimeUsed.getTime()));
            let current_moment = (new Date()).getTime();
            let cooldown_time: number = COOLDOWN_MULT[cooldown_command.type_unit];
            if (lastUse + (cooldown_time * cooldown_command.units) > current_moment) {
                return false
            }
        }
    }
    return true;
}