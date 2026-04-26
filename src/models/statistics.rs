use super::symbol::Symbol;
use serde::Serialize;
use std::collections::HashMap;

/// Aggregated statistics about fetched symbols.
///
/// Contains counts of symbols organized by category and contract type
/// for analysis and reporting purposes.
#[derive(Debug, Clone, Default, Serialize)]
pub struct Statistics {
    /// Total number of symbols in the dataset
    pub total_count: usize,
    /// Count of symbols grouped by category (linear, inverse, etc.)
    #[serde(serialize_with = "serialize_category_map")]
    pub by_category: HashMap<String, usize>,
    /// Count of symbols grouped by contract type
    pub by_contract_type: HashMap<String, usize>,
}

/// Serialize a HashMap<String, usize> as a Vec of {category, count} objects.
/// Keys are sorted alphabetically for deterministic output.
///
/// WHY: HashMap iteration order is non-deterministic across runs. Sorting ensures
/// stable JSON output, which is critical for snapshot testing, reproducible diffs,
/// and reliable integration test assertions.
fn serialize_category_map<S>(map: &HashMap<String, usize>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    use serde::ser::SerializeSeq;

    let mut sorted_entries: Vec<_> = map.iter().collect();
    sorted_entries.sort_by_key(|(k, _)| *k);

    let mut seq = serializer.serialize_seq(Some(sorted_entries.len()))?;
    for (category, count) in sorted_entries {
        seq.serialize_element(&CategoryEntry {
            category,
            count: *count,
        })?;
    }
    seq.end()
}

#[derive(Serialize)]
struct CategoryEntry<'a> {
    category: &'a str,
    count: usize,
}

impl Statistics {
    /// Create statistics from a list of symbols
    ///
    /// Aggregates symbol counts by category and contract type
    /// for analysis and reporting.
    pub fn from_symbols(symbols: &[Symbol]) -> Self {
        let mut by_category = HashMap::new();
        let mut by_contract_type = HashMap::new();

        for symbol in symbols {
            *by_category
                .entry(symbol.category().to_string())
                .or_insert(0) += 1;
            *by_contract_type
                .entry(symbol.contract_type().to_string())
                .or_insert(0) += 1;
        }

        Self {
            total_count: symbols.len(),
            by_category,
            by_contract_type,
        }
    }

    /// Get count for a specific category
    ///
    /// Returns the number of symbols in the given category, or 0 if none.
    #[allow(dead_code)]
    pub fn count_by_category(&self, category: &str) -> usize {
        *self.by_category.get(category).unwrap_or(&0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::create_test_symbol;

    #[test]
    fn test_statistics_aggregation() {
        let symbols = vec![
            create_test_symbol("BTCUSDT", "linear"),
            create_test_symbol("ETHUSDT", "linear"),
            create_test_symbol("BTCUSD", "inverse"),
        ];

        let stats = Statistics::from_symbols(&symbols);

        assert_eq!(stats.total_count, 3);
        assert_eq!(stats.count_by_category("linear"), 2);
        assert_eq!(stats.count_by_category("inverse"), 1);
    }
}
