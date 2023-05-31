# SHAKE Multihash

Implements [SHAKE](https://en.wikipedia.org/wiki/SHA-3) multihash for [multiformats](https://github.com/multiformats/rust-multihash) so you can generate a Content Identifier (CID) for a file using SHAKE.

## Default Usage (48 byte Length)

```rust
use shake_multihash::{Code};
use multihash::MultihashDigest;

const INPUT: &[u8] = b"shake, shake, shake... shake shake shake... shake your booty!";
let mhash = Code::Shake256_48.digest(INPUT);

// perhaps use it to make a CID

const RAW: u64 = 0x55;
let cid = cid::Cid::new_v1(RAW, mhash);

println!("{:?}", cid);
```

## Digest up to `64 bytes`

Alternatively, you can use the convenience functions to generate _aritrary_ length (up to `64 bytes`) by passing in an empty buffer of the length you desire:

```rust
use shake_multihash::shake256_mhash;
use shake_multihash::shake128_mhash;

const INPUT: &[u8] = b"shake, shake, shake... shake shake shake... shake your booty!";

// shake256
let mut digest = [0u8; 42];
let mhash = shake256_mhash(INPUT, &mut digest).unwrap();

// shake128
let mut digest = [0u8; 54];
let mhash = shake128_mhash(INPUT, &mut digest).unwrap();

```

## Custom Length Usage, larger than `64 bytes`

The `alloc_size` set for the codes defaults to 64 bytes.

Since Shake digest is designed to be of any arbitrary length, this library uses a macro to generate the multihash code for each length.

If you want to extend the digest past 64 bytes, you can modify the length in the macro.

The example below shows how to use a custom length of 69 bytes (5 more than 64). If you want to use this multihash to make a very long cid, remember to also extend CidGeneric to support the length. See below:

```rust
use multihash::MultihashDigest;
use multihash::derive::Multihash;
use shake_multihash::{SHAKE_256_HASH_CODE};

#[derive(Clone, Copy, Debug, Eq, Multihash, PartialEq)]
#[mh(alloc_size = 69)] // <== this is the important part
enum MyCode {
    #[mh(code = SHAKE_256_HASH_CODE, hasher = extra_long_shake::Shake256_69)]
    Shake256_69,
}

mod extra_long_shake {
    const CUSTOM_LEN: usize = 69;
    shake_multihash::derive_rustcrypto_shaker!(::sha3::Shake256, Shake256_69, 69);  // <== this is the important part
}

// now you can use the custom length:
let mhash = MyCode::Shake256_69.digest(b"hello world");

const RAW: u64 = 0x55;

let cid = cid::CidGeneric::<69>::new_v1(RAW, mhash);  // <== this is the important part

println!("{:?}", mhash);
```
