use std::io::Read;

use zerocopy::FromBytes;

use crate::{cxx::bridge::size_of_sanity_header, Error};

/// KenLM Model Sanity check
///
/// This struct is stored in bytes 0-88 in binary KenLM models. As its name says, it's used
/// for sanity-checks. We implement it in rust-land to perform the validation here so that
/// we can avoid violent crashes upon C++ runtime exceptions.
#[repr(C)]
#[derive(Debug, PartialEq, FromBytes)]
pub(crate) struct SanityHeader {
    magic: [u8; SanityHeader::PADDED_MAGIC_SIZE],
    float_zero: f32,
    float_one: f32,
    float_minus_half: f32,
    word_idx_one: u32,
    word_idx_max: u32,
    usize_sanity: u64,
}

const fn align8(size: usize) -> usize {
    let size = size as isize;
    (((((size) - 1) / 8) + 1) * 8) as usize
}

type MAGIC = [u8; 52];

impl SanityHeader {
    // see src/cxx/lm/binary_format.hh & src/cxx/lm/binary_format.cc
    const MAGIC_BYTES: MAGIC = *b"mmap lm http://kheafield.com/code format version 5\n\0";
    const PADDED_MAGIC_SIZE: usize = align8(std::mem::size_of::<MAGIC>());
    const fn padded_magic() -> [u8; Self::PADDED_MAGIC_SIZE] {
        assert!(Self::MAGIC_BYTES.len() <= Self::PADDED_MAGIC_SIZE);
        let mut ary = [0; Self::PADDED_MAGIC_SIZE];
        let mut i = 0;
        while i < Self::MAGIC_BYTES.len() {
            ary[i] = Self::MAGIC_BYTES[i];
            i += 1;
        }
        ary
    }

    pub const REFERENCE: SanityHeader = Self {
        magic: Self::padded_magic(),
        float_zero: 0f32,
        float_one: 1f32,
        float_minus_half: -0.5f32,
        word_idx_one: 1u32,
        word_idx_max: u32::MAX,
        usize_sanity: 1,
    };

    pub(crate) fn from_file(fd: &mut std::fs::File) -> Result<SanityHeader, Error> {
        let mut header_bytes = vec![0; size_of_sanity_header() as usize];
        fd.read_exact(&mut header_bytes)?;
        SanityHeader::read_from(header_bytes.as_slice()).ok_or(Error::SanityFormatError)
    }
}

#[cfg(test)]
mod test {
    use super::SanityHeader;
    #[test]
    fn test_reference_expected() {
        let expected = SanityHeader {
            magic: *b"mmap lm http://kheafield.com/code format version 5\n\0\0\0\0\0",
            float_zero: 0f32,
            float_one: 1f32,
            float_minus_half: -0.5f32,
            word_idx_one: 1,
            word_idx_max: u32::MAX,
            usize_sanity: 1,
        };
        assert_eq!(SanityHeader::REFERENCE, expected);
    }

    #[test]
    fn test_loads_expected() {
        let mut fd = std::fs::File::open("test_data/sanity.bin").unwrap();
        let from_bytes = SanityHeader::from_file(&mut fd).unwrap();
        let expected = SanityHeader::REFERENCE;
        assert_eq!(from_bytes, expected);
    }
}
