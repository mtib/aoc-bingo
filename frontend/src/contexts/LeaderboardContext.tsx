'use client';

import { createContext, useContext, useState, useEffect, type ReactNode } from 'react';

interface LeaderboardContextType {
    year: string;
    setYear: (value: string) => void;
    boardId: string;
    setBoardId: (value: string) => void;
    sessionToken: string;
    setSessionToken: (value: string) => void;
    gameMemberships: string[];
    setGameMemberships: (value: string[]) => void;
}

const LeaderboardContext = createContext<LeaderboardContextType | undefined>(undefined);

export function LeaderboardProvider({ children }: { children: ReactNode }) {
    const [year, setYearState] = useState('2025');
    const [boardId, setBoardIdState] = useState('');
    const [sessionToken, setSessionTokenState] = useState('');
    const [gameMemberships, setGameMembershipsState] = useState<string[]>([]);

    // Initialize from localStorage on client mount
    useEffect(() => {
        setYearState(localStorage.getItem('leaderboard-year') || '2025');
        setBoardIdState(localStorage.getItem('leaderboard-boardId') || '');
        setSessionTokenState(localStorage.getItem('leaderboard-sessionToken') || '');
        setGameMembershipsState(JSON.parse(localStorage.getItem('leaderboard-gameMemberships') || '[]') as string[]);
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

    const setGameMemberships = (value: string[]) => {
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