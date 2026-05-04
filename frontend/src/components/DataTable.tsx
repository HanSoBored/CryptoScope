'use client';

import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@/components/ui/table';
import { Skeleton } from '@/components/ui/skeleton';
import { cn } from '@/lib/utils';
import { ArrowUpDown } from 'lucide-react';

export interface Column<T> {
  key: string;
  header: string;
  sortable?: boolean;
  render?: (item: T) => React.ReactNode;
  className?: string;
}

interface DataTableProps<T> {
  columns: readonly Column<T>[];
  data: readonly T[];
  isLoading?: boolean;
  sortKey?: string;
  sortDirection?: 'asc' | 'desc';
  onSort?: (key: string) => void;
  emptyMessage?: string;
  className?: string;
}

export function DataTable<T>({
  columns,
  data,
  isLoading = false,
  sortKey,
  sortDirection = 'asc',
  onSort,
  emptyMessage = 'No data available',
  className,
}: DataTableProps<T>) {
  if (isLoading) {
    return (
      <div className={cn('rounded-md border', className)}>
        <Table>
          <TableHeader>
            <TableRow>
              {columns.map((column) => (
                <TableHead key={column.key} className={column.className}>
                  <Skeleton className="h-4 w-20" />
                </TableHead>
              ))}
            </TableRow>
          </TableHeader>
          <TableBody>
            {Array.from({ length: 5 }).map((_, index) => (
              <TableRow key={index}>
                {columns.map((column) => (
                  <TableCell key={column.key} className={column.className}>
                    <Skeleton className="h-4 w-full" />
                  </TableCell>
                ))}
              </TableRow>
            ))}
          </TableBody>
        </Table>
      </div>
    );
  }

  if (data.length === 0) {
    return (
      <div className={cn('rounded-md border p-8 text-center text-muted-foreground', className)}>
        {emptyMessage}
      </div>
    );
  }

  const handleSort = (key: string) => {
    if (onSort) {
      onSort(key);
    }
  };

  const getValue = (item: T, key: string): unknown => {
    if (key.includes('.')) {
      return key.split('.').reduce((acc, part) => {
        return acc && typeof acc === 'object' ? (acc as Record<string, unknown>)[part] : undefined;
      }, item as unknown);
    }
    return (item as Record<string, unknown>)[key];
  };

  return (
    <div className={cn('rounded-md border', className)}>
      <Table>
        <TableHeader>
          <TableRow>
            {columns.map((column) => (
              <TableHead
                key={column.key}
                className={cn(
                  column.sortable && 'cursor-pointer hover:bg-muted/50',
                  column.className
                )}
                onClick={() => column.sortable && handleSort(column.key)}
              >
                <div className="flex items-center gap-2">
                  {column.header}
                  {column.sortable && (
                    <ArrowUpDown
                      className={cn(
                        'h-4 w-4 transition-transform',
                        sortKey === column.key && sortDirection === 'desc' && 'rotate-180'
                      )}
                    />
                  )}
                </div>
              </TableHead>
            ))}
          </TableRow>
        </TableHeader>
        <TableBody>
          {data.map((item, index) => (
            <TableRow key={index}>
              {columns.map((column) => (
                <TableCell key={column.key} className={column.className}>
                  {column.render
                    ? column.render(item)
                    : String(getValue(item, column.key) ?? '')}
                </TableCell>
              ))}
            </TableRow>
          ))}
        </TableBody>
      </Table>
    </div>
  );
}
