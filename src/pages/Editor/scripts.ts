import { type ParsedMarkdown } from "./types"

export const getMaxHeadingLevel = (parsedMarkdowns: ParsedMarkdown[]) => {
    let ret: number = 1
    parsedMarkdowns.forEach((x: ParsedMarkdown) => {
        if (ret < x.heading_level) ret = x.heading_level
    })
    return ret
}

export const isBlockLeadingVisible = (nestingLevel: number | null, visibleLevel: number | null): boolean => {
    if (!visibleLevel || Number.isNaN(visibleLevel)) {
        return true
    }
    if (!nestingLevel || Number.isNaN(nestingLevel)) {
        return false
    }
    return Number(nestingLevel) <= Number(visibleLevel)
}

export const isBlockContentVisible = (headingLevel: number, visibleLevel: number | null, text: string): boolean => {
    if (!visibleLevel || Number.isNaN(visibleLevel)) {
        return false
    }
    return Number(headingLevel) < Number(visibleLevel) ? text !== null : false
}
