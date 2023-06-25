pub fn trim_spaces(s: &str) -> &str {
    let mut begin_index = 0;
    let mut end_index = 0;

    let string_bytes = s.as_bytes();

    for (index, &item) in string_bytes.iter().enumerate() {
        if item != b' ' {
            begin_index = index;
            break;
        }
    }

    for (index, &item) in string_bytes.iter().enumerate().rev() {
        if item != b' ' {
            end_index = index;
            break;
        }
    }

    return &s[begin_index..end_index];
}