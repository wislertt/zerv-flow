pub fn get_server_address() -> String {
    let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    format!("{}:{}", host, port)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_server_address_default() {
        // Test with default environment
        let addr = get_server_address();

        // Should return default or whatever is in env
        // We don't assert exact value since it depends on test environment
        assert!(!addr.is_empty());
        assert!(addr.contains(':')); // Should have port separator
    }

    #[test]
    fn test_get_server_address_structure() {
        // Test that the function returns a properly formatted address
        let addr = get_server_address();

        let parts: Vec<&str> = addr.split(':').collect();
        assert_eq!(parts.len(), 2); // Should have host and port

        // Port should be a number
        assert!(parts[1].parse::<u16>().is_ok());

        // Host should be a valid IP or hostname
        assert!(!parts[0].is_empty());
    }
}
