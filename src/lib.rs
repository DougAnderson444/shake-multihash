//! The currently released rust-multihash version, is kind of "catch all" for all sorts of hash function.
//! Current master has already split things into smaller pieces.
//!
//! In theory it should be possible to implement other hash functions
//! outside of this crate an integrate it in your own custom code table.

use multihash::derive::Multihash;

// contains the derive macro
pub mod hasher_impl;

/// Default implemented length is 48 bytes
const SHAKE_256_48_LEN: usize = 48;
/// Default implemented length is 48 bytes
const SHAKE_128_48_LEN: usize = 48;

/// Multicodec for Shake128, see [multiformats/multicodec](https://github.com/multiformats/multicodec/blob/df81972d764f30da4ad32e1e5b778d8b619de477/table.csv#L15-L16) for details
/// The code for shake-128 is hex 0x18, decimal 24
pub const SHAKE_128_HASH_CODE: u64 = 24;

/// Multicodec for Shake256, see [multiformats/multicodec](https://github.com/multiformats/multicodec/blob/df81972d764f30da4ad32e1e5b778d8b619de477/table.csv#L15-L16) for details
/// The code for shake-256 is hex 0x19, decimal 25
pub const SHAKE_256_HASH_CODE: u64 = 25;

#[doc = include_str!("../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;

/// Multicodec codes for Shake128 and Shake256 of lengths 48 bytes
#[derive(Clone, Copy, Debug, Eq, Multihash, PartialEq)]
#[mh(alloc_size = 64)]
pub enum Code {
    #[mh(code = SHAKE_128_HASH_CODE, hasher = shake::Shake128_48)]
    Shake128_48,
    // make another one using the macro
    #[mh(code = SHAKE_256_HASH_CODE, hasher = shake::Shake256_48)]
    Shake256_48,
}

mod shake {
    crate::derive_rustcrypto_shaker!(::sha3::Shake128, Shake128_48, super::SHAKE_128_48_LEN);
    crate::derive_rustcrypto_shaker!(::sha3::Shake256, Shake256_48, super::SHAKE_256_48_LEN);
}

#[cfg(test)]
mod tests {
    use amcl_wrapper::field_elem::FieldElement;
    use multibase::Base;
    use multihash::MultihashDigest;

    const RAW: u64 = 0x55;
    const INPUT: &[u8] = b"shake, shake, shake... shake shake shake... shake your booty!";

    #[test]
    fn test_shake_256_48() {
        let mhash = super::Code::Shake256_48.digest(INPUT);
        assert_eq!(mhash.digest().len(), super::SHAKE_256_48_LEN);
        assert_eq!(mhash.size(), super::SHAKE_256_48_LEN as u8);

        // use the mhash to create a cid, just for fun
        let cid = cid::Cid::new_v1(RAW, mhash);

        eprintln!("Cid: {}", cid.to_string_of_base(Base::Base36Lower).unwrap());

        let field_element_from_mhash = FieldElement::from_bytes(mhash.digest()).unwrap();

        // assert same as FieldElement::from_msg_hash(input)
        let straight_outta_input = FieldElement::from_msg_hash(INPUT);
        assert_eq!(field_element_from_mhash, straight_outta_input);
    }

    #[test]
    fn test_128_48() {
        let mhash = super::Code::Shake128_48.digest(INPUT);

        // use the mhash to create a cid, just for fun
        let _cid = cid::Cid::new_v1(RAW, mhash);

        assert_eq!(mhash.size(), super::SHAKE_128_48_LEN as u8);
    }
}
