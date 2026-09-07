export type VtubeStudioConfig = {
    port:number
}

export type VtubeStudioConfigMenu = {
    integrationStarted:boolean
}

export type MenuIntegrationProps = {
    integrationsStarted:IntegrationStartedList;
}

export type IntegrationStartedList = {
    VtubeStudio:boolean;
}

export type AvataresVTubeStudio = {
    model_name:string,
    model_id:string,
    model_shortcut:string
}