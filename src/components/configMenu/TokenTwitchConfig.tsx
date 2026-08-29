import { TYPE_TOKEN } from "../../consts";
import { TokenTwitchConfigComponent } from "../../custom-types/components.td";

const TokenTwitchConfig = ({ bot_token_loaded, get_new_token_bot, get_new_token_streamer, streamer_token_loaded }: TokenTwitchConfigComponent) => {

    return (<div>
        <div>
            {streamer_token_loaded ? <div className="loaded_ok"></div> : <div className="loaded_ko"></div>}
            <button onClick={() => get_new_token_streamer(TYPE_TOKEN.STREAMER)}> NEW STREAMER TOKEN</button>
        </div>
        <div>
            {bot_token_loaded ? <div className="loaded_ok"></div> : <div className="loaded_ko"></div>}
            <button onClick={() => get_new_token_bot(TYPE_TOKEN.BOT)}> NEW BOT TOKEN</button>
        </div >
    </div >)
}

export default TokenTwitchConfig