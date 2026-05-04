/**
 * Stitch Design Tokens - Technical Precision Theme
 * 
 * Maps Stitch design system tokens to CSS custom properties
 * for dynamic theming support in CryptoScope.
 */

export interface StitchTokens {
  colors: {
    primary: string
    primaryForeground: string
    secondary: string
    secondaryForeground: string
    background: string
    surface: string
    onSurface: string
    muted: string
    mutedForeground: string
    border: string
    ring: string
    destructive: string
    destructiveForeground: string
  }
  spacing: {
    xs: string
    sm: string
    md: string
    lg: string
    xl: string
  }
  roundness: {
    sm: string
    md: string
    lg: string
  }
  font: {
    family: string
    tabular: string
  }
}

/**
 * Technical Precision theme - default Stitch theme for CryptoScope
 * Features: Electric Cyan primary, high contrast, 4px grid
 */
export const technicalPrecision: StitchTokens = {
  colors: {
    primary: '#00F0FF', // Electric Cyan
    primaryForeground: '#0a0c10',
    secondary: '#4edea3', // Emerald
    secondaryForeground: '#0a0c10',
    background: '#10131a',
    surface: '#1d2026',
    onSurface: '#e1e2eb',
    muted: '#252932',
    mutedForeground: '#9ca3af',
    border: '#2d313a',
    ring: '#00F0FF',
    destructive: '#ef4444',
    destructiveForeground: '#fef2f2',
  },
  spacing: {
    xs: '4px',
    sm: '8px',
    md: '16px',
    lg: '24px',
    xl: '32px',
  },
  roundness: {
    sm: '2px',
    md: '4px',
    lg: '8px',
  },
  font: {
    family: 'Inter, sans-serif',
    tabular: 'Inter, monospace',
  },
}

/**
 * Obsidian & Emerald theme - alternative dark theme
 * Features: Deep blacks, emerald accents, premium feel
 */
export const obsidianEmerald: StitchTokens = {
  colors: {
    primary: '#10b981', // Emerald
    primaryForeground: '#0a0c10',
    secondary: '#34d399', // Light Emerald
    secondaryForeground: '#0a0c10',
    background: '#030712',
    surface: '#111827',
    onSurface: '#e5e7eb',
    muted: '#1f2937',
    mutedForeground: '#9ca3af',
    border: '#374151',
    ring: '#10b981',
    destructive: '#ef4444',
    destructiveForeground: '#fef2f2',
  },
  spacing: {
    xs: '4px',
    sm: '8px',
    md: '16px',
    lg: '24px',
    xl: '32px',
  },
  roundness: {
    sm: '2px',
    md: '4px',
    lg: '8px',
  },
  font: {
    family: 'Inter, sans-serif',
    tabular: 'Inter, monospace',
  },
}

/**
 * Convert tokens to CSS custom properties
 */
export function tokensToCSS(tokens: StitchTokens, selector: string = ':root'): string {
  return `${selector} {
  /* Colors */
  --stitch-primary: ${tokens.colors.primary};
  --stitch-primary-foreground: ${tokens.colors.primaryForeground};
  --stitch-secondary: ${tokens.colors.secondary};
  --stitch-secondary-foreground: ${tokens.colors.secondaryForeground};
  --stitch-background: ${tokens.colors.background};
  --stitch-surface: ${tokens.colors.surface};
  --stitch-on-surface: ${tokens.colors.onSurface};
  --stitch-muted: ${tokens.colors.muted};
  --stitch-muted-foreground: ${tokens.colors.mutedForeground};
  --stitch-border: ${tokens.colors.border};
  --stitch-ring: ${tokens.colors.ring};
  --stitch-destructive: ${tokens.colors.destructive};
  --stitch-destructive-foreground: ${tokens.colors.destructiveForeground};

  /* Spacing */
  --stitch-spacing-xs: ${tokens.spacing.xs};
  --stitch-spacing-sm: ${tokens.spacing.sm};
  --stitch-spacing-md: ${tokens.spacing.md};
  --stitch-spacing-lg: ${tokens.spacing.lg};
  --stitch-spacing-xl: ${tokens.spacing.xl};

  /* Roundness */
  --stitch-radius-sm: ${tokens.roundness.sm};
  --stitch-radius-md: ${tokens.roundness.md};
  --stitch-radius-lg: ${tokens.roundness.lg};

  /* Font */
  --stitch-font-family: ${tokens.font.family};
  --stitch-font-tabular: ${tokens.font.tabular};
}`
}

/**
 * Type-safe token accessor
 */
export function getToken<K extends keyof StitchTokens>(
  tokens: StitchTokens,
  key: K
): StitchTokens[K] {
  return tokens[key]
}
