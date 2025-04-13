export function bytes(data: string): Uint8Array {
    console.log(`Getting bytes for: ${data}`);
    let result = new Uint8Array();

    let chars = data.split("");
    console.log(`Chars: ${chars}`);

    // TODO: Remove the UTF 8 characters so size of chars as char code is 8-bits big
    chars.map(v => v.)

    let charCodes = chars.map<[string, number]>(v => [v, v.charCodeAt(0)])
    console.log("Max 16-bit value", 2 ** 16 - 1)
    console.log(charCodes)
    return result;
}