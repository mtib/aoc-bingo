import { createFileRoute } from '@tanstack/react-router'

export const Route = createFileRoute('/')({
    component: App,
})

function App() {
    return (
        <>
            <p>Welcome to AoC Bingo. This website allows you to create <em>custom collections of puzzles</em> from the real Advent of Code for your private leaderboards.</p>
        </>
    )
}
