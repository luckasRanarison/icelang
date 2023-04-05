pub fn is_standard_symbol(ch: char) -> bool {
    let symbols = "+-*/(){}.,;!<>=";
    symbols.contains(ch)
}

pub fn is_quote(ch: char) -> bool {
    ch == '\"' || ch == '\''
}

pub fn is_alphabetic(ch: char) -> bool {
    ch.is_ascii_alphabetic() || ch == '_'
}

pub fn is_alphanumeric(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || ch == '_'
}
