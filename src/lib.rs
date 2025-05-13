pub fn run(str: String, target: &str) -> anyhow::Result<String> {
    let mut result: String = String::new();
    let sep_str: Vec<&str> = str.split('\n').collect(); // Separated String at '\n'
    
    for s in sep_str {
        if s.contains(target) {
            result.push_str(s);
            result.push('\n');
        }
    }

    Ok(result)
}
