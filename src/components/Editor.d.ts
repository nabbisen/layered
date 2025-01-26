
export interface ParsedMarkdown {
    node_id: number
    ancestors: number[]
    nesting_level: number
    heading_level: number | null
    heading_text: string | null
    html: string | null
}
