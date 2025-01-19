import { writable, get } from 'svelte/store'
import type { NestedStringArray } from '../type'

let content = writable<NestedStringArray>()

const { subscribe: subscribeContent } = content

const initContent = (value: NestedStringArray) => {
    content.set(value)
}

const updateContent = (indices: number[], value: string) => {
    content.update((x) => {
        setElementByIndices(x, indices, value)
        return x
    })
}

const getIndicedsContent = (indices: number[]): string | undefined => {
    return getElementByIndices(get(content), indices)
}

const exchangeItems = (fromIndices: number[], toIndices: number[]) => {
    content.update((x: NestedStringArray) => {
        swapElements(x, fromIndices, toIndices);
        console.log(x, fromIndices, toIndices)
        // let fromItem: any = getNestedArrayReference(x, fromIndices)
        // let toItem: any = getNestedArrayReference(x, toIndices)
        // console.log(x, ret, fromIndices, fromItem, toIndices, toItem)
        // const orgToValue = toItem
        // toItem = fromItem
        // fromItem = orgToValue
        return x
    })
}

export {
    subscribeContent,
    initContent,
    updateContent,
    getIndicedsContent,
    exchangeItems,
}

// todo: local defs / fns below

type WritableNestedStringArray = { [index: number]: string | WritableNestedStringArray };

function swapElements(arr: WritableNestedStringArray, indices1: number[], indices2: number[]): void {
    let element1 = getElementByIndices(arr, indices1)
    let element2 = getElementByIndices(arr, indices2)
    // swap
    if (element1 !== undefined && element2 !== undefined) {
        setElementByIndices(arr, indices1, element2)
        setElementByIndices(arr, indices2, element1)
    } else {
        console.error("Invalid indices")
    }
}

function getElementByIndices(arr: WritableNestedStringArray, indices: number[]): string | undefined {
    let element = arr
    for (const index of indices) {
        if (Array.isArray(element)) {
            element = element[index] as WritableNestedStringArray;
        } else {
            return undefined
        }
    }
    return element as string
}

function setElementByIndices(arr: WritableNestedStringArray, indices: number[], value: string): void {
    let element = arr
    for (let i = 0; i < indices.length - 1; i++) {
        element = element[indices[i]] as WritableNestedStringArray
    }
    element[indices[indices.length - 1]] = value
}
