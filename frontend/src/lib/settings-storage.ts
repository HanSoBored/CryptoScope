import { encrypt, decrypt, generateSessionPassphrase } from '@/lib/crypto';

/**
 * Storage keys for localStorage and sessionStorage.
 */
export const STORAGE_KEYS = {
  API_KEYS: 'cryptoscope-api-keys',
  REFRESH_INTERVAL: 'cryptoscope-refresh-interval',
  DENSE_MODE: 'cryptoscope-dense-mode',
  SESSION_PASSPHRASE: 'cryptoscope-session-passphrase',
} as const;

/**
 * Exchange names supported by the application.
 */
export const EXCHANGES = ['Bybit', 'Binance', 'OKX'] as const;
export type ExchangeName = typeof EXCHANGES[number];

/**
 * Mapping from exchange name to API keys object key.
 */
export const EXCHANGE_KEY_MAP: Record<ExchangeName, keyof APIKeys> = {
  Bybit: 'bybit',
  Binance: 'binance',
  OKX: 'okx',
} as const;

/**
 * Available refresh interval options in seconds.
 * 0 = manual refresh only.
 */
export const REFRESH_OPTIONS = [
  { value: 5, label: '5 seconds' },
  { value: 10, label: '10 seconds' },
  { value: 30, label: '30 seconds' },
  { value: 60, label: '1 minute' },
  { value: 0, label: 'Manual only' },
] as const;
export type RefreshInterval = typeof REFRESH_OPTIONS[number]['value'];

/**
 * API keys structure for all supported exchanges.
 */
export interface APIKeys {
  bybit: { apiKey: string; apiSecret: string };
  binance: { apiKey: string; apiSecret: string };
  okx: { apiKey: string; apiSecret: string; passphrase?: string };
}

/**
 * Get session passphrase from sessionStorage.
 * @returns Passphrase or null if not found
 */
export function getSessionPassphrase(): string | null {
  if (typeof window === 'undefined') return null;
  return sessionStorage.getItem(STORAGE_KEYS.SESSION_PASSPHRASE);
}

/**
 * Get or create a session passphrase.
 * Creates a new one if it doesn't exist.
 * @returns Session passphrase
 */
export function getOrCreateSessionPassphrase(): string {
  if (typeof window === 'undefined') return '';
  let passphrase = sessionStorage.getItem(STORAGE_KEYS.SESSION_PASSPHRASE);
  if (!passphrase) {
    passphrase = generateSessionPassphrase();
    sessionStorage.setItem(STORAGE_KEYS.SESSION_PASSPHRASE, passphrase);
  }
  return passphrase;
}

/**
 * Get empty API keys structure.
 * @returns API keys object with all fields empty
 */
export function getEmptyAPIKeys(): APIKeys {
  return {
    bybit: { apiKey: '', apiSecret: '' },
    binance: { apiKey: '', apiSecret: '' },
    okx: { apiKey: '', apiSecret: '', passphrase: '' },
  };
}

/**
 * Get stored API keys from localStorage (decrypted).
 * @returns Decrypted API keys or empty keys if not found/decryption fails
 */
export async function getStoredAPIKeys(): Promise<APIKeys> {
  if (typeof window === 'undefined') {
    return getEmptyAPIKeys();
  }
  const stored = localStorage.getItem(STORAGE_KEYS.API_KEYS);
  if (stored) {
    try {
      const passphrase = getSessionPassphrase();
      if (passphrase) {
        // Check for version marker to distinguish encrypted vs plaintext data
        // Encrypted data starts with "v1:" prefix
        if (stored.startsWith('v1:')) {
          const decrypted = await decrypt(stored, passphrase);
          return JSON.parse(decrypted);
        }
        // No version marker - assume it's old unencrypted data for backward compatibility
        // Try parsing as plain JSON
        try {
          return JSON.parse(stored);
        } catch {
          // Not valid JSON, return empty keys
          return getEmptyAPIKeys();
        }
      }
      // No passphrase - try parsing as plain JSON (backward compatibility)
      return JSON.parse(stored);
    } catch {
      return getEmptyAPIKeys();
    }
  }
  return getEmptyAPIKeys();
}

/**
 * Save API keys to localStorage (encrypted).
 * @param keys - API keys to save
 */
export async function saveAPIKeys(keys: APIKeys): Promise<void> {
  const passphrase = getOrCreateSessionPassphrase();
  const encrypted = await encrypt(JSON.stringify(keys), passphrase);
  // Add version marker "v1:" prefix to distinguish encrypted data from plaintext
  localStorage.setItem(STORAGE_KEYS.API_KEYS, `v1:${encrypted}`);
}

/**
 * Check if an exchange has API keys configured.
 * @param keys - API keys object
 * @param exchange - Exchange name
 * @returns true if both apiKey and apiSecret are present
 */
export function hasAPIKey(keys: APIKeys, exchange: ExchangeName): boolean {
  const exchangeKey = EXCHANGE_KEY_MAP[exchange];
  const exchangeKeys = keys[exchangeKey];
  return !!(exchangeKeys.apiKey && exchangeKeys.apiSecret);
}

/**
 * Mask an API key for display (show first 4 and last 4 characters).
 * @param key - API key to mask
 * @returns Masked key string
 */
export function maskKey(key: string): string {
  if (!key) return '';
  if (key.length <= 8) return '••••••••';
  return `${key.slice(0, 4)}${'•'.repeat(key.length - 8)}${key.slice(-4)}`;
}

/**
 * Get stored refresh interval from localStorage.
 * @returns Refresh interval in seconds (default: 10)
 */
export function getStoredRefreshInterval(): RefreshInterval {
  if (typeof window === 'undefined') return 10;
  const stored = localStorage.getItem(STORAGE_KEYS.REFRESH_INTERVAL);
  if (stored) {
    const value = parseInt(stored, 10);
    if ([5, 10, 30, 60, 0].includes(value)) {
      return value as RefreshInterval;
    }
  }
  return 10;
}

/**
 * Save refresh interval to localStorage.
 * @param interval - Refresh interval in seconds
 */
export function saveRefreshInterval(interval: RefreshInterval): void {
  localStorage.setItem(STORAGE_KEYS.REFRESH_INTERVAL, interval.toString());
}

/**
 * Get dense mode setting from localStorage.
 * @returns true if dense mode is enabled
 */
export function getDenseMode(): boolean {
  if (typeof window === 'undefined') return false;
  const stored = localStorage.getItem(STORAGE_KEYS.DENSE_MODE);
  return stored === 'true';
}

/**
 * Save dense mode setting to localStorage and update DOM attribute.
 * @param enabled - Whether dense mode should be enabled
 */
export function saveDenseMode(enabled: boolean): void {
  if (typeof window === 'undefined') return;
  localStorage.setItem(STORAGE_KEYS.DENSE_MODE, enabled.toString());
  document.documentElement.setAttribute('data-dense-mode', enabled.toString());
}
