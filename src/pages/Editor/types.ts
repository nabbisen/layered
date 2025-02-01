
export interface ParsedMarkdown {
    nodeId: number
    headingLevel: number
    isHeading: boolean
    text: string
    parentNodeId: number | null
    ancestors: number[]
}

export interface BlockContentVisible {
    nodeId: number
    childrenVisible: boolean
}
