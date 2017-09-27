extern crate rand;

use num::bigint::{BigUint, ToBigUint, RandBigInt};
use num::traits::{Zero, One};


/// Miller Rabin test for primarity
/// n is number to test
/// k is number of tests
/// 
/// As Rabin Miller is probabilistic test there is some chance of false positives
/// roughtly propability of test failure is 0.25 ^ k - so bigger k gets better assurance.
pub fn miller_rabin_test(n: &BigUint, k: usize) -> bool {
    // From wikipedia
    // write n − 1 as 2r·d with d odd by factoring powers of 2 from n − 1
    // WitnessLoop: repeat k times:
    //    pick a random integer a in the range [2, n − 2]
    //    x ← ad mod n
    //    if x = 1 or x = n − 1 then
    //       continue WitnessLoop
    //    repeat r − 1 times:
    //       x ← x2 mod n
    //       if x = 1 then
    //          return composite
    //       if x = n − 1 then
    //          continue WitnessLoop
    //    return composite
    // return probably prime

    let zero = &BigUint::zero();
    let one = &BigUint::one();
    let two = &2.to_biguint().unwrap();
    let mut rng = rand::thread_rng();

    assert!(n % two == *one);
    let n1 = n - one;
    let mut d = n1.clone();
    let mut r = zero.clone();
    while &d % two == *zero {
        r = r + one;
        d = d / two;
    }

    'witness: for _i in 0..k {
        let a = rng.gen_biguint_range(&two, &n1);
        let mut x = modular_pow(&a,&d,n);
        if x == *one || x == n1 {
            continue
        } 
        let mut count= r.clone();
        while count > *one {
            x = (&x * &x) % n;
            if x == *one {
                return false
            }
            if x == n1 {
                continue 'witness;
            }

            count= count - one;
        }

        return false
    }
    true
}

pub fn modular_pow(n: &BigUint, exp: &BigUint, m:&BigUint) -> BigUint{
// From wikipedia
// function modular_pow(base, exponent, modulus)
//     if modulus = 1 then return 0
//     Assert :: (modulus - 1) * (modulus - 1) does not overflow base
//     result := 1
//     base := base mod modulus
//     while exponent > 0
//         if (exponent mod 2 == 1):
//            result := (result * base) mod modulus
//         exponent := exponent >> 1
//         base := (base * base) mod modulus
//     return result

if *m == BigUint::one() {
    return BigUint::zero()
} 

let mut result = BigUint::one();
let mut e = exp.clone();
let mut base = n % m;
while e > BigUint::zero() {
    if &e % 2.to_biguint().unwrap() == BigUint::one() {
        result = (result * &base) % m;
    }

    e = e >> 1;
    base = (&base * &base) % m;

}

result
}
pub fn pow(n: &BigUint, exp: &BigUint) -> BigUint{
    let mut acc = BigUint::one(); 
    let mut base = n.clone();
    let mut e = exp.clone();
    while e > BigUint::one() {
        if &e & BigUint::one() == BigUint::one() {
            acc= acc * &base;
        }
        base = &base * &base;
        e = e / 2.to_biguint().unwrap();
    }

    if e == BigUint::one() {
        acc = acc * base;
    }

    acc
}

#[cfg(test)]
mod tests {
    use super::*;

#[test]
    fn test_miller_rabin_problems() {
        assert!(miller_rabin_test(&113.to_biguint().unwrap(), 30));
    }

    #[test]
    fn test_miller_rabin() {
        let rounds = 7;
        let prime = 982_451_653.to_biguint().unwrap();
        assert!(miller_rabin_test(&prime, rounds));
        assert!(!miller_rabin_test(&(&prime+2.to_biguint().unwrap()), rounds));
        let big_prime = 32_416_190_071u64.to_biguint().unwrap();
        assert!(miller_rabin_test(&big_prime, rounds));
        let big_no_prime = big_prime+2.to_biguint().unwrap();
        println!("Big No Prime : {}", &big_no_prime);
        assert!(!miller_rabin_test(&big_no_prime, rounds));

        let very_big_num = (BigUint::one() << 640) + BigUint::one();
        
        let is_prime = miller_rabin_test(&very_big_num, rounds);
        println!("very big num : {} is prime {}", &very_big_num, is_prime);
        let m521 = (BigUint::one() << 521) - BigUint::one();
        assert!(miller_rabin_test(&m521,rounds));


    }

    #[test]
    fn test_pow() {
        let b = 2.to_biguint().unwrap();
        assert_eq!(pow(&b, &BigUint::zero()), BigUint::one());
        assert_eq!(pow(&b, &BigUint::one()), b.clone());
        assert_eq!(pow(&b, &2.to_biguint().unwrap()), 4.to_biguint().unwrap());
        assert_eq!(pow(&b, &3.to_biguint().unwrap()), 8.to_biguint().unwrap());
        assert_eq!(pow(&b, &4.to_biguint().unwrap()), 16.to_biguint().unwrap());
        assert_eq!(pow(&b, &5.to_biguint().unwrap()), 32.to_biguint().unwrap());
        assert_eq!(pow(&b, &6.to_biguint().unwrap()), 64.to_biguint().unwrap());
        assert_eq!(pow(&b, &7.to_biguint().unwrap()), 128.to_biguint().unwrap());
        assert_eq!(pow(&b, &8.to_biguint().unwrap()), 256.to_biguint().unwrap());
        assert_eq!(pow(&b, &9.to_biguint().unwrap()), 512.to_biguint().unwrap());
    }

    #[test]
    fn test_modular_pow() {
        let b = 2.to_biguint().unwrap();
        let m = 7907.to_biguint().unwrap();
        assert_eq!(modular_pow(&b, &BigUint::zero(), &m), BigUint::one());
        assert_eq!(modular_pow(&b, &BigUint::one(), &m), b.clone());
        assert_eq!(modular_pow(&b, &2.to_biguint().unwrap(), &m), 4.to_biguint().unwrap());
        assert_eq!(modular_pow(&b, &3.to_biguint().unwrap(), &m), 8.to_biguint().unwrap());
        assert_eq!(modular_pow(&b, &4.to_biguint().unwrap(), &m), 16.to_biguint().unwrap());
        assert_eq!(modular_pow(&b, &5.to_biguint().unwrap(), &m), 32.to_biguint().unwrap());
        assert_eq!(modular_pow(&b, &6.to_biguint().unwrap(), &m), 64.to_biguint().unwrap());
        assert_eq!(modular_pow(&b, &7.to_biguint().unwrap(), &m), 128.to_biguint().unwrap());
        assert_eq!(modular_pow(&b, &8.to_biguint().unwrap(), &m), 256.to_biguint().unwrap());
        assert_eq!(modular_pow(&b, &9.to_biguint().unwrap(), &m), 512.to_biguint().unwrap());
    }


}