pub(crate) fn extract_operator(s:&str) -> (&str, &str) {
    match &s[0..1] {
        "+" | "-" | "*" | "/" => {},
        _ => panic!("Bad Operator: Not implemented yet"),
    }
    (&s[1..], &s[0..1])
}

pub(crate) fn take_while(accept: impl Fn(char) -> bool, s:&str) -> (&str, &str) {
    let extracted_end = s
        .char_indices()
        .find_map(|(idx, ch)| if accept(ch) { None } else { Some(idx) })
        .unwrap_or_else(|| s.len());
    let extracted = &s[..extracted_end];
    let remainder = &s[extracted_end..];
    (remainder, extracted)
}

pub(crate) fn extract_whitespaces(s:&str) -> (&str, &str) {
    take_while(|c| c == ' ', s)
}

pub(crate) fn extract_digits(s:&str) -> (&str, &str) {
    take_while(|c| c.is_ascii_digit(), s)
}

pub(crate) fn extract_ident(s:&str) -> (&str, &str) {
    let input_starts_with_alphabet = s
        .chars()
        .next()
        .map(|ch| ch.is_ascii_alphabetic())
        .unwrap_or(false);
    if input_starts_with_alphabet {
        take_while(|ch| ch.is_ascii_alphanumeric(), s)
    } else {
        (s, "")
    }
}

pub(crate) fn tag<'a, 'b>(starting_text: &'a str, s: &'b str) -> &'b str {
    if s.starts_with(starting_text) {
        &s[starting_text.len()..]
    } else {
        panic!("Expected {}", starting_text);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_one_digit() {
        assert_eq!(extract_digits("1+2"), ("+2", "1"))
    }

    #[test]
    fn extract_two_digits() {
        assert_eq!(extract_digits("10+20"), ("+20", "10"))
    }

    #[test]
    fn do_not_extract_anything_from_empty_input() {
        assert_eq!(extract_digits(""), ("", ""));
    }

    #[test]
    fn extract_digits_with_no_remainder() {
        assert_eq!(extract_digits("100"), ("", "100"));
    }

    #[test]
    fn extract_plus() {
        assert_eq!(extract_operator("+2"), ("2", "+"));
    }

    #[test]
    fn extract_minus() {
        assert_eq!(extract_operator("-10"), ("10", "-"));
    }

    #[test]
    fn extract_star() {
        assert_eq!(extract_operator("*3"), ("3", "*"));
    }

    #[test]
    fn extract_slash() {
        assert_eq!(extract_operator("/4"), ("4", "/"));
    }

    #[test]
    fn extract_spaces() {
        assert_eq!(extract_whitespaces("  4"), ("4", "  "));
    }

    #[test]
    fn extract_alphabetic_ident() {
        assert_eq!(extract_ident("asdfasdfWEfzsdf let"), (" let", "asdfasdfWEfzsdf"));
    }

    #[test]
    fn extract_alphanumeric_ident() {
        assert_eq!(extract_ident("var1()"), ("()", "var1"));
    }

    #[test]
    fn cannot_extract_ident_beginning_with_number() {
        assert_eq!(extract_ident("123abc"), ("123abc", ""));
    }

    #[test]
    fn tag_word() {
        assert_eq!(tag("let", "let a"), " a");
    }
}

