# SHAKE128/256 Multihash

Conveneince implementation of [SHAKE](https://en.wikipedia.org/wiki/SHA-3) multihash for [multiformats](https://github.com/multiformats/rust-multihash) so you can generate a Content Identifier (CID) for a file using SHAKE.

Shake lets you generate a digest of any length, so you can generate a digest of length 48 bytes for use with hashing to BLS12-381 curves.

These multihashes can be converted to CIDs to share with your friends as you desire.

## Standard Usage (any size digest, up to `64 bytes`)

Typically use the convenience functions to generate _aritrary_ length (up to `64 bytes`) by passing in an empty buffer of the length you desire:

```rust
use shake_multihash::shake256_mhash;
use shake_multihash::shake128_mhash;

const INPUT: &[u8] = b"shake, shake, shake... shake shake shake... shake your booty!";

// shake256
let mut sized_array = [0u8; 42];
let mhash = shake256_mhash(INPUT, &mut sized_array).unwrap();

// shake128
let mut sized_array = [0u8; 54];
let mhash = shake128_mhash(INPUT, &mut sized_array).unwrap();

```

## Custom Length Usage, larger than `64 bytes`

The `alloc_size` set for the codes defaults to 64 bytes.

Shake digest is designed to be of any arbitrary length which could exceed `64 bytes`. This library uses a macro to generate the multihash code for each length, so if you want to extend the digest past `64 bytes`, you can modify the length in the macro.

The example below shows how to use a custom length of `69 bytes`. If you want to use this multihash to make a very long cid, remember to also extend `CidGeneric` to support the length as below:

```rust
use multihash::MultihashDigest;
use multihash::derive::Multihash;
use shake_multihash::{SHAKE_256_HASH_CODE};

#[derive(Clone, Copy, Debug, Eq, Multihash, PartialEq)]
#[mh(alloc_size = 69)] // <== extended to 69
enum MyCode {
    #[mh(code = SHAKE_256_HASH_CODE, hasher = extra_long_shake::Shake256_69)]
    Shake256_69,
}

mod extra_long_shake {
    const CUSTOM_LEN: usize = 69; // <== extended to 69
    shake_multihash::derive_rustcrypto_shaker!(::sha3::Shake256, Shake256_69, 69);  // <== extended to 69
}

// now you can use the custom length:
let mhash = MyCode::Shake256_69.digest(b"hello world");

const RAW: u64 = 0x55;

let cid = cid::CidGeneric::<69>::new_v1(RAW, mhash);  // <== extended to 69

println!("{:?}", mhash);
```

## Legacy Usage (set at 48 byte length **only**)

For illustration purposes only. This method is not as flexible as the standard usage, as it fixes the length of the digest to `48 bytes` which defeats the purpose of Shake. Use the standard usage above instead.

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

## Test

```bash
cargo test
```
