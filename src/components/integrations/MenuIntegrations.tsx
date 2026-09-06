import { useState } from "react";
import { MENU_INTEGRACIONES } from "../../consts";
import IntegrationVtubestudio from "./IntegrationVtubestudio";


const MenuIntegration = ({ }) => {
    const [statusMenu, SetStatusMenu] = useState<number>(0);

    function renderMenuIntegracion() {
        switch (statusMenu) {
            case MENU_INTEGRACIONES.VTUBESTUDIO:
                return (<IntegrationVtubestudio></IntegrationVtubestudio>)
                break;

            default:
                return (<></>)
                break;
        }
    }
    return (<div>
        <div className="buttonsHeader"><button onClick={() => { SetStatusMenu(MENU_INTEGRACIONES.VTUBESTUDIO) }}>Vtubestudio</button></div>
        {renderMenuIntegracion()}
    </div >)
}

export default MenuIntegration