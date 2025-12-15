import { Link } from '@tanstack/react-router'

export default function Header() {

  return (
    <>
      <div className="flex gap-3 align-center mb-3">
        <span>AOC-bingo</span>
        <div className="flex flex-wrap gap-x-3 max-w-[400px]">
          <Link to="/">
            Home
          </Link>
          <Link to="/create">
            Create
          </Link>
          <a href="https://github.com/mtib/aoc-bingo">GitHub</a>
          <button>
            About
          </button>
          <button onClick={() => {
            console.log("clicked");
          }}>Console.log</button>
        </div>
      </div>
    </>
  )
}
