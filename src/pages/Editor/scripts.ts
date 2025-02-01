import { MIN_NESTING_LEVEL } from "./consts"
import { type ParsedMarkdown } from "./types"

export const isBlockLeadingVisible = (nestingLevel: number | null, visibleLevel: number | null): boolean => {
    if (!visibleLevel || Number.isNaN(visibleLevel)) {
        return true
    }
    if (!nestingLevel || Number.isNaN(nestingLevel)) {
        return false
    }
    return Number(nestingLevel) <= Number(visibleLevel)
}

export const isBlockContentVisible = (nestingLevel: number | null, visibleLevel: number | null, html: string | null): boolean => {
    if (!visibleLevel || Number.isNaN(visibleLevel)) {
        return false
    }
    if (!nestingLevel || Number.isNaN(nestingLevel)) {
        return false
    }
    return Number(nestingLevel) <= Number(visibleLevel) ? html !== null : false
}

export const getMaxNestingLevel = (parsedMarkdowns: ParsedMarkdown[]) => {
    let maxNestingLevel: number = MIN_NESTING_LEVEL
    parsedMarkdowns.forEach((x: ParsedMarkdown) => {
        if (maxNestingLevel < x.nesting_level) maxNestingLevel = x.nesting_level
    })
    return maxNestingLevel
}
