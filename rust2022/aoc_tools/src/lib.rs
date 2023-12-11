pub fn trim_prefix<'a>(prefix: &'a str, string: &'a str) -> &'a str {
    match string.strip_prefix(prefix) {
        Some(x) => x,
        None => string,
    }
}
