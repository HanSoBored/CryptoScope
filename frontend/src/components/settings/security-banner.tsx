'use client';

import { Shield } from 'lucide-react';

/**
 * SecurityBanner - Security information banner for API keys section.
 * Displays encryption details and security best practices.
 */
export function SecurityBanner() {
  return (
    <div className="mb-4 rounded-lg border border-emerald-500/50 bg-emerald-500/10 p-4">
      <div className="flex items-start gap-3">
        <div className="mt-0.5 text-emerald-500">
          <Shield className="h-5 w-5" />
        </div>
        <div className="space-y-2">
          <h3 className="text-sm font-semibold text-emerald-400">
            ✓ API Keys Encrypted with AES-256-GCM
          </h3>
          <p className="text-sm text-muted-foreground">
            Your API keys are encrypted before being stored in localStorage. The encryption passphrase 
            is stored in sessionStorage and cleared when you close the browser tab, providing protection 
            against persistent storage attacks.
          </p>
          <ul className="list-inside list-disc space-y-1 text-sm text-muted-foreground">
            <li>Still use <strong>READ-ONLY</strong> API keys (no withdrawal permissions)</li>
            <li>Encryption key is cleared when you close the tab</li>
            <li>Clear browser data when using shared computers</li>
            <li>Use exchange IP whitelist feature when available</li>
            <li>Consider creating dedicated API keys for CryptoScope only</li>
          </ul>
        </div>
      </div>
    </div>
  );
}
