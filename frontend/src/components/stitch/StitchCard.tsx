'use client';

import { cn } from '@/lib/utils';
import type { ReactNode } from 'react';

interface StitchCardProps {
  children: ReactNode;
  className?: string;
}

interface StitchCardHeaderProps {
  children: ReactNode;
  className?: string;
  showBorder?: boolean;
  actions?: ReactNode;
}

interface StitchCardContentProps {
  children: ReactNode;
  className?: string;
  variant?: 'default' | 'dense';
}

interface StitchCardFooterProps {
  children: ReactNode;
  className?: string;
  variant?: 'default' | 'bordered';
}

/**
 * StitchCard - Modular card with header border
 * 
 * Features:
 * - Card container with Stitch theme styling
 * - Header with optional 1px bottom border
 * - Utility icon slot for actions
 * - Dense content variant for high-density data
 * - Bordered footer variant
 */
function StitchCard({ children, className }: StitchCardProps) {
  return (
    <div
      className={cn(
        'rounded-lg border border-stitch-border bg-card text-card-foreground',
        'shadow-sm',
        className
      )}
    >
      {children}
    </div>
  );
}

function StitchCardHeader({
  children,
  className,
  showBorder = true,
  actions,
}: StitchCardHeaderProps) {
  return (
    <div
      className={cn(
        'flex items-center justify-between px-4 py-3',
        showBorder && 'border-b border-stitch-border',
        className
      )}
    >
      <div className="flex items-center gap-2">{children}</div>
      {actions && <div className="flex items-center gap-1">{actions}</div>}
    </div>
  );
}

function StitchCardContent({
  children,
  className,
  variant = 'default',
}: StitchCardContentProps) {
  return (
    <div
      className={cn(
        'px-4 py-3',
        variant === 'dense' && 'px-3 py-2',
        className
      )}
    >
      {children}
    </div>
  );
}

function StitchCardFooter({
  children,
  className,
  variant = 'default',
}: StitchCardFooterProps) {
  return (
    <div
      className={cn(
        'flex items-center px-4 py-2',
        variant === 'bordered' && 'border-t border-stitch-border bg-muted/30',
        'rounded-b-lg',
        className
      )}
    >
      {children}
    </div>
  );
}

export {
  StitchCard,
  StitchCardHeader,
  StitchCardContent,
  StitchCardFooter,
  type StitchCardProps,
  type StitchCardHeaderProps,
  type StitchCardContentProps,
  type StitchCardFooterProps,
};
