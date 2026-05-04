/**
 * Format price with locale-specific separators.
 * @param price - Price value to format
 * @returns Formatted price string with 2-5 decimal places
 * 
 * @example
 * ```ts
 * formatPrice(1234.567) // "1,234.57"
 * formatPrice(0.00123)  // "0.00123"
 * ```
 */
export function formatPrice(price: number): string {
  return price.toLocaleString('en-US', {
    minimumFractionDigits: 2,
    maximumFractionDigits: 5,
  });
}

/**
 * Format volume with K/M/B suffixes for readability.
 * @param volume - Volume value to format
 * @returns Formatted volume string with appropriate suffix
 * 
 * @example
 * ```ts
 * formatVolume(1500)        // "1.50K"
 * formatVolume(2500000)     // "2.50M"
 * formatVolume(3500000000)  // "3.50B"
 * ```
 */
export function formatVolume(volume: number): string {
  if (volume >= 1_000_000_000) return `${(volume / 1_000_000_000).toFixed(2)}B`;
  if (volume >= 1_000_000) return `${(volume / 1_000_000).toFixed(2)}M`;
  if (volume >= 1_000) return `${(volume / 1_000).toFixed(2)}K`;
  return volume.toLocaleString();
}

/**
 * Format percentage value with sign and fixed decimals.
 * @param value - Percentage value
 * @param decimals - Number of decimal places (default: 2)
 * @returns Formatted percentage string with + sign for positive values
 * 
 * @example
 * ```ts
 * formatPercent(5.5)      // "+5.50%"
 * formatPercent(-3.2)     // "-3.20%"
 * formatPercent(0)        // "0.00%"
 * ```
 */
export function formatPercent(value: number, decimals = 2): string {
  const sign = value > 0 ? '+' : '';
  return `${sign}${value.toFixed(decimals)}%`;
}

/**
 * Format change value with sign and fixed decimals.
 * @param value - Change value
 * @param decimals - Number of decimal places (default: 5)
 * @returns Formatted change string with + sign for positive values
 * 
 * @example
 * ```ts
 * formatChange(0.00123)  // "+0.00123"
 * formatChange(-0.5)     // "-0.50000"
 * ```
 */
export function formatChange(value: number, decimals = 5): string {
  const sign = value > 0 ? '+' : '';
  return `${sign}${value.toFixed(decimals)}`;
}
