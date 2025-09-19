//! Utility functions and helpers

use crate::error::Result;
use rust_decimal::Decimal;
use std::str::FromStr;

/// Convert string to Decimal safely
pub fn parse_decimal(s: &str) -> Result<Decimal> {
    Decimal::from_str(s)
        .map_err(|e| crate::error::ArbitrageError::calculation(format!("Invalid decimal: {}", e)))
}

/// Calculate percentage difference between two values
pub fn percentage_diff(a: Decimal, b: Decimal) -> Decimal {
    if a.is_zero() {
        return Decimal::ZERO;
    }
    ((b - a) / a) * Decimal::from(100)
}

/// Format decimal as percentage string
pub fn format_percentage(value: Decimal) -> String {
    format!("{:.2}%", value)
}

/// Generate a unique ID
pub fn generate_id() -> String {
    uuid::Uuid::new_v4().to_string()
}

/// Get current timestamp in seconds
pub fn current_timestamp() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_decimal() {
        assert!(parse_decimal("123.45").is_ok());
        assert!(parse_decimal("invalid").is_err());
    }

    #[test]
    fn test_percentage_diff() {
        let a = Decimal::from(100);
        let b = Decimal::from(110);
        let diff = percentage_diff(a, b);
        assert_eq!(diff, Decimal::from(10));
    }

    #[test]
    fn test_generate_id() {
        let id1 = generate_id();
        let id2 = generate_id();
        assert_ne!(id1, id2);
        assert_eq!(id1.len(), 36); // UUID v4 length
    }
}
