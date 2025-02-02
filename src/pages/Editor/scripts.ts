import { type ParsedMarkdown } from "./types"

export const findMaxHeadingLevel = (parsedMarkdowns: ParsedMarkdown[]): number => {
    let ret: number = 1
    parsedMarkdowns.forEach((x: ParsedMarkdown) => {
        if (ret < x.headingLevel) ret = x.headingLevel
    })
    return ret
}

export const mapNodeVisibles = (parsedMarkdowns: ParsedMarkdown[], maxVisibleNodeLevel: number | null): ParsedMarkdown[] => {
    const ret = parsedMarkdowns.map((x) => {
        const mod = x

        if (maxVisibleNodeLevel === null) {
            mod.visible = true
            return mod
        }

        if (mod.isHeading) {
            mod.visible = mod.headingLevel <= maxVisibleNodeLevel
        } else {
            mod.visible = mod.headingLevel < maxVisibleNodeLevel
        }
        return mod
    })
    return ret
}

export const hasNodeChildren = (nodeId: number, parsedMarkdowns: ParsedMarkdown[]): boolean => {
    return parsedMarkdowns.some((x) => x.parentNodeId === nodeId)
}

export const isNodeChildrenVisible = (nodeId: number, parsedMarkdowns: ParsedMarkdown[]): boolean => {
    return parsedMarkdowns.some((x) => x.parentNodeId === nodeId && x.visible === true)
}