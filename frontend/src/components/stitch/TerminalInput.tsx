'use client';

import { Input } from '@/components/ui/input';
import { cn } from '@/lib/utils';
import { Search, type LucideIcon } from 'lucide-react';
import type { InputHTMLAttributes } from 'react';

interface TerminalInputProps extends Omit<InputHTMLAttributes<HTMLInputElement>, 'size'> {
  label?: string;
  icon?: LucideIcon;
  variant?: 'default' | 'compact';
  containerClassName?: string;
}

/**
 * TerminalInput - Search input with ">" prompt prefix
 * 
 * Features:
 * - Monospace font for terminal aesthetic
 * - ">" prefix prompt
 * - Cyan border on focus (Stitch theme)
 * - Optional icon
 * - Two size variants
 */
export function TerminalInput({
  label,
  icon: Icon = Search,
  variant = 'default',
  containerClassName,
  className,
  ...props
}: TerminalInputProps) {
  return (
    <div className={cn('flex flex-col gap-1.5', containerClassName)}>
      {label && (
        <label className="text-xs font-medium text-muted-foreground">
          {label}
        </label>
      )}
      <div className="relative flex items-center">
        <span className="absolute left-3 top-1/2 -translate-y-1/2 font-mono text-sm text-cyan-400">
          &gt;
        </span>
        <Icon className="absolute left-8 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
        <Input
          className={cn(
            'font-mono pl-16',
            'focus-visible:border-cyan-400 focus-visible:ring-cyan-400/20',
            variant === 'compact' && 'h-8 text-sm',
            className
          )}
          {...props}
        />
      </div>
    </div>
  );
}
