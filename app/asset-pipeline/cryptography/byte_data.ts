
export class ByteData {

    arr: Uint8Array
    b64: string
    hex: string

    constructor(buf) {
        if (!arguments.length) {
            this.arr = null;
            this.b64 = null;
            return;
        }

        this.arr = new Uint8Array(buf);
        this.b64 = this.toB64(buf);
        this.hex = this.toHex(buf);
    }

    toB64(buf) {
        let binary = '';
        const bytes = new Uint8Array(buf);
        for (let i = 0; i < bytes.byteLength; i++) {
            binary += String.fromCharCode(bytes[i]);
        }
        return btoa(binary);
    }

    addNewLines(str: string) {
        var finalString = '';
        while (str.length > 0) {
            finalString += str.substring(0, 64) + '\n';
            str = str.substring(64);
        }

        return finalString;
    }

    toPem(type: string) {
        var b64WithLines = this.addNewLines(this.b64);
        var pem = "-----BEGIN " + type + " KEY-----\n" + b64WithLines + "-----END " + type + " KEY-----";

        return pem;
    }

    toHex(bytes: Uint8Array) {
        for (var hex = [], i = 0; i < bytes.length; i++) {
            var current = bytes[i] < 0 ? bytes[i] + 256 : bytes[i];
            hex.push((current >>> 4).toString(16));
            hex.push((current & 0xF).toString(16));
        }
        return hex.join("");
    }

    toText() {
        const dec = new TextDecoder()
        return dec.decode(this.arr)
    }

    static fromB64(base64: string): ByteData {
        var binary_string = atob(base64);
        var len = binary_string.length;
        var bytes = new Uint8Array(len);
        for (var i = 0; i < len; i++) {
            bytes[i] = binary_string.charCodeAt(i);
        }
        return new this(bytes)
    }

    static fromHex(hex: string) {
        for (var bytes = [], c = 0; c < hex.length; c += 2)
            bytes.push(parseInt(hex.substr(c, 2), 16));
        return new this(bytes)
    }

    static fromText(text: string) {
        const enc = new TextEncoder()
        return new this(enc.encode(text))
    }
}