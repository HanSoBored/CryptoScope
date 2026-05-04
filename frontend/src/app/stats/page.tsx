'use client';

import { useState } from 'react';
import { useStats } from '@/hooks/useStats';
import { StatCard } from '@/components/StatCard';
import { CategoryFilter, type CategoryValue } from '@/components/CategoryFilter';
import { Button } from '@/components/ui/button';
import { StitchCard, StitchCardHeader, StitchCardContent, StatusPip } from '@/components/stitch';
import { Alert, AlertDescription } from '@/components/ui/alert';
import { Badge } from '@/components/ui/badge';
import { CardTitle, CardDescription } from '@/components/ui/card';
import { RefreshCw, Coins } from 'lucide-react';
import { cn } from '@/lib/utils';

export default function StatsPage() {
  const [category, setCategory] = useState<CategoryValue>('all');

  const {
    data: stats = {
      total_count: 0,
      by_category: [],
      by_contract_type: [],
    },
    isLoading,
    error,
    refetch,
    isRefetching,
  } = useStats({
    category: category === 'all' ? undefined : category,
  });

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-3xl font-bold tracking-tight">Statistics</h1>
          <p className="text-muted-foreground">
            Market overview and statistics
          </p>
        </div>
        <Button
          variant="outline"
          onClick={() => refetch()}
          disabled={isRefetching}
        >
          <RefreshCw className={cn('mr-2 h-4 w-4', isRefetching && 'animate-spin')} />
          Refresh
        </Button>
      </div>

      {error && (
        <Alert variant="destructive">
          <AlertDescription>
            Failed to load statistics: {(error as Error).message}
          </AlertDescription>
        </Alert>
      )}

      <StitchCard>
        <StitchCardHeader showBorder={false}>
          <div>
            <CardTitle className="text-sm">Category Filter</CardTitle>
            <CardDescription className="text-xs">
              Filter statistics by contract category
            </CardDescription>
          </div>
        </StitchCardHeader>
        <StitchCardContent variant="dense">
          <CategoryFilter
            value={category}
            onChange={setCategory}
            variant="tabs"
          />
        </StitchCardContent>
      </StitchCard>

      {isLoading ? (
        <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
          {Array.from({ length: 6 }).map((_, i) => (
            <StitchCard key={i}>
              <StitchCardHeader showBorder={false}>
                <div className="h-4 w-24 bg-muted animate-pulse rounded" />
              </StitchCardHeader>
              <StitchCardContent>
                <div className="h-8 w-32 bg-muted animate-pulse rounded" />
              </StitchCardContent>
            </StitchCard>
          ))}
        </div>
      ) : stats ? (
        <>
          <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
            <StatCard
              title="Total Symbols"
              value={stats.total_count}
              description="Across all categories"
              icon={<Coins className="h-4 w-4" />}
            />
          </div>

          <div className="grid gap-4 md:grid-cols-2">
            <StitchCard>
              <StitchCardHeader>
                <div className="flex items-center gap-2">
                  <StatusPip variant={isLoading ? 'connecting' : 'connected'} size="sm" />
                  <span className="text-sm font-medium">Category Breakdown</span>
                </div>
              </StitchCardHeader>
              <StitchCardContent>
                <div className="space-y-3">
                  {stats.by_category.map(({ category, count }) => (
                    <div key={category} className="flex items-center justify-between">
                      <div className="flex items-center gap-2">
                        <Badge variant="secondary" className="capitalize">
                          {category}
                        </Badge>
                      </div>
                      <div className="flex items-center gap-4">
                        <div className="w-32 bg-muted rounded-full h-2">
                          <div
                            className="bg-primary h-2 rounded-full"
                            style={{ width: `${stats.total_count > 0 ? (count / stats.total_count) * 100 : 0}%` }}
                          />
                        </div>
                        <span className="text-sm font-medium w-12 text-right">
                          {count}
                        </span>
                      </div>
                    </div>
                  ))}
                </div>
              </StitchCardContent>
            </StitchCard>

            <StitchCard>
              <StitchCardHeader>
                <div className="flex items-center gap-2">
                  <StatusPip variant={isLoading ? 'connecting' : 'connected'} size="sm" />
                  <span className="text-sm font-medium">Contract Type Breakdown</span>
                </div>
              </StitchCardHeader>
              <StitchCardContent>
                <div className="space-y-3">
                  {stats.by_contract_type.map(({ category, count }) => (
                    <div key={category} className="flex items-center justify-between">
                      <div className="flex items-center gap-2">
                        <Badge variant="outline">
                          {category}
                        </Badge>
                      </div>
                      <div className="flex items-center gap-4">
                        <div className="w-32 bg-muted rounded-full h-2">
                          <div
                            className="bg-primary h-2 rounded-full"
                            style={{ width: `${stats.total_count > 0 ? (count / stats.total_count) * 100 : 0}%` }}
                          />
                        </div>
                        <span className="text-sm font-medium w-12 text-right">
                          {count}
                        </span>
                      </div>
                    </div>
                  ))}
                </div>
              </StitchCardContent>
            </StitchCard>
          </div>
        </>
      ) : (
        <Alert>
          <AlertDescription>
            No statistics available. Make sure the backend API is running.
          </AlertDescription>
        </Alert>
      )}
    </div>
  );
}
