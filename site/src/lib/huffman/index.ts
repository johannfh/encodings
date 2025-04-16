import { MinHeap } from "@datastructures-js/heap";
import type { ByteWriter } from "../bytes";

export class HuffmanNode {
    public frequency: number = 0;
    public left?: HuffmanNode;
    public right?: HuffmanNode;
    public byte?: number;

    constructor({ frequency, byte, left, right }: { frequency: number, byte?: number, left?: HuffmanNode; right?: HuffmanNode }) {
        if (byte !== undefined && (left !== undefined || right !== undefined)) {
            throw new Error("Only leaf nodes may have a value in a HuffmanNode");
        }

        this.frequency = frequency;
        this.byte = byte;
        this.left = left;
        this.right = right;
    }

    public isLeafNode(): boolean {
        return this.byte !== undefined;
    }

    public encodeAsBytes(writer: ByteWriter) {
        if (this.isLeafNode()) {
            writer.writeBit(1);
            writer.writeByte(this.byte!)
        } else {
            writer.writeBit(0);
            this.left!.encodeAsBytes(writer)
            this.right!.encodeAsBytes(writer)
        }
    }
}

function getFrequencyMap(data: Uint8Array) {
    let result = new Map<number, number>();

    for (const byte of data) {
        let current = result.get(byte);
        if (current) {
            result.set(byte, current + 1)
        } else {
            result.set(byte, 1)
        }
    }

    return result;
}

/**
 * Generates a huffman tree for a string.
 * @param data The data from which to generate the huffman tree
 * @returns The huffman tree or `null` when `data` is empty.
 */
export function generateHuffmanTree(data: Uint8Array): HuffmanNode | null {
    let frequencies = getFrequencyMap(data);

    let heap = new MinHeap<HuffmanNode>((node) => node.frequency)

    console.log(frequencies)

    frequencies.entries().map(([byte, frequency]): HuffmanNode => {
        return new HuffmanNode({ frequency, byte })
    }).forEach(node => heap.insert(node))

    while (heap.size() > 1) {
        let left = heap.pop()!;
        let right = heap.pop()!;

        let merged = new HuffmanNode({
            frequency: left.frequency + right.frequency,
            left, right,
        })

        heap.insert(merged)
    }

    return heap.pop()
}

let encoder = new TextEncoder()

export function encodeData(data: string) {
    console.log(`Encoding: "${data}"`);
    let frequencies = getFrequencyMap(encoder.encode(data));

    console.log(frequencies.entries().toArray()
        .map(([char, freq]) => `${char}: ${freq}`));
}
