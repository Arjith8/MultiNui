pub fn add_padding(padding: usize, vals: Vec<String>) -> Vec<String> {
    let padding = " ".repeat(padding);
    vals.into_iter().map(|s| format!("{}{}{}", padding, s, padding)).collect()
}
