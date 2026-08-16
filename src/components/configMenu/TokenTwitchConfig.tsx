const TokenTwitchConfig = ({ save_token_twitch_config }: TokenTwitchConfigComponent) => {



    return (<div>
        <label> client_id
            <input type="text" name="" id="client_id" />
        </label>
        <label> client_secret
            <input type="text" name="" id="client_secret" />
        </label>
        <label> redirect_uri
            <input type="text" name="" id="redirect_uri" />
        </label>
        <label> token
            <input type="text" name="" id="token" />
        </label>
        <label> boradcaster_id
            <input type="text" name="" id="boradcaster_id" />
        </label>
        <label> bot_id
            <input type="text" name="" id="bot_id" />
        </label>
        <button onClick={() => save_token_twitch_config()}> GUARDAR</button>

    </div>)
}

export default TokenTwitchConfig