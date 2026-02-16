use std::collections::HashMap;
use std::fs;
use std::{env, process};

use thiserror::Error;

// ---------------------------------------------------------------------------
// Error types
// ---------------------------------------------------------------------------

/// Every fallible operation in this crate returns `Result<T, AppError>`.
///
/// Variants carry enough context to produce a useful message *and* let callers
/// match on the specific failure kind.
#[derive(Debug, Error)]
pub enum AppError {
    /// Wraps `std::io::Error` — file not found, permission denied, etc.
    /// The `#[from]` attribute generates `From<std::io::Error> for AppError`,
    /// which is what lets the `?` operator convert automatically.
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    /// A line in the config file could not be parsed as `key=value`.
    #[error("parse error: {0}")]
    ParseError(String),

    /// A value was present but invalid (e.g. port out of range).
    #[error("validation error in field `{field}`: {message}")]
    ValidationError { field: String, message: String },
}

// ---------------------------------------------------------------------------
// Config
// ---------------------------------------------------------------------------

/// A parsed and validated configuration.
#[derive(Debug, PartialEq)]
pub struct Config {
    pub host: String,
    pub port: u16,
}

// ---------------------------------------------------------------------------
// Parsing & validation
// ---------------------------------------------------------------------------

/// Reads a `key=value` config file and returns a validated [`Config`].
///
/// The `?` operator propagates errors from I/O, parsing, and validation
/// through the single `AppError` type — no manual match-and-return needed.
pub fn read_config(path: &str) -> Result<Config, AppError> {
    let contents = fs::read_to_string(path)?; // IoError via #[from]

    let map = parse_kv(&contents)?;

    let host = map
        .get("host")
        .ok_or_else(|| AppError::ValidationError {
            field: "host".into(),
            message: "missing required field".into(),
        })?
        .to_string();

    let port_str = map.get("port").ok_or_else(|| AppError::ValidationError {
        field: "port".into(),
        message: "missing required field".into(),
    })?;

    let port = validate_port(port_str)?;

    Ok(Config { host, port })
}

/// Parses `key=value` lines into a map. Blank lines and `#` comments are
/// skipped. Lines without `=` produce a [`AppError::ParseError`].
fn parse_kv(input: &str) -> Result<HashMap<&str, &str>, AppError> {
    let mut map = HashMap::new();
    for line in input.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let (key, value) = trimmed
            .split_once('=')
            .ok_or_else(|| AppError::ParseError(format!("expected `key=value`, got: {trimmed}")))?;
        map.insert(key.trim(), value.trim());
    }
    Ok(map)
}

/// Validates that a string is a valid port number (1–65535).
pub fn validate_port(s: &str) -> Result<u16, AppError> {
    let n: u16 = s.parse().map_err(|_| AppError::ValidationError {
        field: "port".into(),
        message: format!("`{s}` is not a valid port number"),
    })?;

    if n == 0 {
        return Err(AppError::ValidationError {
            field: "port".into(),
            message: "port must be between 1 and 65535".into(),
        });
    }

    Ok(n)
}

// ---------------------------------------------------------------------------
// main
// ---------------------------------------------------------------------------

fn main() {
    let path = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("usage: error_demo <config-file>");
        process::exit(2);
    });

    match read_config(&path) {
        Ok(config) => println!("Loaded config: {config:?}"),
        Err(e) => {
            eprintln!("error: {e}");
            process::exit(1);
        }
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    /// Helper: write content to a temporary file and return its path.
    fn temp_config(content: &str) -> tempfile::NamedTempFile {
        let mut f = tempfile::NamedTempFile::new().unwrap();
        f.write_all(content.as_bytes()).unwrap();
        f
    }

    #[test]
    fn happy_path_valid_config() {
        let f = temp_config("host=localhost\nport=8080\n");
        let cfg = read_config(f.path().to_str().unwrap()).unwrap();
        assert_eq!(
            cfg,
            Config {
                host: "localhost".into(),
                port: 8080
            }
        );
    }

    #[test]
    fn comments_and_blank_lines_are_skipped() {
        let f = temp_config("# a comment\n\nhost = 0.0.0.0\nport = 443\n");
        let cfg = read_config(f.path().to_str().unwrap()).unwrap();
        assert_eq!(cfg.host, "0.0.0.0");
        assert_eq!(cfg.port, 443);
    }

    #[test]
    fn file_not_found_propagates_as_io_error() {
        let result = read_config("/tmp/definitely_does_not_exist_12345.conf");
        assert!(matches!(result, Err(AppError::IoError(_))));
    }

    #[test]
    fn invalid_port_is_validation_error() {
        let f = temp_config("host=localhost\nport=notanumber\n");
        let result = read_config(f.path().to_str().unwrap());
        assert!(matches!(
            result,
            Err(AppError::ValidationError { ref field, .. }) if field == "port"
        ));
    }

    #[test]
    fn port_zero_is_rejected() {
        let f = temp_config("host=localhost\nport=0\n");
        let result = read_config(f.path().to_str().unwrap());
        assert!(matches!(result, Err(AppError::ValidationError { .. })));
    }

    #[test]
    fn missing_host_is_validation_error() {
        let f = temp_config("port=8080\n");
        let result = read_config(f.path().to_str().unwrap());
        assert!(matches!(
            result,
            Err(AppError::ValidationError { ref field, .. }) if field == "host"
        ));
    }

    #[test]
    fn missing_port_is_validation_error() {
        let f = temp_config("host=localhost\n");
        let result = read_config(f.path().to_str().unwrap());
        assert!(matches!(
            result,
            Err(AppError::ValidationError { ref field, .. }) if field == "port"
        ));
    }

    #[test]
    fn malformed_line_is_parse_error() {
        let f = temp_config("host=localhost\nthis has no equals sign\n");
        let result = read_config(f.path().to_str().unwrap());
        assert!(matches!(result, Err(AppError::ParseError(_))));
    }

    #[test]
    fn validate_port_accepts_valid() {
        assert_eq!(validate_port("443").unwrap(), 443);
        assert_eq!(validate_port("1").unwrap(), 1);
        assert_eq!(validate_port("65535").unwrap(), 65535);
    }

    #[test]
    fn validate_port_rejects_out_of_range() {
        // u16 parse will fail for numbers > 65535
        assert!(validate_port("70000").is_err());
        assert!(validate_port("-1").is_err());
    }
}
