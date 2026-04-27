use crate::models::{Statistics, Symbol};
use crate::utils::terminal_width;

/// Format and print the full report.
///
/// Outputs a complete formatted report including header, statistics,
/// and sample symbols to stdout.
pub fn format(exchange_name: &str, categories: &[&str], symbols: &[Symbol], stats: &Statistics) {
    println!();
    print_header(exchange_name, categories);
    println!();
    print_statistics(stats);
    println!();
    print_sample_symbols(symbols);
    println!();
}

/// Print the report header with exchange and category information.
fn print_header(exchange_name: &str, categories: &[&str]) {
    let title = format!(
        "=== CryptoScope: {} Perpetual Symbols ===",
        exchange_name.to_uppercase()
    );

    println!("{}", title);
    println!();
    println!("Exchange: {}", exchange_name.to_uppercase());
    println!("Categories: {}", categories.join(", "));
}

/// Print statistics breakdown by category and contract type.
fn print_statistics(stats: &Statistics) {
    println!("📊 Statistics:");
    println!("  Total Symbols: {}", stats.total_count);
    println!();

    print_category_breakdown(stats);
    println!();
    print_contract_breakdown(stats);
}

/// Print category breakdown section.
fn print_category_breakdown(stats: &Statistics) {
    println!("  By Category:");
    for (category, count) in &stats.by_category {
        let description = match category.as_str() {
            "linear" => " (USDT Perpetual)",
            "inverse" => " (Inverse Perpetual)",
            _ => "",
        };
        println!("    {}{}: {}", category.to_uppercase(), description, count);
    }
}

/// Print contract type breakdown section.
fn print_contract_breakdown(stats: &Statistics) {
    println!("  By Contract Type:");
    let mut contract_counts: Vec<_> = stats.by_contract_type.iter().collect();
    contract_counts.sort_by(|a, b| b.1.cmp(a.1));
    for (contract_type, count) in contract_counts {
        println!("    {}: {}", contract_type, count);
    }
}

/// Print a sample of symbols (first 20) with line wrapping.
///
/// Formats symbol names as a comma-separated list that wraps
/// based on the terminal width.
fn print_sample_symbols(symbols: &[Symbol]) {
    println!("📋 Sample Symbols (first 20):");

    let sample: Vec<_> = symbols.iter().take(20).collect();
    let symbol_names: Vec<_> = sample.iter().map(|s| s.symbol.as_str()).collect();

    // Format as comma-separated list with line wrapping
    let mut line = String::from("  ");
    let width = terminal_width(80);

    for (i, symbol) in symbol_names.iter().enumerate() {
        let suffix = if i < symbol_names.len() - 1 { ", " } else { "" };
        let entry = format!("{}{}", symbol, suffix);

        if line.len() + entry.len() > width - 2 {
            println!("{}", line);
            line = String::from("  ");
        }
        line.push_str(&entry);
    }

    if !line.trim().is_empty() {
        println!("{}", line);
    }

    if symbols.len() > 20 {
        println!("  ... and {} more", symbols.len() - 20);
    }
}

/// Filter symbols by search term (case-insensitive).
///
/// Returns symbols whose name contains the search term.
/// Returns all symbols if the search term is empty.
fn by_search(symbols: &[Symbol], search_term: &str) -> Vec<Symbol> {
    if search_term.is_empty() {
        return symbols.to_vec();
    }

    let search_lower = search_term.to_lowercase();
    symbols
        .iter()
        .filter(|s| s.symbol.to_lowercase().contains(&search_lower))
        .cloned()
        .collect()
}

/// Apply search filter to symbols.
/// Returns all symbols if no filter is provided.
pub fn apply(symbols: &[Symbol], search: Option<&str>) -> Vec<Symbol> {
    let mut filtered = symbols.to_vec();

    if let Some(search_term) = search {
        filtered = by_search(&filtered, search_term);
    }

    filtered
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::create_test_symbol;

    #[test]
    fn test_filter_by_search() {
        let symbols = vec![
            create_test_symbol("BTCUSDT", "linear"),
            create_test_symbol("ETHUSDT", "linear"),
            create_test_symbol("BTCUSD", "inverse"),
        ];

        let filtered = by_search(&symbols, "BTC");
        assert_eq!(filtered.len(), 2);
    }
}
