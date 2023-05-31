# SHAKE Multihash

Implements [SHAKE](https://en.wikipedia.org/wiki/SHA-3) multihash for [multiformats](https://github.com/multiformats/rust-multihash) so you can generate a Content Identifier (CID) for a file using SHAKE.

## Default Usage (48 byte Length)

```rust
use shake_multihash::{Code};
use multihash::MultihashDigest;

let data = b"shake, shake, shake... shake shake shake... shake your booty!";
let mhash = Code::Shake256_48.digest(data);

println!("{:?}", mhash);
```

## Custom Length Usage

Since Shake digest is designed to be of any arbitrary length, this library uses a macro to generate the multihash code for each length.

It uses a default length of 48 bytes, but you can change it to whatever by modifying the custom length in the macro.

The example below shows how to use a custom length of 42 bytes.

```rust
use multihash::MultihashDigest;
use multihash::derive::Multihash;
use shake_multihash::{SHAKE_256_HASH_CODE};

#[derive(Clone, Copy, Debug, Eq, Multihash, PartialEq)]
#[mh(alloc_size = 64)]
enum MyCode {
    #[mh(code = SHAKE_256_HASH_CODE, hasher = special_shake::Shake256_42)]
    Shake256_42,
}

mod special_shake {
    const CUSTOM_LEN: usize = 42;
    shake_multihash::derive_rustcrypto_shaker!(::sha3::Shake256, Shake256_42, 42);
}

// now you can use the custom length:
let mhash = MyCode::Shake256_42.digest(b"hello world");

println!("{:?}", mhash);
```
