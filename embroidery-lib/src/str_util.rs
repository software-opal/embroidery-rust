use unicode_segmentation::UnicodeSegmentation;

pub fn char_truncate(s: &str, cap: usize) -> String {
    let mut trunc = String::with_capacity(cap);
    for c in UnicodeSegmentation::graphemes(s, true) {
        if trunc.len() + c.len() <= cap {
            trunc.push_str(c);
        } else {
            break;
        }
    }
    trunc
}

pub fn c_trim(s: &str) -> String {
    if let Some(idx) = s.find('\0') {
        s[..idx].trim().to_string()
    } else {
        s.trim().to_string()
    }
}
