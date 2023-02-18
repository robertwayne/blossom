use std::iter::zip;

use crate::constants::{HTTP_METHODS, INVALID_HTTP_VERSIONS};

/// Capitalizes the first letter of a string.
pub fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

/// Guard function for detecting HTTP traffic on the telnet stream. Checks if
/// the first line of the stream is a valid HTTP method or contains an HTTP
/// version, and if it is, drops it silently.
pub fn is_http(message: &str) -> bool {
    zip(HTTP_METHODS, INVALID_HTTP_VERSIONS)
        .any(|(method, version)| message.starts_with(method) || message.ends_with(version))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn capitalize_string() {
        assert_eq!(capitalize("blossom"), "Blossom");
    }

    #[test]
    fn capitalize_mixed_case() {
        assert_eq!(capitalize("blossom is cool"), "Blossom is cool");
    }

    #[test]
    fn is_http_true() {
        assert!(is_http("GET / HTTP/1.1"));
        assert!(is_http("POST / HTTP/1.1"));
        assert!(is_http("PUT / HTTP/1.1"));
        assert!(is_http("DELETE / HTTP/1.1"));
        assert!(is_http("HEAD / HTTP/1.1"));
        assert!(is_http("OPTIONS / HTTP/1.1"));
        assert!(is_http("CONNECT / HTTP/1.1"));
        assert!(is_http("TRACE / HTTP/1.1"));
        assert!(is_http("PATCH / HTTP/1.1"));
    }
}
