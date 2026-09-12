export type VtubeStudioConfig = {
    port: number,
    models_data: Array<AvataresVTubeStudio>
}

export type VtubeStudioConfigMenu = {
    integrationStarted: boolean
}

export type IntegrationStartedList = {
    VtubeStudio: boolean;
}

export type AvataresVTubeStudio = {
    model_name: string,
    model_id: string,
    shortcut: string
}

export type ExpresionesVtubeStudio = {
    expression_file: string,
    expression_name: string,
    model_id: string,
    shortcut: string
}
export type ShorcutData = {
    id_item: string,
    parent_id:string|undefined,
    shortcut: string
}

export type MenuAvataresVtubestudio = {
    avatares: Array<AvataresVTubeStudio> | undefined
}
export type MenuExpresionesVtubestudio = {
    avatares: Array<AvataresVTubeStudio> | undefined
    expresiones: any
}

export type MenuIntegrationProps = {
    integrationsStarted: IntegrationStartedList
}