
export interface ParsedMarkdown {
    node_id: number
    heading_level: number
    is_heading: boolean
    text: string
    parent_node_id: number | null
    ancestors: number[]
}
