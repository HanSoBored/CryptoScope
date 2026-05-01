'use client';

import { useQuery } from '@tanstack/react-query';
import { statsApi, type CryptoStats } from '@/lib/api';

interface UseStatsParams {
  exchange?: string;
  category?: string;
  enabled?: boolean;
  refetchInterval?: number | false;
}

export function useStats(params: UseStatsParams = {}) {
  const {
    exchange = 'bybit',
    category,
    enabled = true,
    refetchInterval = 60000, // 60 seconds for stats
  } = params;

  return useQuery<CryptoStats, Error>({
    queryKey: ['stats', exchange, category],
    queryFn: () => statsApi.getCryptoStats({ exchange, category }),
    enabled,
    refetchInterval,
    staleTime: 10000, // Consider data fresh for 10 seconds
    retry: 2,
  });
}
