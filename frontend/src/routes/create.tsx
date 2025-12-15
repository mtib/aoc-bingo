import { useLeaderboardContext } from '@/contexts/LeaderboardContext'
import { createFileRoute } from '@tanstack/react-router'

export const Route = createFileRoute('/create')({
    component: RouteComponent,
})

function RouteComponent() {
    const leaderboard = useLeaderboardContext();
    return (
        <>
            <div>Hello "/create"!</div>
            <div>{JSON.stringify(leaderboard)}</div>
            <div>
                <p>Here's an input box: <input type="text" /></p>
            </div>
        </>
    )
}
