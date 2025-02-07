export type EditorLayout = 'raw' | 'both' | 'layers'

export interface ParsedMarkdown {
    nodeId: number
    headingLevel: number
    isHeading: boolean
    text: string
    parentNodeId: number | null
    ancestors: number[]
    visible: boolean | null
}
