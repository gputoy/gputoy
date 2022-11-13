import { writable, type Writable } from "svelte/store"

/**
 * T: Underlying type of store
 * 
 * E: Type for extra methods on store
 */
export type EnhancedWritable<Type, Extras> = Writable<Type> & Extras

/**
 * Constructor for enhanced store. This creates a writable store and
 * passes it to the extras constructor to create the extra methods on store. 
 * @param initial Initial value of store 
 * @param extras Constructor for extra methods within store
 * @returns EnhancedWritable 
 */
export function makeEnhanced<Type, Extras>(
    initial: Type,
    extras: (w: Writable<Type>) => Extras
): () => EnhancedWritable<Type, Extras> {

    return function () {
        const _writable = writable<Type>(initial)
        return { ..._writable, ...extras(_writable) }
    }

}