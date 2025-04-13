let encoder = new TextEncoder();

export function bytes(input?: string): Uint8Array {
    return encoder.encode(input)
}

export function onlyUnique<T>(value: T, index: number, array: T[]) {
    return array.indexOf(value) === index;
}