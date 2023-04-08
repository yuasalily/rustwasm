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
