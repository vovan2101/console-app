pub fn parse_command(command: &str) -> Result<Vec<String>, String> {
    let mut parts = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;

    for c in command.chars() {
        match c {
            '"' => {
                in_quotes = !in_quotes;
                if !in_quotes {
                    parts.push(current.clone());
                    current.clear();
                }
            }
            ' ' if !in_quotes => {
                if !current.is_empty() {
                    parts.push(current.clone());
                    current.clear();
                }
            }
            _=> current.push(c),
        }
    }
    if !current.is_empty() {
        parts.push(current);
    }

    if in_quotes {
        return Err("Unmatched quotes in command.".to_string());
    }

    Ok(parts)
}
