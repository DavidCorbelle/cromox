import { useState } from "react";
import { MENU_INTEGRACIONES } from "../../consts";
import IntegrationVtubestudio from "./VTubeStudio/IntegrationVtubestudio";
import { MenuIntegrationProps } from "./types.td";


const MenuIntegration = ({ integrationsStarted }: MenuIntegrationProps) => {
    const [statusMenu, SetStatusMenu] = useState<number>(0);

    function renderMenuIntegracion() {
        switch (statusMenu) {
            case MENU_INTEGRACIONES.VTUBESTUDIO:
                return (<IntegrationVtubestudio integrationStarted={integrationsStarted.VtubeStudio}></IntegrationVtubestudio>)
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