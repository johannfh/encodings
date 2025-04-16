
export class ByteReader {
    private bitPos = 0;
    private bytePos = 0;

    constructor(private data: Uint8Array) { }

    /**
     * Reads a single bit from the data.
     * @returns The bit value (0 or 1).
     */
    public readBit(): number | undefined {
        if (this.bytePos >= this.data.length) {
            return undefined;
        }

        // Extract the bit at the current position
        const byte = this.data[this.bytePos]
        const bit = (byte >> (7 - this.bitPos)) & 1

        this.bitPos++;
        if (this.bitPos === 8) {
            this.bitPos = 0;
            this.bytePos++;
        }

        return bit;
    }

    public readByte(): number | undefined {
        if (this.remainingBits() < 8) return undefined;

        if (this.bitPos === 0) {
            return this.data[this.bytePos++];
        } else {
            let byte = 0;
            for (let i = 0; i < 8; i++) {
                let bit = this.readBit()!;
                byte = (byte << 1) | bit;
            }
            return byte;
        }
    }

    public remainingBits(): number {
        return (this.data.length * 8) - (this.bytePos * 8 + this.bitPos);
    }

    public remainingBytes(): number {
        return Math.floor(this.remainingBits() / 8);
    }
}