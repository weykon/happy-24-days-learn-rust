extern crate primal;
// extern crate lazy_static   <-第一种，是调用第三方模块，通常是rust写的
// 第二种，用于FFI , foreign function interfaces  ，外部函数接口
// 不懂，先不理，反正就是外部传入的库函数，有一些忽略相关？

use primal::Sieve;
pub fn main() {
    let sieve = Sieve::new(10000);
    let suspect = 5273;
    println!("{} is prime: {}", suspect, sieve.is_prime(suspect));
    let not_a_prime = 1024;
    println!("{} is prime: {}", not_a_prime, sieve.is_prime(not_a_prime));
    let n = 1000;

    match sieve.primes_from(0).nth(n - 1) {
        Some(number) => println!("{}th prime is {}", n, number),
        None => println!("I don't know anything about {}th prime.", n),
    }
    // 因式分解 2610 = 2 * 3 * 3 * 5 * 29
    println!("{:?}", sieve.factor(2610));
    //                       ⬆️
    //type Factors = Vec<(usize, usize)>;
    // fn factor(&self, n: usize) -> Result<Factors, (usize, Factors)>

    println!("{:?}", num_divisors(2610, &sieve));
}

// 一个数有多少个除数？ 48 = 1×48 = 2×24 = 3×16 = 4×12 = 6×8
// 所以48有10个除数
fn num_divisors(n: usize, primes: &Sieve) -> Option<usize> {
    match primes.factor(n) {
        Ok(factors) => Some(factors.into_iter().fold(1, |acc, (_, x)| acc * (x + 1))),
        Err(_) => None,
    }
}

// 数学：
// 2^4 x 3^1 = 48 这里有 4次方 和 1次方 相加 = 5 然后 5*2 = 10.

// 12 = 1 x 12 = 2 * 6 = 3 * 4
//    = 2^2 * 3 = 12 => 2 + 1 = 3 , 3 * 2 = 6;

// 15 = 1 x 15 = 3 * 5
//    = 3^1 x 5^1 => 1 + 1 = 2 , 2 * 2 = 4;
