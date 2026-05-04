'use client';

import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { cn } from '@/lib/utils';
import { TrendingUp, TrendingDown, Minus, type LucideIcon } from 'lucide-react';

interface StatCardProps {
  title: string;
  value: string | number;
  description?: string;
  trend?: 'up' | 'down' | 'neutral';
  trendValue?: string;
  icon?: React.ReactNode;
  className?: string;
  size?: 'default' | 'sm' | 'lg';
}

export function StatCard({
  title,
  value,
  description,
  trend,
  trendValue,
  icon,
  className,
  size = 'default',
}: StatCardProps) {
  const sizeClasses = {
    default: '',
    sm: 'p-3',
    lg: 'p-6',
  };

  const getTrendConfig = () => {
    switch (trend) {
      case 'up':
        return {
          icon: TrendingUp as LucideIcon,
          color: 'text-emerald-500',
          bgColor: 'bg-emerald-500/10',
        };
      case 'down':
        return {
          icon: TrendingDown as LucideIcon,
          color: 'text-red-500',
          bgColor: 'bg-red-500/10',
        };
      case 'neutral':
        return {
          icon: Minus as LucideIcon,
          color: 'text-muted-foreground',
          bgColor: 'bg-muted',
        };
      default:
        return null;
    }
  };

  const config = getTrendConfig();

  return (
    <Card className={cn(sizeClasses[size], className)}>
      <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
        <CardTitle className={cn('text-sm font-medium', size === 'sm' && 'text-xs')}>
          {title}
        </CardTitle>
        {icon && <div className="text-muted-foreground">{icon}</div>}
      </CardHeader>
      <CardContent>
        <div className={cn('font-bold', size === 'lg' ? 'text-3xl' : size === 'sm' ? 'text-xl' : 'text-2xl')}>
          {value}
        </div>
        {description && (
          <p className={cn('text-xs text-muted-foreground mt-1', size === 'sm' && 'text-[10px]')}>
            {description}
          </p>
        )}
        {trend && trendValue && config && (
          <div className="flex items-center gap-1 mt-2">
            <div className={cn('rounded-full p-1', config.bgColor)}>
              <config.icon className={cn('h-3 w-3', config.color)} />
            </div>
            <span className={cn('text-xs font-medium', config.color)}>{trendValue}</span>
          </div>
        )}
      </CardContent>
    </Card>
  );
}
