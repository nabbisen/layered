import { writable, get } from 'svelte/store'
import { exchangeItems } from './editor'

export interface dragDropItemType {
    indices: number[]
}

let fromDragDropItem = writable<dragDropItemType | undefined>()

const { subscribe: subscribeFromDragDropItem } = fromDragDropItem

const dragStart = (indices: number[]) => {
    fromDragDropItem.set({ indices })
}

const dragCancel = () => {
    clearDrag()
}

const drop = (toIndices: number[]) => {
    if (!fromDragDropItem) return
    const fromIndices: number[] = get(fromDragDropItem)!.indices
    if (fromIndices === toIndices) return
    exchangeItems(fromIndices, toIndices)

    clearDrag()
}

export {
    subscribeFromDragDropItem,
    dragStart,
    dragCancel,
    drop,
}

function clearDrag() {
    fromDragDropItem.set(undefined)
}
