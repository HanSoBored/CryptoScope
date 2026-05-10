//! Screener output formatting utilities.
//!
//! Provides filtering, sorting, and formatting functions for price change data.
//! Note: Some formatting functions are deprecated as CLI formatting is no longer used.

use crate::core::models::PriceChange;

/// Compute count and average for a group matching a predicate.
/// Uses a single-pass fold to avoid creating an intermediate Vec.
fn compute_group_stats<F>(changes: &[PriceChange], predicate: F) -> (usize, f64)
where
    F: Fn(&PriceChange) -> bool,
{
    let (count, sum) = changes.iter().fold((0, 0.0), |(count, sum), c| {
        if predicate(c) {
            (count + 1, sum + c.change_percent)
        } else {
            (count, sum)
        }
    });
    let avg = if count == 0 { 0.0 } else { sum / count as f64 };
    (count, avg)
}

/// Display summary statistics for price changes.
/// Returns a formatted string with statistics.
#[deprecated(since = "0.2.0", note = "CLI formatting is no longer used. Use API JSON responses instead.")]
#[allow(dead_code)]
pub fn format_stats(changes: &[PriceChange]) -> String {
    let total = changes.len();

    // Gainers
    let (gainer_count, gainer_avg) = compute_group_stats(changes, |c| c.change_percent > 0.0);

    // Losers
    let (loser_count, loser_avg) = compute_group_stats(changes, |c| c.change_percent < 0.0);

    // Unchanged
    let (unchanged_count, _) = compute_group_stats(changes, |c| c.change_percent == 0.0);

    format!(
        "Stats:\n  Total: {} symbols\n  Gainers: {} (avg {:.2}%)\n  Losers: {} (avg {:.2}%)\n  Unchanged: {}",
        total, gainer_count, gainer_avg, loser_count, loser_avg, unchanged_count
    )
}

/// Apply all filters, sort, and truncate to the result set.
pub fn apply_filters(
    changes: &[PriceChange],
    top_n: Option<usize>,
    min_change: Option<f64>,
    min_volume: Option<f64>,
    symbol_filter: Option<&str>,
) -> Vec<PriceChange> {
    let mut filtered: Vec<PriceChange> = changes.to_vec();

    if let Some(min) = min_change {
        filtered.retain(|c| c.change_percent.abs() >= min);
    }

    if let Some(min_vol) = min_volume {
        // Volume filter uses category-aware calculation:
        // - Perpetuals (linear/inverse): volume is in contracts, multiply by price for USDT
        // - Spot: volume is already in quote currency (USDT), use directly
        filtered.retain(|c| c.volume_usdt() >= min_vol);
    }

    if let Some(symbol) = symbol_filter {
        filtered.retain(|c| c.symbol.to_uppercase().contains(&symbol.to_uppercase()));
    }

    filtered.sort_by(|a, b| {
        b.change_percent
            .abs()
            .partial_cmp(&a.change_percent.abs())
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    if let Some(n) = top_n {
        filtered.truncate(n);
    }

    filtered
}

/// Format price with dynamic precision based on value.
///
/// - < 0.01: 6 decimals
/// - < 1.0: 5 decimals
/// - < 100: 4 decimals
/// - >= 100: 2 decimals
///
/// WHY: Crypto prices span many orders of magnitude — from fractions of a cent
/// (e.g., SHIB at 0.000012) to tens of thousands (e.g., BTC at 60000). A fixed
/// precision would either waste columns on large prices or lose meaningful digits
/// on small ones. These tiers balance readability with column-width constraints.
///
/// Negative prices are formatted with a leading minus sign.
#[deprecated(since = "0.2.0", note = "CLI formatting is no longer used. Use API JSON responses instead.")]
#[allow(dead_code)]
pub fn format_price(price: f64) -> String {
    let sign = if price < 0.0 { "-" } else { "" };
    let abs = price.abs();
    let formatted = if abs < 0.01 {
        format!("{:.6}", abs)
    } else if abs < 1.0 {
        format!("{:.5}", abs)
    } else if abs < 100.0 {
        format!("{:.4}", abs)
    } else {
        format!("{:.2}", abs)
    };
    format!("{sign}{formatted}")
}

/// Format volume with K/M/B suffixes for readability.
///
/// - >= 1B: `$X.XXB`
/// - >= 1M: `$X.XXM`
/// - >= 1K: `$X.XXK`
/// - < 1K: `$X.XX`
#[deprecated(since = "0.2.0", note = "CLI formatting is no longer used. Use API JSON responses instead.")]
#[allow(dead_code)]
pub fn format_volume(volume: f64) -> String {
    if volume >= 1_000_000_000.0 {
        format!("${:.2}B", volume / 1_000_000_000.0)
    } else if volume >= 1_000_000.0 {
        let m_val = volume / 1_000_000.0;
        // If rounding would produce >= 1000.00M, bump to billions
        if m_val >= 999.995 {
            format!("${:.2}B", volume / 1_000_000_000.0)
        } else {
            format!("${:.2}M", m_val)
        }
    } else if volume >= 1_000.0 {
        let k_val = volume / 1_000.0;
        // If rounding would produce >= 1000.00K, bump to millions
        if k_val >= 999.995 {
            format!("${:.2}M", volume / 1_000_000.0)
        } else {
            format!("${:.2}K", k_val)
        }
    } else {
        format!("${:.2}", volume)
    }
}

#[cfg(test)]
#[allow(deprecated)]
mod tests {
    use super::*;

    #[test]
    fn test_format_price_very_small() {
        assert_eq!(format_price(0.000012), "0.000012");
        assert_eq!(format_price(0.009999), "0.009999");
    }

    #[test]
    fn test_format_price_small() {
        assert_eq!(format_price(0.01), "0.01000");
        assert_eq!(format_price(0.5), "0.50000");
        assert_eq!(format_price(0.99999), "0.99999");
    }

    #[test]
    fn test_format_price_medium() {
        assert_eq!(format_price(1.0), "1.0000");
        assert_eq!(format_price(50.12345), "50.1234"); // banker's rounding
        assert_eq!(format_price(99.99), "99.9900");
    }

    #[test]
    fn test_format_price_large() {
        assert_eq!(format_price(100.0), "100.00");
        assert_eq!(format_price(50000.5), "50000.50");
        assert_eq!(format_price(123456.789), "123456.79");
    }

    #[test]
    fn test_format_volume_billions() {
        assert_eq!(format_volume(1_500_000_000.0), "$1.50B");
        // Boundary: values that would round to 1000.00M should bump to B
        assert_eq!(format_volume(999_999_999.99), "$1.00B");
    }

    #[test]
    fn test_format_volume_millions() {
        assert_eq!(format_volume(1_500_000.0), "$1.50M");
        // Boundary: values that would round to 1000.00K should bump to M
        assert_eq!(format_volume(999_999.99), "$1.00M");
    }

    #[test]
    fn test_format_volume_thousands() {
        assert_eq!(format_volume(1_500.0), "$1.50K");
        assert_eq!(format_volume(999.99), "$999.99");
    }

    #[test]
    fn test_format_volume_small() {
        assert_eq!(format_volume(0.0), "$0.00");
        assert_eq!(format_volume(42.5), "$42.50");
        assert_eq!(format_volume(999.99), "$999.99");
    }
}
