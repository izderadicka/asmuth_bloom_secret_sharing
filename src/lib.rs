extern crate num;
#[macro_use]
extern crate quick_error;
extern crate rand;

use num::bigint::{BigInt, BigUint, RandBigInt, ToBigInt, ToBigUint};
use num::traits::{One, Zero};
use rand::os::OsRng;

mod ops;

use ops::{miller_rabin_test, pow};


pub struct BigPrimesGenerator {
    last_odd: BigUint,
    tests: usize,
}

impl BigPrimesGenerator {
    /// Creates new primes iterator for primes bigger then
    /// given limit.
    /// As the primarity test is probabilistic,  tolerable
    /// error_level (probability number not being prime) must be supplied
    /// for example error_level = 1e-9 means there that
    /// for each generated number there is probablity 1 in bilion
    /// that number is not prime
    pub fn new(bigger_then: &BigUint, error_level: f64) -> Self {
        assert!(error_level < 1.0 && error_level > 0.0);
        assert!(*bigger_then > BigUint::zero());
        let start;
        let tests = error_level.log(0.25).ceil() as usize;
        if bigger_then % 2.to_biguint().unwrap() == BigUint::zero() {
            start = bigger_then - BigUint::one();
        } else {
            start = bigger_then.clone();
        }

        BigPrimesGenerator {
            last_odd: start,
            tests,
        }
    }
}

impl Iterator for BigPrimesGenerator {
    type Item = BigUint;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.last_odd = &self.last_odd + 2.to_biguint().unwrap();
            if miller_rabin_test(&self.last_odd, self.tests) {
                return Some(self.last_odd.clone());
            }
        }
    }
}

/// Generator of shared secrects
pub struct AsmuthBloomShare {
    threshold: u16,
    max_bits: u16,
    primes: Vec<BigUint>,
    n0: BigUint,
    rng: OsRng,
}

/// Recoveror from shared secrects
pub struct AsmuthBloomRecover {
    threshold: u16,
}

/// Structure representing shared secret
pub struct ABSharedSecret {
    n0: BigUint,
    shares: Vec<(BigUint, BigUint)>,
}

fn gen_primes(min_n1_bits: u16, n: u16, error_level: f64) -> Vec<BigUint> {
    let min1_n1 = pow(&2.to_biguint().unwrap(), &min_n1_bits.to_biguint().unwrap());
    BigPrimesGenerator::new(&min1_n1, error_level)
        .take(n as usize)
        .collect()
}


fn test_primes(n0: &BigUint, primes: &Vec<BigUint>, k: u16) -> bool {
    let bi = primes.len() - (k as usize) + 1;
    let low = n0 * primes[bi..].iter().fold(BigUint::one(), |a, b| a * b);
    let high = primes[..k as usize]
        .iter()
        .fold(BigUint::one(), |a, b| a * b);
    return low < high;
}

impl AsmuthBloomShare {
    /// Creates new object for generating shared secrects
    /// max_bits - secret always has to be smaller than this limit
    /// shares - total number of shares to generate
    /// threshhold - minimum number of shared secrects need to recover original secret
    /// error_level - probablity that one of geneated moduli is not prime 
    ///               add there some small number like 1e-9 -see below
    /// 
    /// Asmuth-Bloom scheme depends on the Chinese Reminder Theorem, which requires that
    /// moduli are pairwise coprime. To assure that we generate them as prime numbers, but
    /// since they are big we use probabilistic Miller Rabin test.  Problem can arise only
    /// when they are two false primes with GCD bigger the 1.
    pub fn new(max_bits: u16, shares: u16, threshold: u16, error_level: f64) -> Self {
        assert!(shares >= threshold);
        let min_prime_limit = pow(&2.to_biguint().unwrap(), &max_bits.to_biguint().unwrap());
        let n0 = BigPrimesGenerator::new(&min_prime_limit, error_level)
            .next()
            .unwrap();
        let mut primes;
        let mut min_n1_bits = max_bits + 1;
        let mut tries = 3;
        loop {
            primes = gen_primes(min_n1_bits, shares, error_level);
            if test_primes(&n0, &primes, threshold) {
                break;
            }
            min_n1_bits += 1;
            tries -= 1;
            if tries <= 0 {
                panic!("Cannot genereate random numbers satisfying AB condition ");
            }
        }

        AsmuthBloomShare {
            threshold,
            max_bits,
            n0,
            primes,
            rng: OsRng::new().unwrap(),
        }
    }
    /// creates shared secrets
    pub fn create_share(&mut self, secret: &[u8]) -> ABSharedSecret {
        assert!(secret.len() * 8 < self.max_bits as usize);
        let s = BigUint::from_bytes_be(secret);
        let max_limit = (self.primes[..self.threshold as usize]
            .iter()
            .fold(BigUint::one(), |a, b| a * b) - &s) / &self.n0;
        let a = self.rng.gen_biguint_range(&BigUint::one(), &max_limit);
        let mod_s = &s + &a * &self.n0;
        let shares: Vec<(BigUint, BigUint)> = self.primes
            .iter()
            .map(|n| (&mod_s % n, n.clone()))
            .collect();
        ABSharedSecret {
            n0: self.n0.clone(),
            shares,
        }
    }
}

fn mul_inv(a: &BigUint, b: &BigUint) -> BigUint {
    if *b == BigUint::one() {
        return BigUint::one();
    }

    let mut t;
    let mut q;
    let mut x0 = BigInt::zero();
    let mut x1 = BigInt::one();
    let mut a = a.to_bigint().unwrap();
    let mut b = b.to_bigint().unwrap();
    let b0 = b.clone();

    while a > BigInt::one() {
        q = &a / &b;
        t = b.clone();
        b = a % b;
        a = t;
        t = x0.clone();
        x0 = x1 - q * x0;
        x1 = t;
    }
    if x1 < BigInt::zero() {
        x1 = x1 + b0;
    }
    x1.to_biguint().unwrap()
}

fn chinese_remainder(c: &[(BigUint, BigUint)]) -> BigUint {
    let len = c.len();
    let mut p;
    let prod = c.iter().map(|a| &a.1).fold(BigUint::one(), |a, b| a * b);
    let mut sum = BigUint::zero();

    for i in 0..len {
        p = &prod / &c[i].1;
        sum = &sum + &c[i].0 * mul_inv(&p, &c[i].1) * p;
    }
    sum % prod
}


impl AsmuthBloomRecover {
    /// Create object to recover original secret
    /// threshold is minimum number of shared secrets required
    pub fn new(threshold: u16) -> Self {
        AsmuthBloomRecover { threshold }
    }
    /// recovers original secret from shared secrets
    pub fn recover_secret(&self, share: &ABSharedSecret) -> Vec<u8> {
        assert!(share.shares.len() >= self.threshold as usize);

        let s0 = chinese_remainder(&share.shares);
        let s = s0 % &share.n0;
        s.to_bytes_be()
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ab_creation() {
        let mut ab = AsmuthBloomShare::new(50, 5, 3, 1e-9);
        assert_eq!(ab.primes.len(), 5);
        {
            let p = &ab.primes;
            assert!(&ab.n0 * &p[3] * &p[4] < &p[0] * &p[1] * &p[2]);
        }
        let mut share = ab.create_share(b"ABCD");
        assert_eq!((&share).shares.len(), 5);

        let abr = AsmuthBloomRecover::new(3);
        share.shares.remove(1);
        share.shares.remove(2);
        let s = abr.recover_secret(&share);

        assert_eq!(&s, b"ABCD");
    }


    #[test]
    #[ignore]
    fn test_ab_creation_big() {
        let mut ab = AsmuthBloomShare::new(800, 7, 4, 1e-12);
        assert_eq!(ab.primes.len(), 7);
        let my_secret=b"This is very secret secret, top secret that no one should know ever forefer";
        let mut share = ab.create_share(my_secret);
        assert_eq!((&share).shares.len(), 7);

        let abr = AsmuthBloomRecover::new(4);
        share.shares.remove(1);
        share.shares.remove(2);
        share.shares.remove(4);
        let s = abr.recover_secret(&share);

        assert_eq!(&s[..], &my_secret[..]);


    }



    #[test]
    fn test_primes_iterator() {
        fn conv<T: ToBigUint>(input: Vec<T>) -> Vec<BigUint> {
            input.into_iter().map(|x| x.to_biguint().unwrap()).collect()
        }
        let gen = BigPrimesGenerator::new(&100.to_biguint().unwrap(), 1e-9);
        let res: Vec<BigUint> = gen.take(10).collect();
        assert_eq!(
            res,
            conv(vec![101, 103, 107, 109, 113, 127, 131, 137, 139, 149])
        );

        let gen = BigPrimesGenerator::new(&961748940.to_biguint().unwrap(), 1e-9);
        let res: Vec<BigUint> = gen.take(16).collect();

        let v = vec![ 961748941, 961748947, 961748951, 961748969, 961748987, 961748993, 961749023, 
        961749037, 961749043, 961749067, 961749079, 961749091, 961749097, 961749101, 961749121, 
        961749157, ];
        assert_eq!(res, conv(v))
    }

}
