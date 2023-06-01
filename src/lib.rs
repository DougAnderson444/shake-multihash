//! The currently released rust-multihash version, is kind of "catch all" for all sorts of hash function.
//! Current `rust-multihash` master has already split things into smaller pieces.
//!
//! In theory it should be possible to implement other hash functions
//! outside of this crate an integrate it in your own custom code table.

use multihash::derive::Multihash;
use sha3::digest::{ExtendableOutput, Update, XofReader};

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

/// Illustrative purposes only, do not use. Multicodec codes for Shake128 and Shake256 of fixed lengths `48 bytes`
#[derive(Clone, Copy, Debug, Eq, Multihash, PartialEq)]
#[mh(alloc_size = 64)]
pub enum Code {
    #[mh(code = SHAKE_128_HASH_CODE, hasher = shake::Shake128_48)]
    Shake128_48,
    // make another one using the macro
    #[mh(code = SHAKE_256_HASH_CODE, hasher = shake::Shake256_48)]
    Shake256_48,
}

/// Illustrative purposes only, do not use. Multicodec codes for Shake128 and Shake256 of fixed lengths `48 bytes`
mod shake {
    crate::derive_rustcrypto_shaker!(::sha3::Shake128, Shake128_48, super::SHAKE_128_48_LEN);
    crate::derive_rustcrypto_shaker!(::sha3::Shake256, Shake256_48, super::SHAKE_256_48_LEN);
}

/// Generate a SHAKE-128 multihash of any length, up to 64 bytes long
///
/// # Example
/// ```
/// use shake_multihash::shake128_mhash;
/// const INPUT: &[u8] = b"shake, shake, shake... shake shake shake... shake your booty!";
/// let mut sized_array = [0u8; 42];
/// let mhash = shake128_mhash(INPUT, &mut sized_array).unwrap();
/// ```
pub fn shake128_mhash(
    input: &[u8],
    output_buffer: &mut [u8],
) -> Result<multihash::MultihashGeneric<64>, multihash::Error> {
    let mut hasher = sha3::Shake128::default();
    hasher.update(input);
    let mut reader = hasher.finalize_xof();
    reader.read(output_buffer);

    Multihash::wrap(SHAKE_128_HASH_CODE, output_buffer)
}

/// Generate a SHAKE-256 multihash of any length, up to 64 bytes long
///
/// # Example
/// ```
/// use shake_multihash::shake256_mhash;
/// const INPUT: &[u8] = b"shake, shake, shake... shake shake shake... shake your booty!";
/// let mut sized_array = [0u8; 42];
/// let mhash = shake256_mhash(INPUT, &mut sized_array).unwrap();
/// ```
pub fn shake256_mhash(
    input: &[u8],
    output_buffer: &mut [u8],
) -> Result<multihash::MultihashGeneric<64>, multihash::Error> {
    // generate SHAKE digest
    let mut hasher = sha3::Shake256::default();
    hasher.update(input);
    let mut reader = hasher.finalize_xof();
    reader.read(output_buffer);

    Multihash::wrap(SHAKE_256_HASH_CODE, output_buffer)
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

        // print mhash
        eprintln!("mhash: {:X?}", mhash.digest());

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

    #[test]
    fn test_straight_impl() {
        use super::SHAKE_256_HASH_CODE;

        use multihash::Multihash;
        use sha3::{
            digest::{ExtendableOutput, Update, XofReader},
            Shake256,
        };

        // generate SHAKE digest
        let mut hasher = Shake256::default();
        hasher.update(INPUT);
        let mut reader = hasher.finalize_xof();
        let mut digest = [0u8; 48];
        reader.read(&mut digest);

        let mhash = Multihash::wrap(SHAKE_256_HASH_CODE, &digest).unwrap();

        // print mhash
        eprintln!("mhash: {:X?}", mhash.digest());

        let field_element_from_mhash = FieldElement::from_bytes(mhash.digest()).unwrap();
        let field_element_from_digest = FieldElement::from_bytes(&digest).unwrap();

        // assert same as FieldElement::from_msg_hash(input)
        let straight_outta_input = FieldElement::from_msg_hash(INPUT);
        assert_eq!(field_element_from_mhash, field_element_from_digest);
        assert_eq!(field_element_from_mhash, straight_outta_input);
    }

    #[test]
    fn test_shortcut() {
        use super::*;

        // test shake256_mhash
        let mut digest = [0u8; 48];
        let mhash = shake256_mhash(INPUT, &mut digest).unwrap();

        let field_element_from_mhash = FieldElement::from_bytes(mhash.digest()).unwrap();
        let field_element_from_digest = FieldElement::from_bytes(&digest).unwrap();

        // assert same as FieldElement::from_msg_hash(input)
        let straight_outta_input = FieldElement::from_msg_hash(INPUT);
        assert_eq!(field_element_from_mhash, field_element_from_digest);
        assert_eq!(field_element_from_mhash, straight_outta_input);

        // test shake256_mhash
        let mut digest = [0u8; 48];
        let mhash = shake128_mhash(INPUT, &mut digest).unwrap();

        // generate SHAKE digest
        let mut hasher = sha3::Shake128::default();
        hasher.update(INPUT);
        let mut reader = hasher.finalize_xof();
        let mut digest_2 = [0u8; 48];
        reader.read(&mut digest_2);

        // assert is same as digest_2
        assert_eq!(mhash.digest(), digest_2);
    }
}
