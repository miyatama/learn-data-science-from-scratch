use std::io::Read;

pub fn read(input: Option<&str>) -> anyhow::Result<String> {
    match input {
        Some(path) => std::fs::read_to_string(path)
            .map_err(|e| anyhow::anyhow!("Failed to read file '{}': {}", path, e)),
        None => {
            let mut buf = String::new();
            std::io::stdin()
                .read_to_string(&mut buf)
                .map_err(|e| anyhow::anyhow!("Failed to read stdin: {}", e))?;
            Ok(buf)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_file() {
        let json = read(Some("examples/line.json")).unwrap();
        assert!(json.contains("series"));
    }

    #[test]
    fn test_read_file_not_found() {
        let result = read(Some("nonexistent.json"));
        assert!(result.is_err());
    }
}
