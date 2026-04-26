use colored::Colorize;

use crate::models::PriceChange;
use crate::utils::terminal_width;

/// Colorize text based on value sign (green=positive, red=negative, neutral=zero).
fn colorize(value: f64, text: String) -> String {
    if value > 0.0 {
        text.green().to_string()
    } else if value < 0.0 {
        text.red().to_string()
    } else {
        text
    }
}

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

/// Compute max column widths from data (before min-enforcement and distribution).
fn compute_max_widths(changes: &[PriceChange]) -> [usize; 6] {
    let mut max_symbol_len = 10;
    let mut max_open_len = 10;
    let mut max_current_len = 10;
    let mut max_change_pct_len = 10;
    let mut max_change_val_len = 12;

    for change in changes {
        max_symbol_len = max_symbol_len.max(change.symbol.len());
        max_open_len = max_open_len.max(format_price(change.open_price).len());
        max_current_len = max_current_len.max(format_price(change.current_price).len());
        max_change_pct_len = max_change_pct_len.max(change.change_percent_formatted().len());
        max_change_val_len = max_change_val_len.max(change.change_value_formatted().len());
    }

    [
        max_symbol_len,
        max_open_len,
        max_current_len,
        max_change_pct_len,
        max_change_val_len,
        10, // volume is fixed
    ]
}

/// Enforce minimum widths and distribute extra space evenly.
fn distribute_widths(raw: [usize; 6], term_width: usize) -> [usize; 6] {
    let min_widths = [10, 10, 10, 10, 12, 10];
    let mut widths = [
        raw[0].max(min_widths[0]),
        raw[1].max(min_widths[1]),
        raw[2].max(min_widths[2]),
        raw[3].max(min_widths[3]),
        raw[4].max(min_widths[4]),
        raw[5].max(min_widths[5]),
    ];

    let total_min: usize = min_widths.iter().sum();
    let padding = 7;
    let available = term_width.saturating_sub(padding);

    if available >= total_min {
        let extra = available - total_min;
        let per_col = extra / widths.len();
        for width in &mut widths {
            *width += per_col;
        }
    }

    widths
}

/// Display screener results with optional filtering and statistics.
pub fn display(
    changes: &[PriceChange],
    top_n: Option<usize>,
    min_change: Option<f64>,
    min_volume: Option<f64>,
    symbol_filter: Option<&str>,
) {
    let filtered = apply_filters(changes, top_n, min_change, min_volume, symbol_filter);
    print_table(&filtered);
}

/// Display summary statistics for price changes.
pub fn display_stats(changes: &[PriceChange]) {
    let total = changes.len();

    println!();
    println!("Stats:");
    println!("  Total: {} symbols", total);

    // Gainers
    let (gainer_count, gainer_avg) = compute_group_stats(changes, |c| c.change_percent > 0.0);
    println!("  Gainers: {} (avg {:.2}%)", gainer_count, gainer_avg);

    // Losers
    let (loser_count, loser_avg) = compute_group_stats(changes, |c| c.change_percent < 0.0);
    println!("  Losers: {} (avg {:.2}%)", loser_count, loser_avg);

    // Unchanged
    let (unchanged_count, _) = compute_group_stats(changes, |c| c.change_percent == 0.0);
    println!("  Unchanged: {}", unchanged_count);
}

/// Apply all filters, sort, and truncate to the result set.
fn apply_filters(
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

fn print_table(changes: &[PriceChange]) {
    if changes.is_empty() {
        println!("No price changes to display.");
        return;
    }

    let term_width = terminal_width(120);
    let col_widths = calculate_column_widths(changes, term_width);

    print_header(&col_widths);
    print_separator(&col_widths);

    for change in changes {
        print_row(change, &col_widths);
    }
}

fn calculate_column_widths(changes: &[PriceChange], term_width: usize) -> [usize; 6] {
    let max_widths = compute_max_widths(changes);
    distribute_widths(max_widths, term_width)
}

fn print_header(col_widths: &[usize; 6]) {
    println!(
        "{:<w1$} | {:>w2$} | {:>w3$} | {:>w4$} | {:>w5$} | {:>w6$}",
        "Symbol",
        "Open",
        "Current",
        "Change %",
        "Change Value",
        "Volume 24h",
        w1 = col_widths[0],
        w2 = col_widths[1],
        w3 = col_widths[2],
        w4 = col_widths[3],
        w5 = col_widths[4],
        w6 = col_widths[5],
    );
}

fn print_separator(col_widths: &[usize; 6]) {
    println!(
        "{:-<w1$}-+-{:-<w2$}-+-{:-<w3$}-+-{:-<w4$}-+-{:-<w5$}-+-{:-<w6$}",
        "",
        "",
        "",
        "",
        "",
        "",
        w1 = col_widths[0],
        w2 = col_widths[1],
        w3 = col_widths[2],
        w4 = col_widths[3],
        w5 = col_widths[4],
        w6 = col_widths[5],
    );
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

fn print_row(change: &PriceChange, col_widths: &[usize; 6]) {
    let open_str = format_price(change.open_price);
    let current_str = format_price(change.current_price);
    let change_pct_str = change.change_percent_formatted();
    let change_val_str = change.change_value_formatted();
    let volume_str = format_volume(change.volume_usdt());

    let change_pct_colored = colorize(change.change_percent, change_pct_str);
    let change_val_colored = colorize(change.change_value, change_val_str);

    println!(
        "{:<w1$} | {:>w2$} | {:>w3$} | {:>w4$} | {:>w5$} | {:>w6$}",
        change.symbol,
        open_str,
        current_str,
        change_pct_colored,
        change_val_colored,
        volume_str,
        w1 = col_widths[0],
        w2 = col_widths[1],
        w3 = col_widths[2],
        w4 = col_widths[3],
        w5 = col_widths[4],
        w6 = col_widths[5],
    );
}

fn format_volume(volume: f64) -> String {
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
