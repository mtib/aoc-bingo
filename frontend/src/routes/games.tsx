import { createFileRoute, Link } from '@tanstack/react-router'
import { useLeaderboardContext } from '@/contexts/LeaderboardContext'

export const Route = createFileRoute('/games')({
    component: RouteComponent,
})

function RouteComponent() {
    const { gameMemberships } = useLeaderboardContext();

    return (
        <>
            <h1>Games</h1>

            {gameMemberships.length === 0 ? (
                <p>You're not part of any games. You can <Link to="/create">create one</Link>.</p>
            ) : (
                <>
                    <h2>Your Games</h2>
                    <ul>
                        {gameMemberships.map((gameId) => (
                            <li key={gameId}>
                                <Link to="/game/$id" params={{ id: gameId }}>
                                    Game {gameId}
                                </Link>
                            </li>
                        ))}
                    </ul>
                    <p><Link to="/create">Create another game</Link></p>
                </>
            )}
        </>
    )
}
