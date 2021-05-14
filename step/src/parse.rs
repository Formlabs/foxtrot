/// Flattens a STEP file into a set of slices
pub fn flatten(data: &[u8]) -> Vec<&[u8]> {
    let mut blocks = Vec::new();
    let mut comment_start = None;
    let mut in_string = false;
    let mut start = 0;
    for (i, &c) in data.iter().enumerate() {
        match c as char {
            ' '|'\n' => if i == start {
                start += 1;
            },
            ';' => if !in_string && comment_start.is_none() {
                blocks.push(&data[start..i]);
                start = i + 1;
            },
            '/' => if comment_start.is_some() && !in_string && i > 0 && data[i - 1] == '*' as u8 {
                if start == comment_start.unwrap() {
                    start = i + 1;
                }
                comment_start = None;
            },
            '*' => if !in_string && i > 0 && data[i - 1] == '/' as u8 {
                comment_start = Some(i - 1);
            },
            '\'' => if comment_start.is_none() {
                in_string = !in_string;
            },
            _ => (),
        }
    }
    blocks
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flatten() {
        let s = r#"
/* test comment please ignore */
DATA;
#10=PROPERTY_DEFINITION_REPRESENTATION(#14,#12);
#11=PROPERTY_DEFINITION_REPRESENTATION(#15,#13);
"#;
        let out = flatten(s.as_bytes());
        let s = |i| std::str::from_utf8(out[i]).unwrap();
        assert_eq!(s(0), "DATA");
        assert_eq!(s(1), "#10=PROPERTY_DEFINITION_REPRESENTATION(#14,#12)");
        assert_eq!(s(2), "#11=PROPERTY_DEFINITION_REPRESENTATION(#15,#13)");
    }
}
