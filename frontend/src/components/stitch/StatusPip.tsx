'use client';

import { cn } from '@/lib/utils';

export type StatusPipVariant = 'connected' | 'connecting' | 'disconnected' | 'error';

interface StatusPipProps {
  variant?: StatusPipVariant;
  size?: 'sm' | 'md' | 'lg';
  showLabel?: boolean;
  className?: string;
}

const variantConfig: Record<StatusPipVariant, { color: string; glowColor: string; label: string }> = {
  connected: { 
    color: 'bg-emerald-500', 
    glowColor: 'shadow-[0_0_6px_2px_rgba(16,185,129,0.6)]', 
    label: 'Connected' 
  },
  connecting: { 
    color: 'bg-amber-500', 
    glowColor: 'shadow-[0_0_6px_2px_rgba(245,158,11,0.6)]', 
    label: 'Connecting' 
  },
  disconnected: { 
    color: 'bg-gray-500', 
    glowColor: 'shadow-[0_0_6px_2px_rgba(107,114,128,0.4)]', 
    label: 'Disconnected' 
  },
  error: { 
    color: 'bg-red-500', 
    glowColor: 'shadow-[0_0_6px_2px_rgba(239,68,68,0.6)]', 
    label: 'Error' 
  },
};

const sizeConfig: Record<'sm' | 'md' | 'lg', { pip: string; glow: string }> = {
  sm: { pip: 'h-2 w-2', glow: 'shadow-[0_0_4px_1px]' },
  md: { pip: 'h-2.5 w-2.5', glow: 'shadow-[0_0_6px_2px]' },
  lg: { pip: 'h-3 w-3', glow: 'shadow-[0_0_8px_3px]' },
};

/**
 * StatusPip - Glowing connection status indicator
 * 
 * Features:
 * - 8px circle with CSS box-shadow glow
 * - Green/yellow/red/gray states for connection status
 * - Optional label
 * - Three size variants
 */
export function StatusPip({
  variant = 'connected',
  size = 'md',
  showLabel = false,
  className,
}: StatusPipProps) {
  const config = variantConfig[variant];
  const sizes = sizeConfig[size];

  return (
    <div className={cn('flex items-center gap-2', className)}>
      <div
        className={cn(
          'rounded-full transition-all duration-300',
          sizes.pip,
          config.color,
          sizes.glow,
          config.glowColor
        )}
      />
      {showLabel && (
        <span className="text-xs font-medium text-muted-foreground">
          {config.label}
        </span>
      )}
    </div>
  );
}
