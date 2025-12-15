const BACKEND_URL = import.meta.env.VITE_BACKEND_URL || 'http://localhost:8000';

export interface GameDto {
    id: string;
    leaderboard_id: number;
    session_token: string;
    created_at: string;
    updated_at: string;
}

export interface CreateGameResponse {
    game: GameDto;
}

export async function createGame(leaderboardId: number, sessionToken: string): Promise<GameDto> {
    const response = await fetch(`${BACKEND_URL}/game`, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify({
            leaderboard_id: leaderboardId,
            session_token: sessionToken,
        }),
    });

    if (!response.ok) {
        const error = await response.text();
        throw new Error(`Failed to create game: ${error}`);
    }

    const data: CreateGameResponse = await response.json();
    return data.game;
}
