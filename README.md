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
let mut share = ab.create_share(b"ABCD").unwrap();
// Again threshold is needed to check if we have enough shares
let abr = AsmuthBloomRecover::new(3);
// remove 2 to leave 3
share.shares.remove(1);
share.shares.remove(2);
let s = abr.recover_secret(&share).unwrap();
assert_eq!(&s, b"ABCD");
```

## Command line utility
There is also command line utility `asmuth_bloom_secret_sharing`. Build release version with cargo build --release and you can test as:
```bash
cargo build --release
pgm=target/release/asmuth_bloom_secret_sharing
$pgm  generate  -n 10 -t 3 my_secret_password | head -3 | $pgm recover -t 3
```

This command creates 10 shared secrest, then limits them only to 3 (threshold) and then recovers original secret.

Output of `generate` sub-command looks like this:
```
2011db:40112b:cevret0vv791j15em18gquca698poc1qhdk
2011db:401139:75586ennsi1vegmf0tc9qpmbpucnfvnq7jh
2011db:40113b:28kvq29nue0ttaoq7g992lj1fdcc38oeo9io
2011db:40117p:2kicmllmgao0jdj4hefdkddogtguceij5bgk
2011db:40119t:2jeoqmicq08m5knnnas4kqq0gn9nqr078nhp
2011db:4011ch:1vf5q51nmqfvr41ob98opv27a43tuje936hc
```

Each line is one shared secret.
