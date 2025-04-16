export class ByteWriter {
    private buffer: number[] = [];
    private currentByte = 0;
    private bitPos = 0;

    public writeBits(bits: Iterable<number>) {
        for (const bit of bits) {
            this.writeBit(bit);
        }
    }

    public writeBit(bit: number) {
        if (bit !== 0 && bit !== 1) {
            throw new Error("Bit value must be 0 or 1");
        }

        this.currentByte |= (bit << (7 - this.bitPos))
        this.bitPos++;

        if (this.bitPos === 8) {
            this.buffer.push(this.currentByte);
            this.currentByte = 0;
            this.bitPos = 0
        }
    }

    public writeByte(byte: number) {
        if (byte < 0 || byte > 255) {
            throw new Error("Byte value must be between 0 and 255");
        }

        if (this.bitPos === 0) {
            // If aligned to a byte boundary, directly push the byte
            this.buffer.push(byte)
        } else {
            // If not aligned, write the byte bit by bit
            for (let i = 7; i >= 0; i--) {
                // byte right shifted by i with AND 1 to
                // get the right most bit value from the
                // byte(0 or 1) reading from left to right
                this.writeBit((byte >> i) & 1)
            }
        }
    }

    public flush(): Uint8Array {
        if (this.bitPos !== 0) {
            this.buffer.push(this.currentByte)
        };
        let result = new Uint8Array(this.buffer);
        this.buffer = [];
        this.currentByte = 0;
        this.bitPos = 0;
        return result
    }

    public byteLength(): number {
        return this.buffer.length;
    }

    public bitLength(): number {
        return this.buffer.length * 8 + this.bitPos;
    }
}