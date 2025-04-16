import { ByteReader } from './byte-reader';
import { ByteWriter } from './byte-writer';

export { ByteReader, ByteWriter };

export function uint8ArrayToBinary(value: Uint8Array): string {
    let bitArray: number[][] = [];
    let reader = new ByteReader(value);
    while (reader.remainingBits() > 0) {
        if (bitArray.length === 0) bitArray.push([]);
        if (bitArray[bitArray.length - 1].length >= 8) bitArray.push([]);
        let bit = reader.readBit()!;
        bitArray[bitArray.length - 1].push(bit);
    }

    return bitArray.map((v) => v.join('')).join(' ');
}