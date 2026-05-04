'use client';

import { useState } from 'react';
import { Key, Save, Shield } from 'lucide-react';
import { Button } from '@/components/ui/button';
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog';
import { PasswordField } from './password-field';

interface APIKeyDialogProps {
  exchange: string;
  open: boolean;
  onOpenChange: (open: boolean) => void;
  existingKeys: { apiKey: string; apiSecret: string; passphrase?: string };
  onSave: (keys: { apiKey: string; apiSecret: string; passphrase?: string }) => void;
}

/**
 * APIKeyDialog - Dialog for editing exchange API keys.
 * Handles API key, secret, and passphrase (for OKX) input.
 */
export function APIKeyDialog({
  exchange,
  open,
  onOpenChange,
  existingKeys,
  onSave,
}: APIKeyDialogProps) {
  const [apiKey, setApiKey] = useState(existingKeys.apiKey);
  const [apiSecret, setApiSecret] = useState(existingKeys.apiSecret);
  const [passphrase, setPassphrase] = useState(existingKeys.passphrase || '');

  const handleOpenChange = (newOpen: boolean) => {
    if (newOpen) {
      setApiKey(existingKeys.apiKey);
      setApiSecret(existingKeys.apiSecret);
      setPassphrase(existingKeys.passphrase || '');
    }
    onOpenChange(newOpen);
  };

  const handleSave = () => {
    onSave({ apiKey, apiSecret, passphrase: exchange === 'OKX' ? passphrase : undefined });
    onOpenChange(false);
  };

  const hasChanges =
    apiKey !== existingKeys.apiKey ||
    apiSecret !== existingKeys.apiSecret ||
    (exchange === 'OKX' && passphrase !== (existingKeys.passphrase || ''));

  return (
    <Dialog open={open} onOpenChange={handleOpenChange}>
      <DialogContent className="sm:max-w-md">
        <DialogHeader>
          <DialogTitle className="flex items-center gap-2">
            <Key className="h-4 w-4" />
            {exchange} API Keys
          </DialogTitle>
          <DialogDescription>
            Store your {exchange} API credentials securely. Keys are stored locally and never sent to our servers.
          </DialogDescription>
        </DialogHeader>

        <div className="space-y-4 py-4">
          <PasswordField 
            label="API Key" 
            value={apiKey} 
            onChange={setApiKey} 
            placeholder="Enter your API key" 
          />
          <PasswordField 
            label="API Secret" 
            value={apiSecret} 
            onChange={setApiSecret} 
            placeholder="Enter your API secret" 
          />
          {exchange === 'OKX' && (
            <PasswordField 
              label="Passphrase" 
              value={passphrase} 
              onChange={setPassphrase} 
              placeholder="Enter your passphrase" 
            />
          )}

          <div className="rounded-md border border-emerald-500/50 bg-emerald-500/10 p-3">
            <div className="flex items-start gap-2">
              <Shield className="mt-0.5 h-4 w-4 text-emerald-500" />
              <div className="text-xs text-emerald-200">
                <strong>Encrypted Storage:</strong> API keys are encrypted with AES-256-GCM and stored in your browser&apos;s localStorage.
                The encryption key is stored in sessionStorage and cleared when you close the tab.
                For best security, still use <strong>read-only permissions only</strong> (no trading or withdrawals).
              </div>
            </div>
          </div>
        </div>

        <DialogFooter>
          <Button variant="outline" onClick={() => onOpenChange(false)}>
            Cancel
          </Button>
          <Button onClick={handleSave} disabled={!hasChanges || !apiKey || !apiSecret || (exchange === 'OKX' && !passphrase)}>
            <Save className="mr-2 h-4 w-4" />
            Save Keys
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
