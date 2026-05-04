'use client';

import { cn } from '@/lib/utils';
import { Check } from 'lucide-react';
import type { ThemeName } from '@/lib/stitch';

interface ThemePreviewProps {
  theme: ThemeName;
  isSelected: boolean;
  onSelect: () => void;
}

/**
 * ThemePreview - Visual theme selector card.
 * Shows a preview of the theme with selection indicator.
 */
export function ThemePreview({ theme, isSelected, onSelect }: ThemePreviewProps) {
  const isTechnicalPrecision = theme === 'technical-precision';
  
  return (
    <button
      onClick={onSelect}
      className={cn(
        'relative flex w-full flex-col gap-3 rounded-lg border p-4 text-left transition-all',
        'focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring',
        isSelected
          ? 'border-primary bg-muted/50'
          : 'border-stitch-border hover:border-muted-foreground/50'
      )}
      aria-pressed={isSelected}
    >
      {isSelected && (
        <div className="absolute top-2 right-2">
          <Check className="h-4 w-4 text-primary" />
        </div>
      )}
      
      {/* Preview Header */}
      <div
        className={cn(
          'flex h-8 items-center gap-2 rounded px-2',
          isTechnicalPrecision ? 'bg-[#00F0FF]/20' : 'bg-[#10b981]/20'
        )}
      >
        <div
          className={cn(
            'h-3 w-3 rounded-full',
            isTechnicalPrecision ? 'bg-[#00F0FF]' : 'bg-[#10b981]'
          )}
        />
        <div
          className={cn(
            'h-2 w-16 rounded',
            isTechnicalPrecision ? 'bg-[#00F0FF]/60' : 'bg-[#10b981]/60'
          )}
        />
      </div>
      
      {/* Preview Content */}
      <div className="space-y-2">
        <div
          className={cn(
            'h-6 rounded',
            isTechnicalPrecision ? 'bg-[#252932]' : 'bg-[#1f2937]'
          )}
        />
        <div className="flex gap-2">
          <div
            className={cn(
              'h-4 flex-1 rounded',
              isTechnicalPrecision ? 'bg-[#2d313a]' : 'bg-[#374151]'
            )}
          />
          <div
            className={cn(
              'h-4 w-16 rounded',
              isTechnicalPrecision ? 'bg-[#00F0FF]/30' : 'bg-[#10b981]/30'
            )}
          />
        </div>
      </div>
      
      {/* Theme Info */}
      <div className="mt-2">
        <div className="font-medium text-foreground">
          {isTechnicalPrecision ? 'Technical Precision' : 'Obsidian & Emerald'}
        </div>
        <div className="text-xs text-muted-foreground">
          {isTechnicalPrecision
            ? 'Electric Cyan accents, high contrast'
            : 'Deep blacks, emerald accents'}
        </div>
      </div>
    </button>
  );
}
