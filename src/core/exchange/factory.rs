use super::bybit::{BybitClient, EXCHANGE_NAME};
use super::exchange_trait::Exchange;
use crate::core::error::{CryptoScopeError, Result};

/// Create an exchange client by name
pub fn create_exchange(name: &str) -> Result<Box<dyn Exchange>> {
    match name.to_lowercase().as_str() {
        "bybit" => Ok(Box::new(BybitClient::new()?)),
        _ => Err(CryptoScopeError::UnknownExchange(name.to_string())),
    }
}

/// Get list of supported exchanges
#[allow(dead_code)]
pub fn get_supported_exchanges() -> &'static [&'static str] {
    &[EXCHANGE_NAME]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_bybit_exchange() {
        let exchange = create_exchange("bybit");
        assert!(exchange.is_ok());
        assert_eq!(exchange.unwrap().name(), "bybit");
    }

    #[test]
    fn test_create_unknown_exchange() {
        let exchange = create_exchange("unknown");
        assert!(exchange.is_err());
    }

    #[test]
    fn test_case_insensitive() {
        let exchange = create_exchange("BYBIT");
        assert!(exchange.is_ok());
    }
}
