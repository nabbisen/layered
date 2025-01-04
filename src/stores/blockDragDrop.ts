import { writable, get } from 'svelte/store'
import { exchangeItems } from './editor'

interface dragDropItemType {
    indices: number[]
}

let fromDragDropItem = writable<dragDropItemType | undefined>()

const dragStart = (indices: number[]) => {
    fromDragDropItem.set({ indices })
}

const drop = (toIndices: number[]) => {
    if (!fromDragDropItem) return
    const fromIndices: number[] = get(fromDragDropItem)!.indices
    if (fromIndices === toIndices) return
    exchangeItems(fromIndices, toIndices)
    fromDragDropItem.set(undefined)
}

export {
    dragStart,
    drop,
}
