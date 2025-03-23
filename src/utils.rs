pub fn split_sentences(text: &str) -> Vec<String> {
    text.split(&['.', '!', '?'])
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.trim().to_string() + ".")
        .collect()
}
