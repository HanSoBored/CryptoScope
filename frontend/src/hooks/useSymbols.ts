'use client';

import { useQuery } from '@tanstack/react-query';
import { symbolsApi, type CryptoSymbol } from '@/lib/api';

interface UseSymbolsParams {
  exchange?: string;
  category?: string;
  search?: string;
  enabled?: boolean;
  refetchInterval?: number | false;
}

export function useSymbols(params: UseSymbolsParams = {}) {
  const {
    exchange = 'bybit',
    category,
    search,
    enabled = true,
    refetchInterval = 30000, // 30 seconds
  } = params;

  return useQuery<CryptoSymbol[], Error>({
    queryKey: ['symbols', exchange, category, search],
    queryFn: () => symbolsApi.list({ exchange, category, search }),
    enabled,
    refetchInterval,
    staleTime: 5000, // Consider data fresh for 5 seconds
    retry: 2,
  });
}
