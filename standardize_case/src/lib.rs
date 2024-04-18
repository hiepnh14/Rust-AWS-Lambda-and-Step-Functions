pub fn process_text(input: &str, mode: &str) -> Result<String, &'static str> {
    match mode {
        "upper" => Ok(input.to_uppercase()),
        "lower" => Ok(input.to_lowercase()),
        _ => Err("Invalid mode"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_text_upper() {
        let result = process_text("hello", "upper").unwrap();
        assert_eq!(result, "HELLO");
    }

    #[test]
    fn test_process_text_lower() {
        let result = process_text("HELLO", "lower").unwrap();
        assert_eq!(result, "hello");
    }

    #[test]
    fn test_process_text_invalid_mode() {
        let result = process_text("hello", "invalid");
        assert!(result.is_err());
    }
}
