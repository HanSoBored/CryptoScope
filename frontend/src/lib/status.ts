import type { StatusPipVariant } from '@/components/stitch';

/**
 * Human-readable labels for connection status variants.
 */
export const STATUS_LABELS: Record<StatusPipVariant, string> = {
  connected: 'Connected',
  connecting: 'Connecting...',
  disconnected: 'Disconnected',
  error: 'Connection Error',
};

/**
 * Get human-readable label for connection status.
 * @param status - Connection status variant
 * @returns Formatted status label string
 * 
 * @example
 * ```ts
 * getStatusLabel('connected')    // "Connected"
 * getStatusLabel('connecting')   // "Connecting..."
 * ```
 */
export function getStatusLabel(status: StatusPipVariant): string {
  return STATUS_LABELS[status];
}
