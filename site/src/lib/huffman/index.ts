
console.log(parseInt("11", 2))


export type HuffmanNode = {
    frequency: number
} & {
    left: HuffmanNode;
    right: HuffmanNode;
} | {
    data: string;
}

function getFrequencyMap(data: string) {
    let result = new Map<string, number>();

    for (let i = 0; i < data.length; i++) {
        const char = data.charAt(i);
        let current = result.get(char);
        if (current) {
            result.set(char, current + 1)
        } else {
            result.set(char, 1)
        }
        console.log(char)
    }

    return result;
}

export function encodeData(data: string) {
    console.log(`Encoding: "${data}"`);
    let frequencies = getFrequencyMap(data);

    console.log(frequencies.entries().toArray().map(([char, freq]) => `${char}: ${freq}`));
}
