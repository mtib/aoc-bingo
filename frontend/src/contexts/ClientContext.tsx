import React, { type ReactNode } from 'react';
import { LeaderboardProvider } from './LeaderboardContext';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';

interface ClientContextProviderProps {
    children: ReactNode;
}

const queryClient = new QueryClient()

export const ClientContextProvider: React.FC<ClientContextProviderProps> = ({ children }) => {
    return (
        <QueryClientProvider client={queryClient}>
            <LeaderboardProvider>
                {children}
            </LeaderboardProvider>
        </QueryClientProvider>
    );
};