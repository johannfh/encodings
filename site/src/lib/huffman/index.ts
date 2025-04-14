import { MinHeap, type IGetCompareValue } from "@datastructures-js/heap";


console.log(parseInt("11", 2))

export type HuffmanNode = {
    frequency: number
} & ({
    left: HuffmanNode;
    right: HuffmanNode;
} | {
    data: string;
})

const getHuffmanNodeCompareValue: IGetCompareValue<HuffmanNode> = (node) => node.frequency;

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

/**
 * Generates a huffman tree for a string.
 * @param data The data from which to generate the huffman tree
 * @returns The huffman tree or `null` when `data` is empty.
 */
export function generateHuffmanTree(data: string): HuffmanNode | null {
    let frequencies = getFrequencyMap(data);

    let heap = new MinHeap<HuffmanNode>(getHuffmanNodeCompareValue)

    console.log(frequencies)

    frequencies.entries().map(([char, freq]): HuffmanNode => {
        return { data: char, frequency: freq }
    }).forEach(node => heap.insert(node))

    while (heap.size() > 1) {
        let left = heap.pop()!;
        let right = heap.pop()!;

        let merged: HuffmanNode = {
            frequency: left.frequency + right.frequency,
            left: left, right: right,
        }

        heap.insert(merged)
    }

    return heap.pop()
}

export function encodeData(data: string) {
    console.log(`Encoding: "${data}"`);
    let frequencies = getFrequencyMap(data);

    console.log(frequencies.entries().toArray().map(([char, freq]) => `${char}: ${freq}`));
}
