/// This is used for matching against HTTP traffic on the telnet stream.
pub const HTTP_METHODS: &[&str] = &[
    "GET", "HEAD", "POST", "PUT", "DELETE", "CONNECT", "OPTIONS", "TRACE", "PATCH",
];

/// This is used for matching against HTTP traffic on the telnet stream. Telnet
/// is only accessible with HTTP/0.9.
pub const INVALID_HTTP_VERSIONS: &[&str] = &["HTTP/1.0", "HTTP/1.1", "HTTP/2.0"];
