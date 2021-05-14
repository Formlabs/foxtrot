use crate::ap214::Entity;
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
                blocks.push(&data[start..(i + next)]);

                i += next + 1; // Skip the semicolon
                start = i;
            },
            _ => unreachable!(),
        }
    }
    blocks
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
