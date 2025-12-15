import { createFileRoute, Link } from '@tanstack/react-router'

export const Route = createFileRoute('/games')({
    component: RouteComponent,
})

function RouteComponent() {
    return (
        <>
            <h1>Games</h1>
            <p>You're not part of any games. You can <Link to="/create">create one</Link>.</p>

            <h2>Your Games</h2>
            <p>You are not part of any games yet.</p>

            <p>For testing purposes, see this mocked <Link to={`/game/$id`} params={{ id: '123' }}>game</Link>.</p >
        </>
    )
}
