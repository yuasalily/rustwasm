/* tslint:disable */
/* eslint-disable */
/**
* 素数判定をする関数です。
* 返り値が0なら素数、0以外なら少なくともその値で割り切れる。
* * `n` - 1以上の18446744073709551615以下の自然数
* @param {bigint} n
* @returns {bigint}
*/
export function is_prime(n: bigint): bigint;
/**
* 素因数分解をする関数です。
* * `n` - 1以上の18446744073709551615以下の自然数
* @param {bigint} n
* @returns {string}
*/
export function prime_factorization(n: bigint): string;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly is_prime: (a: number) => number;
  readonly prime_factorization: (a: number, b: number) => void;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number) => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {SyncInitInput} module
*
* @returns {InitOutput}
*/
export function initSync(module: SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
