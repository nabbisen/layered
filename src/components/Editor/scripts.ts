import { type ParsedMarkdown } from "./types"

export const visible = (block: ParsedMarkdown, visibleLevel: number | null): boolean => {
    if (!visibleLevel || Number.isNaN(visibleLevel) || Number(visibleLevel) <= 0) {
        return true
    }
    return block.heading_level ? block.heading_level <= Number(visibleLevel) : false
}

export const maxNestingLevel = (parsedMarkdowns: ParsedMarkdown[]) => {
    let maxLevel: number = 0
    parsedMarkdowns.forEach((x: ParsedMarkdown) => {
        if (maxLevel < x.nesting_level) maxLevel = x.nesting_level
    })
    return maxLevel
}
