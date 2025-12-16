import { Link } from '@tanstack/react-router'
import Disclaimer from './Disclaimer';

export default function Header() {

  return (
    <>
      <div className="flex gap-3 align-center mb-3 max-w-[55em]">
        <em>AoC Bingo</em>
        <div className="flex flex-wrap gap-x-3 flex-shrink flex-grow">
          <Link to="/">
            Home
          </Link>
          <Link to="/create">
            Create
          </Link>
          <Link to="/games">
            Games
          </Link>
          <Link to="/about">
            About
          </Link>
          <a href="https://github.com/mtib/aoc-bingo">GitHub</a>
        </div>
      </div>
      <Disclaimer className='mb-3' />
    </>
  )
}
