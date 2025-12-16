'use client';

import type { GameEntry } from '@/lib/game.types';
import { createContext, useContext, useState, useEffect, type ReactNode } from 'react';

/**
 * `null` during load
 */
interface LeaderboardContextType {
    year: string | null;
    setYear: (value: string) => void;
    boardId: string | null;
    setBoardId: (value: string) => void;
    sessionToken: string | null;
    setSessionToken: (value: string) => void;
    gameMemberships: GameEntry[] | null;
    setGameMemberships: (value: GameEntry[]) => void;
}

const LeaderboardContext = createContext<LeaderboardContextType | undefined>(undefined);

export function LeaderboardProvider({ children }: { children: ReactNode }) {
    const [year, setYearState] = useState<string | null>(null);
    const [boardId, setBoardIdState] = useState<string | null>(null);
    const [sessionToken, setSessionTokenState] = useState<string | null>(null);
    const [gameMemberships, setGameMembershipsState] = useState<GameEntry[] | null>(null);

    // Initialize from localStorage on client mount
    useEffect(() => {
        setYearState(localStorage.getItem('leaderboard-year') || '2025');
        setBoardIdState(localStorage.getItem('leaderboard-boardId') || '');
        setSessionTokenState(localStorage.getItem('leaderboard-sessionToken') || '');
        setGameMembershipsState(JSON.parse(localStorage.getItem('leaderboard-gameMemberships') || '[]') as GameEntry[]);
    }, []);

    // Persist to localStorage when values change
    const setYear = (value: string) => {
        setYearState(value);
        localStorage.setItem('leaderboard-year', value);
    };

    const setBoardId = (value: string) => {
        setBoardIdState(value);
        localStorage.setItem('leaderboard-boardId', value);
    };

    const setSessionToken = (value: string) => {
        setSessionTokenState(value);
        localStorage.setItem('leaderboard-sessionToken', value);
    };

    const setGameMemberships = (value: GameEntry[]) => {
        setGameMembershipsState(value);
        localStorage.setItem('leaderboard-gameMemberships', JSON.stringify(value));
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
                gameMemberships,
                setGameMemberships,
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