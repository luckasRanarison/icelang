pub fn is_standard_symbol(ch: char) -> bool {
    let symbols = "+-*/(){}.,;!<>=";
    symbols.contains(ch)
}
