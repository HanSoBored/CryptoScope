'use client'

import * as React from 'react'
import { technicalPrecision, obsidianEmerald, type StitchTokens } from './design-tokens'

const STORAGE_KEY = 'cryptoscope-theme'

export type ThemeName = 'technical-precision' | 'obsidian-emerald'

interface StitchContextType {
  theme: ThemeName
  tokens: StitchTokens
  setTheme: (theme: ThemeName) => void
  toggleTheme: () => void
}

const StitchContext = React.createContext<StitchContextType | undefined>(undefined)

const themes: Record<ThemeName, StitchTokens> = {
  'technical-precision': technicalPrecision,
  'obsidian-emerald': obsidianEmerald,
}

function getInitialTheme(defaultTheme: ThemeName = 'technical-precision'): ThemeName {
  if (typeof window === 'undefined') {
    return defaultTheme
  }
  
  const stored = localStorage.getItem(STORAGE_KEY)
  if (stored && (stored === 'technical-precision' || stored === 'obsidian-emerald')) {
    return stored
  }
  
  return defaultTheme
}

interface StitchProviderProps {
  children: React.ReactNode
  defaultTheme?: ThemeName
}

export function StitchProvider({ children, defaultTheme = 'technical-precision' }: StitchProviderProps) {
  const [theme, setThemeState] = React.useState<ThemeName>(() => getInitialTheme(defaultTheme))

  const setTheme = React.useCallback((newTheme: ThemeName) => {
    setThemeState(newTheme)
    localStorage.setItem(STORAGE_KEY, newTheme)
  }, [])

  const toggleTheme = React.useCallback(() => {
    setTheme(theme === 'technical-precision' ? 'obsidian-emerald' : 'technical-precision')
  }, [theme, setTheme])

  const tokens = themes[theme]

  React.useEffect(() => {
    document.documentElement.setAttribute('data-stitch-theme', theme)
  }, [theme])

  return (
    <StitchContext.Provider
      value={{
        theme,
        tokens,
        setTheme,
        toggleTheme,
      }}
    >
      {children}
    </StitchContext.Provider>
  )
}

export function useStitch() {
  const context = React.useContext(StitchContext)
  if (context === undefined) {
    throw new Error('useStitch must be used within a StitchProvider')
  }
  return context
}

/**
 * Hook to get current theme tokens
 */
export function useStitchTokens(): StitchTokens {
  const { tokens } = useStitch()
  return tokens
}

/**
 * Hook to get current theme name
 */
export function useStitchTheme(): ThemeName {
  const { theme } = useStitch()
  return theme
}
