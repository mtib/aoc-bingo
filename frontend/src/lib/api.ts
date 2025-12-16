const BACKEND_URL = import.meta.env.VITE_BACKEND_URL || 'http://localhost:8000';

export interface GameDto {
    id: string;
    leaderboard_id: number;
    session_token: string;
    created_at: string;
    updated_at: string;
}

export interface GameMembershipDto {
    id: number;
    game_id: string;
    member_id: number;
    member_name: string;
    created_at: string;
}

export interface GameLeaderboardMemberDto {
    id: number;
    name: string;
}

export interface PuzzleDate {
    year: number;
    day: number;
}

export type AocPart = 'One' | 'Two';

export interface AocPuzzle {
    date: PuzzleDate;
    part: AocPart;
}

export interface CreateGameResponse {
    game: GameDto;
}

export interface GetGameMembersResponse {
    possible_members: GameLeaderboardMemberDto[];
    members: GameMembershipDto[];
}

export interface GetAllPuzzlesResponse {
    puzzles: AocPuzzle[];
    members: GameMembershipDto[];
    game_id: string;
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

export async function getGameMembers(gameId: string): Promise<GetGameMembersResponse> {
    const response = await fetch(`${BACKEND_URL}/game/${gameId}/members`, {
        method: 'GET',
        headers: {
            'Content-Type': 'application/json',
        },
    });

    if (!response.ok) {
        const error = await response.text();
        throw new Error(`Failed to get game members: ${error}`);
    }

    return await response.json();
}

export async function getGamePuzzles(gameId: string): Promise<GetAllPuzzlesResponse> {
    const response = await fetch(`${BACKEND_URL}/game/${gameId}/puzzles/all`, {
        method: 'GET',
        headers: {
            'Content-Type': 'application/json',
        },
    });

    if (!response.ok) {
        const error = await response.text();
        throw new Error(`Failed to get game puzzles: ${error}`);
    }

    return await response.json();
}

export interface CreateMembershipRequest {
    member_id: number;
    member_name: string;
}

export interface CreateMembershipResponse {
    membership: GameMembershipDto;
}

export async function addGameMember(
    gameId: string,
    memberId: number,
    memberName: string
): Promise<GameMembershipDto> {
    const response = await fetch(`${BACKEND_URL}/game/${gameId}/members`, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify({
            member_id: memberId,
            member_name: memberName,
        }),
    });

    if (!response.ok) {
        const error = await response.text();
        throw new Error(`Failed to add game member: ${error}`);
    }

    const data: CreateMembershipResponse = await response.json();
    return data.membership;
}

export async function removeGameMember(gameId: string, memberId: number): Promise<void> {
    const response = await fetch(`${BACKEND_URL}/game/${gameId}/members/${memberId}`, {
        method: 'DELETE',
        headers: {
            'Content-Type': 'application/json',
        },
    });

    if (!response.ok) {
        const error = await response.text();
        throw new Error(`Failed to remove game member: ${error}`);
    }
}

/**
 * Tuple representing a completed puzzle: [Year, Day, Part, CompletionTimestamp]
 */
export type CompletionKey = [number, number, AocPart, string];

/**
 * Response from the completion endpoint
 * Maps member IDs to arrays of completed puzzles with timestamps
 */
export type GetCompletionResponse = Record<number, CompletionKey[]>;

/**
 * Fetches completion data for all members in a game
 * Returns which puzzles each member has completed
 */
export async function getGameCompletion(gameId: string): Promise<GetCompletionResponse> {
    const response = await fetch(`${BACKEND_URL}/game/${gameId}/completion`, {
        method: 'GET',
        headers: {
            'Content-Type': 'application/json',
        },
    });

    if (!response.ok) {
        const error = await response.text();
        throw new Error(`Failed to get game completion: ${error}`);
    }

    return await response.json();
}
