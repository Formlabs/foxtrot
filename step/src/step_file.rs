use memchr::{memchr, memchr2, memchr_iter};
use log::warn;

#[cfg(feature = "rayon")]
use rayon::prelude::*;

use crate::{
    ap214::Entity,
    id::Id,
    parse::{parse_entity_decl, parse_entity_fallback},
};

#[derive(Debug)]
pub struct StepFile<'a>(pub Vec<Entity<'a>>);
impl<'a> StepFile<'a> {
    /// Parses a STEP file from a raw array of bytes
    /// `data` must be preprocessed by [`strip_flatten`] first
    pub fn parse(data: &'a [u8]) -> Self {
        let blocks = Self::into_blocks(&data);
        let data_start = blocks.iter()
            .position(|b| b == b"DATA;")
            .unwrap_or(0) + 1;
        let data_end = blocks.iter()
            .skip(data_start)
            .position(|b| b == b"ENDSEC;")
            .unwrap_or(0) + data_start;

        // Parse every block, accumulating a Vec of Results.  We parse in
        // single-threaded mode in WASM builds, because there's no thread
        // pool.
        let block_iter = {
            let block_slice = &blocks[data_start..data_end];
            #[cfg(feature = "rayon")]
            { block_slice.par_iter() }
            #[cfg(not(feature = "rayon"))]
            { block_slice.iter() }
        };

        let parsed: Vec<(usize, Entity)> = block_iter
            .filter_map(|b| parse_entity_decl(*b)
                .or_else(|e| {
                    warn!("Failed to parse {}: {:?}",
                        std::str::from_utf8(b).unwrap_or("[INVALID UTF-8]"),
                              e);
                    parse_entity_fallback(*b)
                })
                .ok())
            .map(|b| b.1)
            .collect();

        // Awkward construction because `Entity` is not `Clone`
        let max_id = parsed.iter().map(|b| b.0).max().unwrap_or(0);
        let mut out: Vec<Entity> = (0..=max_id)
            .map(|_| Entity::_EmptySlot)
            .collect();

        for p in parsed.into_iter() {
            out[p.0] = p.1;
        }

        Self(out)
    }

    /// Flattens a STEP file, removing comments and whitespace
    pub fn strip_flatten(data: &[u8]) -> Vec<u8> {
        let mut out = Vec::with_capacity(data.len());
        let mut i = 0;
        while i < data.len() {
            match data[i] {
                b'/' => if i + 1 < data.len() && data[i + 1] == b'*' {
                    for j in memchr_iter(b'/', &data[i + 2..]) {
                        if data[i + j + 1] == b'*' {
                            i += j + 2;
                            break;
                        }
                    }
                }
                // TODO: don't skip whitespace inside of strings
                c if c.is_ascii_whitespace() => (),
                c => out.push(c),
            }
            i += 1;
        }
        out
    }

    /// Splits a STEP file into individual blocks.  The input must be pre-processed
    /// by [`strip_flatten`] beforehand.
    fn into_blocks(data: &[u8]) -> Vec<&[u8]> {
        let mut blocks = Vec::new();
        let mut i = 0;
        let mut start = 0;
        while i < data.len() {
            let next = memchr2(b'\'', b';', &data[i..]).unwrap();
            match data[i + next] {
                // Skip over quoted blocks
                b'\'' => i += next + memchr(b'\'', &data[i + next..]).unwrap() + 1,
                b';' => {
                    blocks.push(&data[start..=(i + next)]);

                    i += next + 1; // Skip the semicolon
                    start = i;
                },
                _ => unreachable!(),
            }
        }
        blocks
    }

    pub fn entity<T: FromEntity<'a>>(&'a self, i: Id<T>) -> Option<&'a T> {
        T::try_from_entity(&self.0[i.0])
    }
}

impl<'a, T> std::ops::Index<Id<T>> for StepFile<'a> {
    type Output = Entity<'a>;

    fn index(&self, id: Id<T>) -> &Self::Output {
        &self.0[id.0]
    }
}

pub trait FromEntity<'a> {
    fn try_from_entity(e: &'a Entity<'a>) -> Option<&'a Self>;
}
