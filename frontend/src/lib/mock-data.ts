import type { ComparisonRow } from '@/app/compare/page';

/**
 * Generate mock comparison data for cross-exchange price comparison.
 * This simulates price differences across exchanges for demo purposes.
 * 
 * @param selectedSymbol - Selected trading pair symbol or 'ALL'
 * @returns Array of comparison data rows
 * 
 * @deprecated Replace with real API call when backend supports multi-exchange
 */
export function generateMockComparisonData(selectedSymbol: string): ComparisonRow[] {
  const symbols = [
    selectedSymbol === 'ALL' ? 'BTCUSDT' : selectedSymbol,
    selectedSymbol === 'ALL' ? 'ETHUSDT' : '',
    selectedSymbol === 'ALL' ? 'SOLUSDT' : '',
    selectedSymbol === 'ALL' ? 'XRPUSDT' : '',
    selectedSymbol === 'ALL' ? 'BNBUSDT' : '',
  ].filter(Boolean);

  return symbols.map((symbol) => {
    // Base price with some randomness per exchange
    const basePrice = symbol.includes('BTC') 
      ? 95000 + Math.random() * 500 
      : symbol.includes('ETH') 
        ? 3500 + Math.random() * 100 
        : symbol.includes('SOL') 
          ? 180 + Math.random() * 10 
          : symbol.includes('XRP') 
            ? 2.5 + Math.random() * 0.2 
            : 600 + Math.random() * 20;

    // Exchange-specific prices (simulate price differences)
    const bybitPrice = basePrice;
    const binancePrice = basePrice * (1 + (Math.random() - 0.5) * 0.002); // ±0.1%
    const okxPrice = basePrice * (1 + (Math.random() - 0.5) * 0.003); // ±0.15%

    const prices = [bybitPrice, binancePrice, okxPrice];
    const minPrice = Math.min(...prices);
    const maxPrice = Math.max(...prices);
    const avgPrice = (bybitPrice + binancePrice + okxPrice) / 3;
    const priceDiff = maxPrice - minPrice;
    const priceDiffPercent = (priceDiff / avgPrice) * 100;

    // Determine best/worst exchanges
    const priceMap = {
      'Bybit': bybitPrice,
      'Binance': binancePrice,
      'OKX': okxPrice,
    };
    const bestExchange = Object.entries(priceMap).find(([, price]) => price === minPrice)?.[0] || 'Bybit';
    const worstExchange = Object.entries(priceMap).find(([, price]) => price === maxPrice)?.[0] || 'Bybit';

    // Volumes
    const bybitVolume = Math.floor(1000000 + Math.random() * 5000000);
    const binanceVolume = Math.floor(2000000 + Math.random() * 8000000);
    const okxVolume = Math.floor(500000 + Math.random() * 3000000);

    return {
      symbol,
      bybit_price: bybitPrice,
      binance_price: binancePrice,
      okx_price: okxPrice,
      avg_price: avgPrice,
      price_diff: priceDiff,
      price_diff_percent: priceDiffPercent,
      bybit_volume: bybitVolume,
      binance_volume: binanceVolume,
      okx_volume: okxVolume,
      total_volume: bybitVolume + binanceVolume + okxVolume,
      arbitrage_opportunity: priceDiffPercent > 0.1, // Threshold for arbitrage
      best_exchange: bestExchange,
      worst_exchange: worstExchange,
      status: 'simulated' as const,
    };
  });
}
