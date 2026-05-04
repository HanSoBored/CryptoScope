'use client';

import { useState } from 'react';
import { Key } from 'lucide-react';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import { cn } from '@/lib/utils';
import { APIKeyDialog } from './api-key-dialog';
import { maskKey } from '@/lib/settings-storage';

interface APIKeyFormProps {
  exchange: string;
  keys: { apiKey: string; apiSecret: string; passphrase?: string };
  onUpdate: (keys: { apiKey: string; apiSecret: string; passphrase?: string }) => void;
}

/**
 * APIKeyForm - Display and edit API keys for an exchange.
 * Shows masked key and provides edit button that opens APIKeyDialog.
 */
export function APIKeyForm({ exchange, keys, onUpdate }: APIKeyFormProps) {
  const [dialogOpen, setDialogOpen] = useState(false);

  return (
    <>
      <div className="flex items-center justify-between rounded-lg border border-stitch-border p-4">
        <div className="flex items-center gap-3">
          <div
            className={cn(
              'flex h-10 w-10 items-center justify-center rounded-lg',
              exchange === 'Bybit' && 'bg-[#1a1a1a]',
              exchange === 'Binance' && 'bg-[#FCD535]',
              exchange === 'OKX' && 'bg-[#000000]'
            )}
          >
            <Key className={cn(
              'h-5 w-5',
              exchange === 'Bybit' && 'text-white',
              exchange === 'Binance' && 'text-black',
              exchange === 'OKX' && 'text-white'
            )} />
          </div>
          <div>
            <div className="font-medium text-foreground">{exchange}</div>
            {keys.apiKey ? (
              <div className="text-sm text-muted-foreground font-mono">
                {maskKey(keys.apiKey)}
              </div>
            ) : (
              <div className="text-sm text-muted-foreground">No API key configured</div>
            )}
          </div>
        </div>
        <div className="flex items-center gap-2">
          {keys.apiKey && (
            <Badge variant="secondary" className="bg-emerald-500/20 text-emerald-400">
              Configured
            </Badge>
          )}
          <Button variant="outline" size="sm" onClick={() => setDialogOpen(true)}>
            {keys.apiKey ? 'Edit' : 'Add'} Key
          </Button>
        </div>
      </div>

      <APIKeyDialog
        exchange={exchange}
        open={dialogOpen}
        onOpenChange={setDialogOpen}
        existingKeys={keys}
        onSave={onUpdate}
      />
    </>
  );
}
