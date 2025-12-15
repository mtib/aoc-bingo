import React, { type ReactNode } from 'react';
import { LeaderboardProvider } from './LeaderboardContext';

interface ClientContextProviderProps {
    children: ReactNode;
}

export const ClientContextProvider: React.FC<ClientContextProviderProps> = ({ children }) => {
    return (
        <LeaderboardProvider>
            {children}
        </LeaderboardProvider>
    );
};