'use client';

import { useQuery } from '@tanstack/react-query';
import { screenerApi, type ScreenerItem } from '@/lib/api';

interface UseScreenerParams {
  exchange?: string;
  mode?: 'kline' | 'mark';
  top?: number;
  minChange?: number;
  enabled?: boolean;
  refetchInterval?: number | false;
}

export function useScreener(params: UseScreenerParams = {}) {
  const {
    exchange = 'bybit',
    mode = 'kline',
    top = 20,
    minChange = 0,
    enabled = true,
    refetchInterval = 10000, // 10 seconds for screener
  } = params;

  return useQuery<ScreenerItem[], Error>({
    queryKey: ['screener', exchange, mode, top, minChange],
    queryFn: () => screenerApi.screen({ exchange, mode, top, min_change: minChange }),
    enabled,
    refetchInterval,
    staleTime: 3000, // Consider data fresh for 3 seconds
    retry: 2,
  });
}
