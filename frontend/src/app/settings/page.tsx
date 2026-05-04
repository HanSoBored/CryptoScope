'use client';

import { useEffect } from 'react';
import { useStitch } from '@/lib/stitch';
import { StitchCard, StitchCardHeader, StitchCardContent } from '@/components/stitch';
import { Button } from '@/components/ui/button';
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select';
import { Badge } from '@/components/ui/badge';
import { RefreshCw, LayoutGrid, Check } from 'lucide-react';
import { useSaveIndicator } from '@/hooks/useSaveIndicator';
import { ThemePreview, APIKeyForm, SecurityBanner } from '@/components/settings';
import { useSettingsStorage } from '@/hooks/use-settings-storage';
import { EXCHANGES, REFRESH_OPTIONS, type ExchangeName } from '@/lib/settings-storage';

// Main Page Component

export default function SettingsPage() {
  const { theme, setTheme } = useStitch();
  const {
    apiKeys,
    refreshInterval,
    denseMode,
    isLoaded,
    updateAPIKeys,
    updateRefreshInterval,
    updateDenseMode,
  } = useSettingsStorage();
  const { isSaved, markSaved } = useSaveIndicator();

  // Mark as saved when settings are loaded
  useEffect(() => {
    if (isLoaded) {
      markSaved();
    }
  }, [isLoaded, markSaved]);

  const handleUpdateAPIKey = async (
    exchange: ExchangeName,
    keys: { apiKey: string; apiSecret: string; passphrase?: string }
  ) => {
    await updateAPIKeys(exchange, keys);
    markSaved();
  };

  const handleRefreshIntervalChange = (value: string | null) => {
    if (!value) return;
    const interval = parseInt(value, 10) as typeof refreshInterval;
    updateRefreshInterval(interval);
    markSaved();
  };

  const handleDenseModeToggle = (enabled: boolean) => {
    updateDenseMode(enabled);
    markSaved();
  };

  if (!isLoaded) {
    return (
      <div className="space-y-6">
        <div className="flex items-center justify-between">
          <div>
            <h1 className="text-3xl font-bold tracking-tight">Settings</h1>
            <p className="text-muted-foreground">
              Loading your preferences...
            </p>
          </div>
        </div>
      </div>
    );
  }

  return (
    <div className="space-y-6">
      {/* Page Header */}
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-3xl font-bold tracking-tight">Settings</h1>
          <p className="text-muted-foreground">
            Configure your CryptoScope preferences and API connections
          </p>
        </div>
        {isSaved && (
          <Badge variant="secondary" className="bg-emerald-500/20 text-emerald-400 animate-in fade-in">
            <Check className="mr-1 h-3 w-3" />
            Saved
          </Badge>
        )}
      </div>

      {/* Appearance Section */}
      <StitchCard>
        <StitchCardHeader>
          <div>
            <div className="font-heading text-base leading-tight font-semibold text-stitch-on-surface">
              Appearance
            </div>
            <div className="text-sm text-muted-foreground">
              Choose your preferred theme
            </div>
          </div>
        </StitchCardHeader>
        <StitchCardContent>
          <div className="grid gap-4 md:grid-cols-2">
            <ThemePreview
              theme="technical-precision"
              isSelected={theme === 'technical-precision'}
              onSelect={() => { setTheme('technical-precision'); markSaved(); }}
            />
            <ThemePreview
              theme="obsidian-emerald"
              isSelected={theme === 'obsidian-emerald'}
              onSelect={() => { setTheme('obsidian-emerald'); markSaved(); }}
            />
          </div>
        </StitchCardContent>
      </StitchCard>

      {/* Exchange API Keys Section */}
      <StitchCard>
        <StitchCardHeader>
          <div>
            <div className="font-heading text-base leading-tight font-semibold text-stitch-on-surface">
              Exchange API Keys
            </div>
            <div className="text-sm text-muted-foreground">
              Connect your exchange accounts for real-time data
            </div>
          </div>
        </StitchCardHeader>
        <StitchCardContent>
          <SecurityBanner />

          <div className="space-y-4">
            {EXCHANGES.map((exchange) => (
              <APIKeyForm
                key={exchange}
                exchange={exchange}
                keys={apiKeys[exchange.toLowerCase() as keyof typeof apiKeys]}
                onUpdate={(keys) => handleUpdateAPIKey(exchange, keys)}
              />
            ))}
          </div>
        </StitchCardContent>
      </StitchCard>

      {/* Data & Refresh Section */}
      <StitchCard>
        <StitchCardHeader>
          <div>
            <div className="font-heading text-base leading-tight font-semibold text-stitch-on-surface">
              Data & Refresh
            </div>
            <div className="text-sm text-muted-foreground">
              Configure data refresh behavior
            </div>
          </div>
        </StitchCardHeader>
        <StitchCardContent>
          <div className="space-y-4">
            <div className="flex items-center justify-between">
              <div className="space-y-0.5">
                <div className="font-medium text-foreground">Refresh Interval</div>
                <div className="text-sm text-muted-foreground">
                  How often to refresh market data
                </div>
              </div>
              <Select
                value={refreshInterval.toString()}
                onValueChange={handleRefreshIntervalChange}
              >
                <SelectTrigger className="w-40">
                  <SelectValue />
                </SelectTrigger>
                <SelectContent>
                  {REFRESH_OPTIONS.map((option) => (
                    <SelectItem key={option.value} value={option.value.toString()}>
                      {option.label}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
            </div>

            <div className="flex items-center justify-between">
              <div className="space-y-0.5">
                <div className="font-medium text-foreground flex items-center gap-2">
                  <LayoutGrid className="h-4 w-4" />
                  Dense Mode
                </div>
                <div className="text-sm text-muted-foreground">
                  Compact layout for more data on screen
                </div>
              </div>
              <Button
                variant={denseMode ? 'default' : 'outline'}
                size="sm"
                onClick={() => handleDenseModeToggle(!denseMode)}
              >
                {denseMode ? 'Enabled' : 'Disabled'}
              </Button>
            </div>
          </div>
        </StitchCardContent>
      </StitchCard>

      {/* Info Card */}
      <StitchCard>
        <StitchCardContent>
          <div className="flex items-start gap-3 rounded-lg bg-muted/50 p-4">
            <RefreshCw className="mt-0.5 h-5 w-5 text-muted-foreground" />
            <div className="space-y-1 text-sm text-muted-foreground">
              <div className="font-medium text-foreground">Settings are saved locally</div>
               <p>
                 All your settings are stored in your browser&apos;s local storage.
                 They will persist across sessions but won&apos;t sync between devices.
               </p>
            </div>
          </div>
        </StitchCardContent>
      </StitchCard>
    </div>
  );
}
