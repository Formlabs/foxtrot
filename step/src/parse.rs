use std::fs::File;
use std::io::Read;
use std::str;

use rayon::prelude::*;

use memchr::{memchr, memchr2, memchr_iter};

use crate::ap214::StepFile;
use crate::ap214_autogen::DataEntity;
use crate::parse_autogen::data_line;

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

pub fn into_blocks(data: &[u8]) -> Vec<&[u8]> {
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

pub fn parse_entities_from_striped_file(stripped_file: &Vec<u8>) -> StepFile {
    let blocks = into_blocks(&stripped_file);

    let mut data_sec_idx = 0;
    let mut end_sec_idx = blocks.len();
    for (i, block) in blocks.iter().enumerate() {
        let line_string = str::from_utf8(block).expect("ok utf8 str");
        if line_string == "DATA;" {
            data_sec_idx = i;
            break;
        }
    }
    for (i, block) in blocks.iter().enumerate().rev() {
        let line_string = str::from_utf8(block).expect("ok utf8 str");
        if line_string == "ENDSEC;" {
            end_sec_idx = i;
            break;
        }
    }

    let id_entity_pairs: Vec<_> = blocks[(data_sec_idx+1)..end_sec_idx].par_iter().map(|&block| {
        let line_string = str::from_utf8(block).expect("ok utf8 str");
        let (_rest_block, (id, entity)) = data_line(line_string).expect("ok parse");
        (id, entity)
    }).collect();

    let max_idx = id_entity_pairs.iter().max_by_key(|p| p.0.0).unwrap().0.0;

    let mut entities: Vec<DataEntity> = Vec::new();
    entities.resize_with(max_idx+1, || DataEntity::Null);
    for (id, entity) in id_entity_pairs {
        entities[id.0] = entity
    }

    StepFile(entities)
}

pub fn striped_string_from_path(filename: &str) -> Vec<u8> {
    let mut f = File::open(filename).expect("file opens");
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer).expect("read ok");
    strip_flatten(&buffer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strip() {
        let s = "\n/* test comment please ignore */\nDATA;\n#10=PROPERTY_DEFINITION_REPRESENTATION(/* another comment*/ #14, #12);\n#11=PROPERTY_DEFINITION_REPRESENTATION(#15,#13);\n";
        let out = strip_flatten(s.as_bytes());
        assert_eq!(std::str::from_utf8(&out).unwrap(),
        "DATA;#10=PROPERTY_DEFINITION_REPRESENTATION(#14,#12);#11=PROPERTY_DEFINITION_REPRESENTATION(#15,#13);");
    }

    #[test]
    fn test_flatten() {
        let s = "\n/* test comment please ignore */\nDATA;\n#10=PROPERTY_DEFINITION_REPRESENTATION(#14,#12);\n#11=PROPERTY_DEFINITION_REPRESENTATION(#15,#13);\n";
        let stripped = strip_flatten(s.as_bytes());
        let out = into_blocks(&stripped);
        let s = |i| std::str::from_utf8(out[i]).unwrap();
        assert_eq!(s(0), "DATA;");
        assert_eq!(s(1), "#10=PROPERTY_DEFINITION_REPRESENTATION(#14,#12);");
        assert_eq!(s(2), "#11=PROPERTY_DEFINITION_REPRESENTATION(#15,#13);");
    }
}
