'use client';

import { useState, useMemo } from 'react';
import { useScreener } from '@/hooks/useScreener';
import { DataTable } from '@/components/DataTable';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Tabs, TabsList, TabsTrigger } from '@/components/ui/tabs';
import { Card, CardHeader, CardTitle, CardDescription } from '@/components/ui/card';
import { Alert, AlertDescription } from '@/components/ui/alert';
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
  DialogFooter,
  DialogClose,
} from '@/components/ui/dialog';
import { RefreshCw, Filter, Activity } from 'lucide-react';
import { StatusPip, StitchCard, StitchCardHeader, StitchCardContent } from '@/components/stitch';

import { cn } from '@/lib/utils';
import { getSortedData } from '@/lib/sort';
import { useSortState } from '@/hooks/useSortState';
import { createScreenerColumns } from './columns';

export default function ScreenerPage() {
  const [mode, setMode] = useState<'kline' | 'mark'>('kline');
  const [top, setTop] = useState<number | undefined>(undefined);
  const [minChange, setMinChange] = useState('');
  const { sortKey: sortField, sortDirection, handleSort } = useSortState({
    defaultKey: 'change_percent',
    defaultDirection: 'desc',
  });

  const {
    data: screenerData = [],
    isLoading,
    error,
    refetch,
    isRefetching,
  } = useScreener({
    exchange: 'bybit',
    mode,
    top,
    minChange: minChange ? parseFloat(minChange) : undefined,
  });

  const columns = useMemo(() => createScreenerColumns(), []);

  const data = Array.isArray(screenerData) ? screenerData : [];
  const sortedData = getSortedData(data, sortField, sortDirection);

  const gainers = data.filter((item) => item.change_percent > 0);
  const losers = data.filter((item) => item.change_percent < 0);

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-3xl font-bold tracking-tight">Screener</h1>
          <p className="text-muted-foreground">
            Top gainers and losers in the cryptocurrency market
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
            Failed to load screener data: {(error as Error).message}
          </AlertDescription>
        </Alert>
      )}

      <div className="grid gap-4 md:grid-cols-2">
        <StitchCard>
          <StitchCardHeader showBorder={false}>
            <div className="flex items-center gap-2">
              <StatusPip variant={isLoading ? 'connecting' : 'connected'} size="sm" />
              <span className="text-sm font-medium text-emerald-500">Top Gainers</span>
            </div>
            <Activity className="h-4 w-4 text-muted-foreground" />
          </StitchCardHeader>
          <StitchCardContent variant="dense">
            <div className="text-2xl font-bold">{gainers.length}</div>
            <p className="text-xs text-muted-foreground">
              Symbols with positive change
            </p>
          </StitchCardContent>
        </StitchCard>
        <StitchCard>
          <StitchCardHeader showBorder={false}>
            <div className="flex items-center gap-2">
              <StatusPip variant={isLoading ? 'connecting' : 'connected'} size="sm" />
              <span className="text-sm font-medium text-red-500">Top Losers</span>
            </div>
            <Activity className="h-4 w-4 text-muted-foreground" />
          </StitchCardHeader>
          <StitchCardContent variant="dense">
            <div className="text-2xl font-bold">{losers.length}</div>
            <p className="text-xs text-muted-foreground">
              Symbols with negative change
            </p>
          </StitchCardContent>
        </StitchCard>
      </div>

      <Card>
        <CardHeader>
          <div className="flex items-center justify-between">
            <div>
              <CardTitle>Filters</CardTitle>
              <CardDescription>
                Customize your screener view
              </CardDescription>
            </div>
            <Dialog>
              <DialogTrigger
                render={(props) => (
                  <Button variant="outline" size="sm" {...props}>
                    <Filter className="mr-2 h-4 w-4" />
                    Filter
                  </Button>
                )}
              />
              <DialogContent>
                <DialogHeader>
                  <DialogTitle>Screener Filters</DialogTitle>
                </DialogHeader>
                <div className="space-y-4 py-4">
                  <div className="space-y-2">
                    <label className="text-sm font-medium">Mode</label>
                    <Tabs value={mode} onValueChange={(v) => setMode(v as 'kline' | 'mark')}>
                      <TabsList>
                        <TabsTrigger value="kline">Kline</TabsTrigger>
                        <TabsTrigger value="mark">Mark</TabsTrigger>
                      </TabsList>
                    </Tabs>
                  </div>

                  <div className="space-y-2">
                    <label className="text-sm font-medium">Top (Optional)</label>
                    <Input
                      type="number"
                      value={top || ''}
                      onChange={(e) => {
                        const val = e.target.value;
                        const parsed = val ? parseInt(val, 10) : undefined;
                        setTop(parsed !== undefined && Number.isNaN(parsed) ? undefined : parsed);
                      }}
                      placeholder="All"
                      min={5}
                      max={100}
                    />
                    <p className="text-xs text-muted-foreground">
                      Leave empty to show all symbols
                    </p>
                  </div>

                  <div className="space-y-2">
                    <label className="text-sm font-medium">Min Change %</label>
                    <Input
                      type="number"
                      value={minChange}
                      onChange={(e) => setMinChange(e.target.value)}
                      placeholder="0"
                      className="w-full"
                      step={0.1}
                    />
                  </div>
                </div>
                <DialogFooter>
                  <DialogClose
                    render={(props) => (
                      <Button {...props}>Done</Button>
                    )}
                  />
                </DialogFooter>
              </DialogContent>
            </Dialog>
          </div>
        </CardHeader>
      </Card>

      <DataTable
        columns={columns}
        data={sortedData}
        isLoading={isLoading}
        sortKey={sortField}
        sortDirection={sortDirection}
        onSort={handleSort}
        emptyMessage="No data found matching your criteria"
      />

      {!isLoading && (
        <p className="text-sm text-muted-foreground text-center">
          Showing {data.length} symbol{data.length !== 1 ? 's' : ''} • Mode: {mode}
        </p>
      )}
    </div>
  );
}
