'use client';

import { createContext, useContext, ReactNode, useState, useEffect } from 'react';

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
  const [year, setYearState] = useState('2025');
  const [boardId, setBoardIdState] = useState('');
  const [sessionToken, setSessionTokenState] = useState('');
  const [isClient, setIsClient] = useState(false);

  // Initialize from localStorage on client mount
  useEffect(() => {
    setIsClient(true);
    setYearState(localStorage.getItem('leaderboard-year') || '2025');
    setBoardIdState(localStorage.getItem('leaderboard-boardId') || '');
    setSessionTokenState(localStorage.getItem('leaderboard-sessionToken') || '');
  }, []);

  // Persist to localStorage when values change
  const setYear = (value: string) => {
    setYearState(value);
    if (isClient) {
      localStorage.setItem('leaderboard-year', value);
    }
  };

  const setBoardId = (value: string) => {
    setBoardIdState(value);
    if (isClient) {
      localStorage.setItem('leaderboard-boardId', value);
    }
  };

  const setSessionToken = (value: string) => {
    setSessionTokenState(value);
    if (isClient) {
      localStorage.setItem('leaderboard-sessionToken', value);
    }
  };

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
