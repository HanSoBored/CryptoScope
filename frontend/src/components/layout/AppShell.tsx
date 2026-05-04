'use client';

import { cn } from '@/lib/utils';
import { Sidebar } from './Sidebar';
import { TopBar } from './TopBar';
import { useState, type ReactNode } from 'react';
import {
  Sheet,
  SheetContent,
  SheetTitle,
} from '@/components/ui/sheet';

interface AppShellProps {
  children: ReactNode;
  className?: string;
  connectionStatus?: 'connected' | 'connecting' | 'disconnected' | 'error';
  onSearch?: (query: string) => void;
}

/**
 * AppShell - Main application layout wrapper
 * 
 * Features:
 * - Flex layout: Sidebar (fixed width) + Main content (flexible)
 * - Full height (h-screen)
 * - Dark theme ready
 * - Responsive design with mobile drawer
 * - High-density professional trading UI
 * 
 * Layout Structure:
 * - Desktop: Sidebar (256px) + Main Content (flexible)
 * - Mobile: TopBar only, sidebar in drawer
 */
export function AppShell({
  children,
  className,
  connectionStatus = 'connected',
  onSearch,
}: AppShellProps) {
  const [mobileOpen, setMobileOpen] = useState(false);

  return (
    <div className={cn('flex h-screen bg-background', className)}>
      {/* Desktop Sidebar */}
      <Sidebar
        connectionStatus={connectionStatus}
        className="hidden md:flex"
      />

      {/* Mobile Sidebar Drawer */}
      <Sheet open={mobileOpen} onOpenChange={setMobileOpen}>
        <SheetContent side="left" className="w-64 p-0">
          <SheetTitle className="sr-only">Navigation Menu</SheetTitle>
          <Sidebar
            connectionStatus={connectionStatus}
            className="border-r-0"
          />
        </SheetContent>
      </Sheet>

      {/* Main Content Area */}
      <div className="flex-1 flex flex-col min-w-0">
        {/* Top Bar */}
        <TopBar
          connectionStatus={connectionStatus}
          onMenuClick={() => setMobileOpen(true)}
          onSearch={onSearch}
        />

        {/* Page Content */}
        <main className="flex-1 overflow-y-auto">
          <div className="container mx-auto px-4 py-6 md:px-6 md:py-8">
            {children}
          </div>
        </main>
      </div>
    </div>
  );
}
