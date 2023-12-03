pub fn is_valid_ident(string: &String) -> bool {
    string.chars().all(|c| c.is_alphanumeric() || c == '_')
}
