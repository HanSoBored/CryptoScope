import type { Column } from '@/components/DataTable';
import type { ComparisonRow } from './page';
import { StatusPip } from '@/components/stitch';
import { Badge } from '@/components/ui/badge';
import { cn } from '@/lib/utils';
import { ArrowLeftRight, TrendingUp, TrendingDown } from 'lucide-react';
import { formatPrice } from '@/lib/formatters';

const statusVariantMap: Record<string, 'connected' | 'connecting' | 'disconnected' | 'error'> = {
  connected: 'connected',
  connecting: 'connecting',
  disconnected: 'disconnected',
  error: 'error',
  simulated: 'connecting',
};

/**
 * Create column definitions for the comparison data table.
 * @param _arbitrageThreshold - Price diff % threshold for highlighting arbitrage opportunities
 * @returns Array of column definitions
 */
export function createComparisonColumns(_arbitrageThreshold: number = 0.1): Column<ComparisonRow>[] {
  return [
    {
      key: 'symbol',
      header: 'Symbol',
      sortable: true,
      render: (item) => (
        <div className="font-semibold">{item.symbol}</div>
      ),
    },
    {
      key: 'bybit_price',
      header: 'Bybit',
      sortable: true,
      render: (item) => (
        <div className="space-y-1">
          <div className="flex items-center gap-1.5">
            <StatusPip 
              variant={statusVariantMap[item.status] || 'disconnected'} 
              size="sm" 
            />
            <span className="font-mono text-sm">
              ${formatPrice(item.bybit_price)}
            </span>
          </div>
          <div className="text-xs text-muted-foreground">
            Vol: {(item.bybit_volume / 1000000).toFixed(2)}M
          </div>
        </div>
      ),
    },
    {
      key: 'binance_price',
      header: 'Binance',
      sortable: true,
      render: (item) => (
        <div className="space-y-1">
          <div className="flex items-center gap-1.5">
            <StatusPip 
              variant={statusVariantMap[item.status] || 'disconnected'} 
              size="sm" 
            />
            <span className="font-mono text-sm">
              ${formatPrice(item.binance_price)}
            </span>
          </div>
          <div className="text-xs text-muted-foreground">
            Vol: {(item.binance_volume / 1000000).toFixed(2)}M
          </div>
        </div>
      ),
    },
    {
      key: 'okx_price',
      header: 'OKX',
      sortable: true,
      render: (item) => (
        <div className="space-y-1">
          <div className="flex items-center gap-1.5">
            <StatusPip 
              variant={statusVariantMap[item.status] || 'disconnected'} 
              size="sm" 
            />
            <span className="font-mono text-sm">
              ${formatPrice(item.okx_price)}
            </span>
          </div>
          <div className="text-xs text-muted-foreground">
            Vol: {(item.okx_volume / 1000000).toFixed(2)}M
          </div>
        </div>
      ),
    },
    {
      key: 'avg_price',
      header: 'Avg Price',
      sortable: true,
      render: (item) => (
        <span className="font-mono text-sm text-muted-foreground">
          ${formatPrice(item.avg_price)}
        </span>
      ),
    },
    {
      key: 'price_diff_percent',
      header: 'Price Diff %',
      sortable: true,
      render: (item) => (
        <Badge
          variant={item.price_diff_percent >= 0 ? 'default' : 'secondary'}
          className={cn(
            'font-mono',
            item.arbitrage_opportunity 
              ? 'bg-amber-500 hover:bg-amber-600 text-white' 
              : item.price_diff_percent > 0 
                ? 'bg-emerald-500/20 text-emerald-600 hover:bg-emerald-500/30' 
                : 'bg-muted text-muted-foreground'
          )}
        >
          {item.arbitrage_opportunity && <ArrowLeftRight className="mr-1 h-3 w-3" />}
          {item.price_diff_percent > 0 && '+'}
          {item.price_diff_percent.toFixed(3)}%
        </Badge>
      ),
    },
    {
      key: 'price_diff',
      header: 'Price Diff ($)',
      sortable: true,
      render: (item) => (
        <span className={cn(
          'font-mono text-sm',
          item.arbitrage_opportunity ? 'text-amber-500 font-semibold' : 'text-muted-foreground'
        )}>
          ${item.price_diff.toFixed(2)}
        </span>
      ),
    },
    {
      key: 'total_volume',
      header: 'Total Volume (24h)',
      sortable: true,
      render: (item) => (
        <span className="font-mono text-sm text-muted-foreground">
          ${(item.total_volume / 1000000).toFixed(2)}M
        </span>
      ),
    },
    {
      key: 'best_exchange',
      header: 'Best/Worst',
      sortable: false,
      render: (item) => (
        <div className="space-y-1 text-xs">
          <div className="flex items-center gap-1 text-emerald-600">
            <TrendingUp className="h-3 w-3" />
            <span className="font-medium">{item.best_exchange}</span>
          </div>
          <div className="flex items-center gap-1 text-red-500">
            <TrendingDown className="h-3 w-3" />
            <span className="font-medium">{item.worst_exchange}</span>
          </div>
        </div>
      ),
    },
  ];
}
