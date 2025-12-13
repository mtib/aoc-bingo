'use client';

import { ReactNode } from 'react';
import { LeaderboardProvider } from './LeaderboardContext';

export function ClientProviders({ children }: { children: ReactNode }) {
  return (
    <LeaderboardProvider>
      {children}
    </LeaderboardProvider>
  );
}
