export type SortDirection = 'asc' | 'desc';

export interface SortState {
  key: string;
  direction: SortDirection;
}

/**
 * Generic sort comparator for any array of objects.
 * Supports dot-notation for nested properties (e.g., "user.name").
 */
export function createSortComparator<T>(
  sortKey: string,
  direction: SortDirection,
): (a: T, b: T) => number {
  return (a: T, b: T) => {
    const getValue = (obj: T, key: string): unknown => {
      if (key.includes('.')) {
        return key.split('.').reduce(
          (acc, part) => acc && typeof acc === 'object'
            ? (acc as Record<string, unknown>)[part]
            : undefined,
          obj as unknown,
        );
      }
      return (obj as Record<string, unknown>)[key];
    };

    const aValue = getValue(a, sortKey);
    const bValue = getValue(b, sortKey);

    if (aValue === undefined || bValue === undefined || aValue === null || bValue === null) return 0;
    if (aValue < bValue) return direction === 'asc' ? -1 : 1;
    if (aValue > bValue) return direction === 'asc' ? 1 : -1;
    return 0;
  };
}

/**
 * Returns sorted array given data + sort state.
 */
export function getSortedData<T>(
  data: T[],
  sortKey: string,
  direction: SortDirection,
): T[] {
  return [...data].sort(createSortComparator(sortKey, direction));
}
