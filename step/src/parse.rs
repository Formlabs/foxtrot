use std::fs::File;
use std::io::Read;
use std::str;
use std::cmp::max;

use crate::parse_autogen::{data_line, DataEntity};
use memchr::{memchr, memchr2, memchr_iter};


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


pub fn parse_file_as_string(file: &Vec<u8>) -> Vec<DataEntity> {
    let stripped = strip_flatten(&file);
    let blocks = into_blocks(&stripped);

    let mut entities : Vec<DataEntity> = Vec::new();
    let mut max_idx = 0;
    let mut started = false;
    for block in blocks {
        let st = str::from_utf8(block).expect("ok utf8 str");
        if !started {
            if st == "DATA;" {
                started = true;
            }
            continue;
        }
        if st == "ENDSEC;" {
            break
        }
        let (_rest_block, (id, entity)) = data_line(st).expect("ok parse");
        max_idx = max(max_idx, id.0);
        if id.0 >= entities.len() {
            entities.resize_with(max_idx * 3 / 2 + 1, || { DataEntity::Null });
        }
        entities[id.0] = entity;
    }
    entities.resize_with(max_idx, || { DataEntity::Null });
    entities
}

pub fn parse_file_at_path(filename: &str) -> Vec<DataEntity> {
    let mut f = File::open(filename).expect("file opens");
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer).expect("read ok");
    parse_file_as_string(&buffer)
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strip() {
        let s = r#"
/* test comment please ignore */
DATA;
#10=PROPERTY_DEFINITION_REPRESENTATION(/* another comment*/ #14, #12);
#11=PROPERTY_DEFINITION_REPRESENTATION(#15,#13);
"#;
        let out = strip_flatten(s.as_bytes());
        assert_eq!(std::str::from_utf8(&out).unwrap(),
        "DATA;#10=PROPERTY_DEFINITION_REPRESENTATION(#14,#12);#11=PROPERTY_DEFINITION_REPRESENTATION(#15,#13);");
    }

    #[test]
    fn test_flatten() {
        let s = r#"
/* test comment please ignore */
DATA;
#10=PROPERTY_DEFINITION_REPRESENTATION(#14,#12);
#11=PROPERTY_DEFINITION_REPRESENTATION(#15,#13);
"#;
        let stripped = strip_flatten(s.as_bytes());
        let out = into_blocks(&stripped);
        let s = |i| std::str::from_utf8(out[i]).unwrap();
        assert_eq!(s(0), "DATA");
        assert_eq!(s(1), "#10=PROPERTY_DEFINITION_REPRESENTATION(#14,#12)");
        assert_eq!(s(2), "#11=PROPERTY_DEFINITION_REPRESENTATION(#15,#13)");
    }
}
