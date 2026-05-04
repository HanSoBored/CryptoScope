'use client';

import { cn } from '@/lib/utils';
import { StatusPip } from '@/components/stitch/StatusPip';
import { getStatusLabel } from '@/lib/status';
import Link from 'next/link';
import { usePathname } from 'next/navigation';
import {
  LayoutDashboard,
  Table,
  BarChart3,
  GitCompare,
  Settings,
  type LucideIcon,
} from 'lucide-react';

interface NavItem {
  label: string;
  href: string;
  icon: LucideIcon;
  priority?: boolean;
}

const NAV_ITEMS: NavItem[] = [
  { label: 'Screener', href: '/screener', icon: LayoutDashboard, priority: true },
  { label: 'Symbols', href: '/symbols', icon: Table, priority: true },
  { label: 'Stats', href: '/stats', icon: BarChart3, priority: true },
  { label: 'Compare', href: '/compare', icon: GitCompare, priority: true },
  { label: 'Settings', href: '/settings', icon: Settings, priority: true },
];

interface SidebarProps {
  className?: string;
  connectionStatus?: 'connected' | 'connecting' | 'disconnected' | 'error';
}

/**
 * Sidebar - Left navigation panel
 * 
 * Features:
 * - Fixed width sidebar with navigation links
 * - Active state highlighting
 * - Connection status indicator
 * - Collapsible on mobile (hidden by default on small screens)
 * - High-density professional trading UI
 */
export function Sidebar({ className, connectionStatus = 'connected' }: SidebarProps) {
  const pathname = usePathname();

  return (
    <aside
      className={cn(
        'hidden md:flex md:flex-col',
        'w-64 border-r bg-card',
        'h-screen sticky top-0',
        className
      )}
    >
      {/* Logo Section */}
      <div className="h-14 flex items-center px-4 border-b">
        <Link href="/" className="flex items-center gap-2">
          <div className="h-8 w-8 rounded-md bg-gradient-to-br from-cyan-500 to-blue-600 flex items-center justify-center">
            <span className="text-white font-bold text-sm">CS</span>
          </div>
          <span className="font-semibold text-lg tracking-tight">CryptoScope</span>
        </Link>
      </div>

      {/* Navigation Links */}
      <nav className="flex-1 overflow-y-auto py-4 px-2">
        <ul className="space-y-1">
          {NAV_ITEMS.map((item) => {
            const Icon = item.icon;
            const isActive = pathname === item.href;

            return (
              <li key={item.href}>
                <Link
                  href={item.href}
                  className={cn(
                    'flex items-center gap-3 rounded-md px-3 py-2 text-sm font-medium transition-colors',
                    isActive
                      ? 'bg-secondary text-secondary-foreground'
                      : 'text-muted-foreground hover:bg-muted hover:text-foreground'
                  )}
                >
                  <Icon className="h-4 w-4 shrink-0" />
                  {item.label}
                  {item.priority && (
                    <span className="ml-auto text-[10px] text-muted-foreground">P1</span>
                  )}
                </Link>
              </li>
            );
          })}
        </ul>
      </nav>

      {/* Connection Status */}
      <div className="p-4 border-t">
        <div className="flex items-center gap-2 text-xs text-muted-foreground">
          <StatusPip variant={connectionStatus} size="sm" />
          <span>{getStatusLabel(connectionStatus)}</span>
        </div>
      </div>
    </aside>
  );
}
