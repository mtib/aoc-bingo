import { createFileRoute } from '@tanstack/react-router'
import _ from 'lodash';
import { useMemo, useEffect } from 'react';
import { useLeaderboardContext } from '@/contexts/LeaderboardContext';

export const Route = createFileRoute('/game/$id')({
    component: RouteComponent,
})

function RouteComponent() {
    const { id } = Route.useParams();
    const { gameMemberships, setGameMemberships } = useLeaderboardContext();

    // Add this game to memberships when visited
    useEffect(() => {
        if (!gameMemberships.includes(id)) {
            setGameMemberships([...gameMemberships, id]);
        }
    }, [id, gameMemberships, setGameMemberships]);
    const members = useMemo(() => {
        return _.times(13, (i) => ({
            id: `user_${i}`,
            name: `User ${i}`,
            score: Math.floor(Math.random() * 300),
        }));
    }, []);

    const sortedMembers = useMemo(() => {
        return _.orderBy(members, ['score'], ['desc']);
    }, [members]);

    const maxScore = useMemo(() => {
        return sortedMembers.length > 0 ? sortedMembers[0].score : 0;
    }, [sortedMembers])

    const puzzleCount = 90;
    return (
        <>
            <h1>Game Room {id}</h1>
            <div>This is <em>game room</em> id={id}.</div>
            <h2>Leaderboard</h2>
            <div>
                <div className='pre'>
                    {" ".repeat(members.length.toString().length + maxScore.toString().length + 1)}<div className='inline-block'>{'Y\nY\n\nD\nD\n\nP '}</div>{_.times(puzzleCount, (i) => {
                        return (
                            <ColumnText key={i} year={2024 + Math.floor(i / 2 / 25)} day={Math.floor(i / 2) % 25 + 1} part={i % 2 + 1} />
                        )
                    })}
                </div>

                {sortedMembers.map((member, index) => (
                    <div key={member.id} className='pre'>
                        <Position index={index} members={sortedMembers} />{member.score.toString().padStart(maxScore.toString().length, ' ')}{' '}
                        {_.times(puzzleCount, (_i) => {
                            const solved = Math.random() < member.score / (maxScore + 200);
                            const firstOnly = false;
                            return (
                                solved ? (<span className={firstOnly ? 'first-only' : 'gold'}>*</span>) : ' '
                            )
                        })}
                        {' '}{member.name}
                    </div>
                ))}

            </div>
        </>
    )
}

function Position({ index, members }: { index: number, members: Array<any> }) {
    if (index > 0 && members[index].score === members[index - 1].score) {
        return (
            <>
                {' '.repeat(members.length.toString().length)}{'  '}
            </>
        )
    }
    return (
        <>
            {(index + 1).toString().padStart(members.length.toString().length, ' ')}{') '}
        </>
    )
}

function ColumnText({ year, day, part }: { year: number, day: number, part: number }) {
    const text = `${year % 100} ${day > 9 ? '' : ' '}${day} ${part}`;
    return (
        <div className="inline-block">
            <a href={`https://adventofcode.com/${year}/day/${day}`} target="_blank" rel="noreferrer" title={`AoC ${year} Day ${day}`}>
                {text.split('').join('\n')}
            </a>
        </div>
    )
}