pub(crate) fn take_while(accept: impl Fn(char) -> bool, s:&str) -> (&str, &str) {
    let extracted_end = s
        .char_indices()
        .find_map(|(idx, ch)| if accept(ch) { None } else { Some(idx) })
        .unwrap_or_else(|| s.len());
    let extracted = &s[..extracted_end];
    let remainder = &s[extracted_end..];
    (remainder, extracted)
}

fn take_while_with_error(accept: impl Fn(char) -> bool, s:&str, error_msg: String) -> Result<(&str, &str), String> {
    let (remainder, extracted) = take_while(accept, s);
    if extracted.is_empty() {
        Err(error_msg)
    } else {
        Ok((remainder, extracted))
    }
}

const WHITESPACE: &[char] = &[' ', '\n'];

pub(crate) fn extract_whitespaces(s:&str) -> (&str, &str) {
    take_while(|c| WHITESPACE.contains(&c), s)
}

pub(crate) fn extract_whitespaces_with_error(s:&str) -> Result<(&str, &str), String> {
    take_while_with_error(|c| WHITESPACE.contains(&c), s, "expected a space".to_string())
}

pub(crate) fn extract_digits(s:&str) -> Result<(&str, &str), String> {
    take_while_with_error(|c| c.is_ascii_digit(), s, "expected digits".to_string())
}

pub(crate) fn extract_ident(s:&str) -> Result<(&str, &str), String> {
    let input_starts_with_alphabet = s
        .chars()
        .next()
        .map(|ch| ch.is_ascii_alphabetic())
        .unwrap_or(false);
    if input_starts_with_alphabet {
        Ok(take_while(|ch| ch.is_ascii_alphanumeric(), s))
    } else {
        Err("expected identifier".to_string())
    }
}

pub(crate) fn tag<'a, 'b>(starting_text: &'a str, s: &'b str) -> Result<&'b str, String> {
    if s.starts_with(starting_text) {
        Ok(&s[starting_text.len()..])
    } else {
        Err(format!("expected {}", starting_text))
    }
}

pub(crate) fn sequence<T>(
    parser: impl Fn(&str) -> Result<(&str, T), String>,
    separator_parser: impl Fn(&str) -> (&str, &str),
    mut s: &str,
) -> Result<(&str, Vec<T>), String> {
    let mut items = Vec::new();
    while let Ok((new_s, item)) = parser(s) {
        s = new_s;
        items.push(item);
        let (new_s, _) = separator_parser(s);
        s = new_s;
    }
    Ok((s, items))
}

pub(crate) fn non_empty_sequence<T>(
    parser: impl Fn(&str) -> Result<(&str, T), String>,
    separator_parser: impl Fn(&str) -> (&str, &str),
    s: &str,
) -> Result<(&str, Vec<T>), String> {
    let (s, sequence) = sequence(parser, separator_parser, s)?;
    if sequence.is_empty() {
        Err("expected a sequence with more than one item".to_string())
    } else {
        Ok((s, sequence))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_one_digit() {
        assert_eq!(extract_digits("1+2"), Ok(("+2", "1")))
    }

    #[test]
    fn extract_two_digits() {
        assert_eq!(extract_digits("10+20"), Ok(("+20", "10")))
    }

    #[test]
    fn do_not_extract_digits_when_input_is_invalid() {
        assert_eq!(extract_digits("abcd"), Err("expected digits".to_string()))
    }

    #[test]
    fn extract_newlines_or_spaces() {
        assert_eq!(extract_whitespaces(" \n     \n\nabc"), ("abc", " \n     \n\n"));
    }

    #[test]
    fn do_not_extract_spaces_when_input_does_not_start_with_them() {
        assert_eq!(extract_whitespaces_with_error("blah"), Err("expected a space".to_string()));
    }

    #[test]
    fn extract_digits_with_no_remainder() {
        assert_eq!(extract_digits("100"), Ok(("", "100")))
    }

    #[test]
    fn extract_spaces() {
        assert_eq!(extract_whitespaces("  4"), ("4", "  "))
    }

    #[test]
    fn extract_alphabetic_ident() {
        assert_eq!(extract_ident("asdfasdfWEfzsdf let"), Ok((" let", "asdfasdfWEfzsdf")))
    }

    #[test]
    fn extract_alphanumeric_ident() {
        assert_eq!(extract_ident("var1()"), Ok(("()", "var1")))
    }

    #[test]
    fn cannot_extract_ident_beginning_with_number() {
        assert_eq!(extract_ident("123abc"), Err("expected identifier".to_string()));
    }

    #[test]
    fn tag_word() {
        assert_eq!(tag("let", "let a"), Ok(" a"));
    }
}

