import { createFileRoute, Link } from '@tanstack/react-router'
import { useLeaderboardContext } from '@/contexts/LeaderboardContext'

export const Route = createFileRoute('/games')({
    component: RouteComponent,
})

function RouteComponent() {
    const { gameMemberships } = useLeaderboardContext();

    if (!gameMemberships) return null;

    return (
        <>
            <h1>Games</h1>

            {gameMemberships.length === 0 ? (
                <p>You're not part of any games. You can <Link to="/create">create one</Link>.</p>
            ) : (
                <>
                    <p>You can create games <Link to="/create">here</Link>. Once created you can add players to them.</p>
                    <h2>Your Games</h2>
                    <ul>
                        {gameMemberships.map((game) => (
                            <li key={game.id}>
                                Game{' '}
                                <Link to="/game/$id" params={{ id: game.id }}>
                                    {game.id}
                                </Link>
                                {game.admin && <>
                                    {' '}<span className='quiet'>(admin)</span>
                                </>}
                            </li>
                        ))}
                    </ul>
                </>
            )}
        </>
    )
}
