#![cfg_attr(not(feature = "std"), no_std)]
#![feature(array_chunks, iter_array_chunks, int_roundings)]

const INPUT_CHUNK_SIZE: usize = 4;

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
struct Record {
    len: u32,
    payload: [u8; INPUT_CHUNK_SIZE],
}

impl Record {
    #[inline]
    pub fn new(len: u32, payload: &[u8]) -> Self {
        let mut s = Self {
            len,
            payload: Default::default(),
        };
        s.payload.copy_from_slice(payload);
        s
    }
}

const RECORD_SIZE: usize = core::mem::size_of::<Record>();

/// Transmutes a record into a series of bytes.
#[inline]
fn transmute_to_bytes_copy(x: &Record) -> [u8; RECORD_SIZE] {
    unsafe { core::mem::transmute_copy(x) }
}

/// SAFETY: Since an arbitrary byte slice is not guaranteed to have the precise layout required for
/// a `Record` type to be transmutable from, this must be unsafe.
#[inline]
unsafe fn transmute_from_bytes_copy(x: &[u8]) -> &Record {
    if x.len() != core::mem::size_of::<Record>() {
        panic!("Byte slice must be the same length as a Record")
    }

    unsafe { &*(x.as_ptr() as *const _) }
}

const ROT: usize = 7;

pub fn obfuscate_iter(bytes: &[u8]) -> impl Iterator<Item = u8> + '_ {
    let num_records = bytes.len().div_floor(INPUT_CHUNK_SIZE);
    let chunks = bytes.array_chunks::<INPUT_CHUNK_SIZE>();

    let remainder = chunks.remainder();
    let shuffled_chunks = chunks.cycle().skip(ROT).take(num_records);

    let divisble_parts = shuffled_chunks
        .enumerate()
        .map(|(i, c)| Record::new(i as _, c))
        .flat_map(|r| transmute_to_bytes_copy(&r));

    divisble_parts.chain(remainder.iter().copied())
}

pub fn deobfuscate_iter(bytes: &[u8]) -> impl Iterator<Item = u8> + '_ {
    let num_records = bytes.len().div_floor(RECORD_SIZE);
    let num_chunks = bytes
        .len()
        .saturating_sub(4 * num_records)
        .div_floor(INPUT_CHUNK_SIZE);

    let chunks = bytes.array_chunks::<RECORD_SIZE>();
    chunks
        .clone()
        .map(|c| unsafe { transmute_from_bytes_copy(c.as_slice()) })
        .flat_map(|r| r.payload)
        .array_chunks::<INPUT_CHUNK_SIZE>()
        .cycle()
        .skip(num_chunks - ROT) // unshuffle the chunks
        .take(num_chunks)
        .flatten()
        .chain(chunks.remainder().iter().copied())
}

#[test]
#[cfg(test)]
#[cfg(feature = "std")]
fn test_obfuscation() -> anyhow::Result<()> {
    let original_bytes =
        b"Hello there it is me bytes johnson and I'm here to take your cheese away";

    let obfuscated: Vec<_> = obfuscate_iter(&original_bytes[..]).collect();
    let deobfuscated: Vec<_> = deobfuscate_iter(&obfuscated).collect();

    assert_eq!(deobfuscated, original_bytes);

    Ok(())
}
