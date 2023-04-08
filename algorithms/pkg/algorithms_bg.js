let wasm;
export function __wbg_set_wasm(val) {
    wasm = val;
}

/**
* 素数判定をする関数です。
* 返り値が0なら素数、0以外なら少なくともその値で割り切れる。
* * `n` - 1以上の18446744073709551615以下の自然数
* @param {bigint} n
* @returns {bigint}
*/
export function is_prime(n) {
    const ret = wasm.is_prime(n);
    return BigInt.asUintN(64, ret);
}

let cachedInt32Memory0 = null;

function getInt32Memory0() {
    if (cachedInt32Memory0 === null || cachedInt32Memory0.byteLength === 0) {
        cachedInt32Memory0 = new Int32Array(wasm.memory.buffer);
    }
    return cachedInt32Memory0;
}

const lTextDecoder = typeof TextDecoder === 'undefined' ? (0, module.require)('util').TextDecoder : TextDecoder;

let cachedTextDecoder = new lTextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

let cachedUint8Memory0 = null;

function getUint8Memory0() {
    if (cachedUint8Memory0 === null || cachedUint8Memory0.byteLength === 0) {
        cachedUint8Memory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8Memory0;
}

function getStringFromWasm0(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}
/**
* 素因数分解をする関数です。
* * `n` - 1以上の18446744073709551615以下の自然数
* @param {bigint} n
* @returns {string}
*/
export function prime_factorization(n) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        wasm.prime_factorization(retptr, n);
        var r0 = getInt32Memory0()[retptr / 4 + 0];
        var r1 = getInt32Memory0()[retptr / 4 + 1];
        return getStringFromWasm0(r0, r1);
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
        wasm.__wbindgen_free(r0, r1);
    }
}

