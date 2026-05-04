'use client';

import { cn } from '@/lib/utils';
import { X } from 'lucide-react';
import type { ButtonHTMLAttributes } from 'react';

interface FilterChipProps extends ButtonHTMLAttributes<HTMLButtonElement> {
  label: string;
  active?: boolean;
  onDismiss?: () => void;
  variant?: 'default' | 'secondary' | 'outline';
  size?: 'sm' | 'md';
}

/**
 * FilterChip - Compact filter tag with dismiss
 * 
 * Features:
 * - Small tag with label + optional "X" dismiss button
 * - Secondary color scheme by default
 * - Active state highlighting
 * - Two size variants
 */
export function FilterChip({
  label,
  active = true,
  onDismiss,
  variant = 'secondary',
  size = 'sm',
  className,
  ...props
}: FilterChipProps) {
  const variantClasses = {
    default: active
      ? 'bg-primary text-primary-foreground hover:bg-primary/90'
      : 'bg-muted text-muted-foreground hover:bg-muted/80',
    secondary: active
      ? 'bg-cyan-500/10 text-cyan-400 border-cyan-500/20 hover:bg-cyan-500/20'
      : 'bg-muted text-muted-foreground border-border hover:bg-muted/80',
    outline: 'bg-transparent border border-border text-foreground hover:bg-muted',
  };

  const sizeClasses = {
    sm: 'h-6 px-2 text-xs gap-1',
    md: 'h-7 px-2.5 text-sm gap-1.5',
  };

  return (
    <button
      type="button"
      className={cn(
        'inline-flex items-center rounded-full border transition-colors duration-200',
        'font-medium',
        variantClasses[variant],
        sizeClasses[size],
        className
      )}
      {...props}
    >
      <span className="truncate max-w-[120px]">{label}</span>
      {onDismiss && (
        <span
          role="button"
          tabIndex={-1}
          onClick={(e) => {
            e.stopPropagation();
            onDismiss();
          }}
          className={cn(
            'flex items-center justify-center rounded-full',
            'hover:bg-black/20 transition-colors',
            size === 'sm' ? 'h-3.5 w-3.5' : 'h-4 w-4'
          )}
        >
          <X className={size === 'sm' ? 'h-2.5 w-2.5' : 'h-3 w-3'} />
        </span>
      )}
    </button>
  );
}
