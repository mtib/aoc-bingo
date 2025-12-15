import { useLeaderboardContext } from '@/contexts/LeaderboardContext'
import { createFileRoute, useNavigate } from '@tanstack/react-router'
import { useMemo, useState } from 'react';
import { createGame } from '@/lib/api';

export const Route = createFileRoute('/create')({
    component: RouteComponent,
})

function RouteComponent() {
    const {
        boardId,
        setBoardId,
        sessionToken,
        setSessionToken,
        gameMemberships,
        setGameMemberships,
    } = useLeaderboardContext();

    const [isCreating, setIsCreating] = useState(false);
    const [error, setError] = useState<string | null>(null);
    const navigate = useNavigate();

    const validInput = useMemo(() => {
        return boardId.trim().length > 0 && sessionToken.length == 128 && /^[a-f0-9]+$/.test(sessionToken) && /^\d+$/.test(boardId);
    }, [boardId, sessionToken]);

    const handleCreate = async () => {
        setIsCreating(true);
        setError(null);
        try {
            const game = await createGame(parseInt(boardId), sessionToken);
            // Add to memberships if not already present
            if (!gameMemberships.includes(game.id)) {
                setGameMemberships([...gameMemberships, game.id]);
            }
            // Navigate to the new game
            navigate({ to: '/game/$id', params: { id: game.id } });
        } catch (err) {
            setError(err instanceof Error ? err.message : 'Failed to create game');
            setIsCreating(false);
        }
    };

    return (
        <>
            <h1>Create a new game</h1>
            <div>
                <p>
                    Create a new leaderboard by entering a Board ID and Session Token.
                    Note, the session token will be stored. Do not provide me with your main account's session token if you care about security.
                    Consider reviewing the source code on <a href="https://github.com/mtib/aoc-bingo">GitHub</a> or self-host if you have concerns.
                </p>
                <p>
                    The session token will not be exposed to other users or sent anywhere other than the Advent of Code servers to fetch data.
                    The server will also identify itself via the User-Agent header and follow the 900s rate limit per leaderboard.
                </p>
            </div>
            <h2>Settings</h2>
            <div className="flex flex-col gap-4">
                <p>Leaderboard id: <input type="text" value={boardId} onChange={(e) => setBoardId(e.target.value.trim())} /></p>
                <p>Session token: <input type="text" value={sessionToken} onChange={(e) => setSessionToken(e.target.value.trim())} /></p>
                {error && <p className="text-red-500">{error}</p>}
                <p><button disabled={!validInput || isCreating} onClick={handleCreate}>
                    {isCreating ? 'Creating...' : 'Create'}
                </button></p>
            </div>
        </>
    )
}
