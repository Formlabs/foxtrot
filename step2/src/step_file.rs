use memchr::{memchr, memchr2, memchr_iter};
use rayon::prelude::*;

use crate::{
    ap214::Entity,
    parse::parse_entity_decl,
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

        Self(blocks[data_start..data_end]
            .par_iter()
            .map(|b| parse_entity_decl(*b).unwrap().1.1)
            .collect())
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
}
