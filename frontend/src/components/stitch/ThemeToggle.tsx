'use client'

import { useStitch } from '@/lib/stitch'
import { Button } from '@/components/ui/button'
import { Sun, Moon } from 'lucide-react'
import { useState, useEffect } from 'react'

export function ThemeToggle() {
  const { theme, toggleTheme } = useStitch()
  const isCyan = theme === 'technical-precision'
  
  // Prevent hydration mismatch by only rendering theme-dependent content after mount
  const [isMounted, setIsMounted] = useState(false)
  
  useEffect(() => {
    setIsMounted(true)
  }, [])
  
  // Render placeholder until mounted to avoid SSR/client mismatch
  if (!isMounted) {
    return (
      <Button variant="ghost" size="sm" disabled className="gap-2">
        <span className="sr-only">Loading theme...</span>
      </Button>
    )
  }

  return (
    <Button
      variant="ghost"
      size="sm"
      onClick={toggleTheme}
      className="gap-2"
      aria-label={`Switch to ${isCyan ? 'Emerald' : 'Cyan'} theme`}
    >
      {isCyan ? (
        <>
          <Moon className="size-4" />
          <span className="hidden sm:inline">Emerald</span>
        </>
      ) : (
        <>
          <Sun className="size-4" />
          <span className="hidden sm:inline">Cyan</span>
        </>
      )}
    </Button>
  )
}
