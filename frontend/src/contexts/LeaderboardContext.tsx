'use client';

import { createContext, useContext, ReactNode } from 'react';
import { useStorageState } from 'react-use-storage-state';

interface LeaderboardContextType {
  year: string;
  setYear: (value: string) => void;
  boardId: string;
  setBoardId: (value: string) => void;
  sessionToken: string;
  setSessionToken: (value: string) => void;
}

const LeaderboardContext = createContext<LeaderboardContextType | undefined>(undefined);

export function LeaderboardProvider({ children }: { children: ReactNode }) {
  const [year, setYear] = useStorageState('leaderboard-year', '2025');
  const [boardId, setBoardId] = useStorageState('leaderboard-boardId', '');
  const [sessionToken, setSessionToken] = useStorageState('leaderboard-sessionToken', '');

  return (
    <LeaderboardContext.Provider
      value={{
        year,
        setYear,
        boardId,
        setBoardId,
        sessionToken,
        setSessionToken,
      }}
    >
      {children}
    </LeaderboardContext.Provider>
  );
}

export function useLeaderboardContext() {
  const context = useContext(LeaderboardContext);
  if (context === undefined) {
    throw new Error('useLeaderboardContext must be used within a LeaderboardProvider');
  }
  return context;
}
