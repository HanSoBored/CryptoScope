import axios from 'axios';

// Base API configuration
const API_BASE_URL = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:3000';

export const api = axios.create({
  baseURL: API_BASE_URL,
  headers: {
    'Content-Type': 'application/json',
  },
});

// Types (manual for now, can be auto-generated from Rust types via ts-rs later)

export interface SymbolInfo {
  symbol: string;
  name: string;
  exchange: string;
  asset_class: string;
  sector?: string;
  industry?: string;
  market_cap?: number;
  is_active: boolean;
}

// Crypto-specific types for Bybit API
export interface CryptoSymbol {
  symbol: string;
  category: string;
  contractType: string;
  baseCoin: string;
  quoteCoin: string;
  launchTime: string;
  deliveryTime: string;
  deliveryFeeRate: string;
  // Optional fields for backward compatibility or extended data
  status?: 'Trading' | 'Closed' | 'Delisted';
  lot_size_filter?: {
    base_precision: string;
    quote_precision: string;
  };
  price_filter?: {
    tick_size: string;
  };
}

export interface PriceData {
  symbol: string;
  price: number;
  change: number;
  change_percent: number;
  volume: number;
  timestamp: string;
}

export interface ScreenerItem {
  symbol: string;
  category: string;
  contract_type: string;
  open_price: number;
  current_price: number;
  change_value: number;
  change_percent: number;
  volume_24h: number;
}

export interface ScreenerCriteria {
  exchange?: string;
  asset_class?: string;
  sector?: string;
  min_market_cap?: number;
  max_market_cap?: number;
  min_price?: number;
  max_price?: number;
  is_active?: boolean;
}

export interface ScreenerParams {
  exchange?: string;
  mode?: 'kline' | 'mark';
  top?: number;
  min_change?: number;
}

export interface StatsSummary {
  total_symbols: number;
  active_symbols: number;
  exchanges: Record<string, number>;
  asset_classes: Record<string, number>;
  sectors?: Record<string, number>;
}

export interface CryptoStats {
  total_count: number;
  by_category: Array<{ category: string; count: number }>;
  by_contract_type: Array<{ category: string; count: number }>;
}

// API Response wrappers
export interface SymbolsListResponse {
  symbols: CryptoSymbol[];
}

export interface ScreenerResponse {
  results: ScreenerItem[];
  statistics: CryptoStats;
}

export interface HealthStatus {
  status: string;
  version: string;
  uptime: number;
}

// API Methods

export const symbolsApi = {
  // GET /api/v1/symbols - List all crypto symbols
  list: async (params?: { exchange?: string; category?: string; search?: string }) => {
    const response = await api.get<SymbolsListResponse>('/api/v1/symbols', { params });
    return response.data.symbols;
  },

  // GET /api/symbols - List all symbols (legacy)
  listLegacy: async (params?: { exchange?: string; asset_class?: string; limit?: number; offset?: number }) => {
    const response = await api.get<SymbolInfo[]>('/api/symbols', { params });
    return response.data;
  },

  // GET /api/symbols/{symbol} - Get symbol details
  get: async (symbol: string) => {
    const response = await api.get<SymbolInfo>(`/api/symbols/${encodeURIComponent(symbol)}`);
    return response.data;
  },

  // GET /api/symbols/{symbol}/price - Get current price
  getPrice: async (symbol: string) => {
    const response = await api.get<PriceData>(`/api/symbols/${encodeURIComponent(symbol)}/price`);
    return response.data;
  },
};

export const screenerApi = {
  // GET /api/v1/screener - Screen symbols by criteria
  screen: async (params?: ScreenerParams) => {
    const response = await api.get<ScreenerResponse>('/api/v1/screener', { params });
    return response.data.results;
  },

  // POST /api/screener - Screen symbols by criteria (legacy)
  screenLegacy: async (criteria: ScreenerCriteria) => {
    const response = await api.post<SymbolInfo[]>('/api/screener', criteria);
    return response.data;
  },
};

export const statsApi = {
  // GET /api/v1/stats - Get crypto statistics
  getCryptoStats: async (params?: { exchange?: string; category?: string }) => {
    const response = await api.get<{ statistics: CryptoStats }>('/api/v1/stats', { params });
    return response.data.statistics;
  },

  // GET /api/stats - Get statistics summary (legacy)
  getSummary: async () => {
    const response = await api.get<StatsSummary>('/api/stats');
    return response.data;
  },
};

export const healthApi = {
  // GET /api/health - Health check
  check: async () => {
    const response = await api.get<HealthStatus>('/api/health');
    return response.data;
  },
};
