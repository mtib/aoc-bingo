import { createFileRoute, Link } from '@tanstack/react-router'

export const Route = createFileRoute('/about')({
    component: RouteComponent,
})

function RouteComponent() {
    return (
        <>
            <section className="mb-4">
                <h2>What is this?</h2>
                <p>
                    AoC Bingo is a tool for creating custom collections of puzzles from{' '}
                    <a href="https://adventofcode.com/">Advent of Code</a> for your private
                    leaderboards. Instead of completing all 25 days in order, you can create
                    games with randomized puzzle selections to add variety and challenge to your
                    AoC experience.
                </p>
            </section>

            <section className="mb-4">
                <h2>Features</h2>
                <ul>
                    <li>Create custom games with your AoC leaderboard</li>
                    <li>Generate randomized puzzle collections</li>
                    <li>Track progress across your games</li>
                    <li>Share games with your private leaderboard members</li>
                </ul>
            </section>

            <section className="mb-4">
                <h2>Getting Started</h2>
                <p>
                    To get started, <Link to="/create">create a game</Link> by entering your
                    Advent of Code leaderboard details. You'll need your leaderboard ID and
                    session token from adventofcode.com.
                </p>
            </section>

            <section className="mb-4">
                <h2>Open Source</h2>
                <p>
                    This project is open source and available on{' '}
                    <a href="https://github.com/mtib/aoc-bingo">GitHub</a>. Contributions and
                    feedback are welcome!
                </p>
            </section>
        </>
    )
}
