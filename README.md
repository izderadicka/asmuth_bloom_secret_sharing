# Asmuth-Bloom Secret Sharing

[![Build Status](https://travis-ci.org/izderadicka/asmuth_bloom_secret_sharing.svg?branch=master)](https://travis-ci.org/izderadicka/asmuth_bloom_secret_sharing)

[Asmuth-Bloom algorithm](https://en.wikipedia.org/wiki/Secret_sharing_using_the_Chinese_remainder_theorem#Asmuth-Bloom.27s_threshold_secret_sharing_scheme) implemented in Rust.

My learning exercise so cannot guarantee that it's bullet-proof. If you intend to use it in your project I recommend to review the code ( or use some proven Shamir secret sharing library, as Asmuth Bloom is bit obscure - but theoretically fine).

## How to use

```rust
// Secret is max. 50 bits - this information is basically visible in shared secret, 
// so normaly use bigger number to increase search space for brute force attacks
// 5 shares total, 3 minimum to recover secret (threshold)
let mut ab = AsmuthBloomShare::new(50, 5, 3, 1e-9);
let mut share = ab.create_share(b"ABCD");
// Again threshold is needed to check if we have enough shares
let abr = AsmuthBloomRecover::new(3);
// remove 2 to leave 3
share.shares.remove(1);
share.shares.remove(2);
let s = abr.recover_secret(&share);
assert_eq!(&s, b"ABCD");
```