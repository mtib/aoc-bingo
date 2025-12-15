import { createFileRoute } from '@tanstack/react-router'
import _ from 'lodash';
import { useMemo } from 'react';

export const Route = createFileRoute('/game/$id')({
    component: RouteComponent,
})

function RouteComponent() {
    const { id } = Route.useParams();
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

    const puzzleCount = 50;
    return (
        <>
            <div>This is game room id={id}</div>

            <div className="py-4">
                <div className='pre'>
                    {" ".repeat(members.length.toString().length + maxScore.toString().length + 3)}{_.times(puzzleCount, (i) => {
                        return (
                            <ColumnText key={i} year={2024 + Math.floor(i / 25)} day={i % 25 + 1} />
                        )
                    })}
                </div>

                {sortedMembers.map((member, index) => (
                    <div key={member.id} className='pre'>
                        <Position index={index} members={sortedMembers} />{member.score.toString().padStart(maxScore.toString().length, ' ')}{' '}
                        {_.times(puzzleCount, (_i) => {
                            const solved = Math.random() < member.score / (maxScore + 200);
                            const firstOnly = Math.random() < 0.1;
                            return (
                                solved ? (<span className={firstOnly ? 'first-only' : 'gold'}>*</span>) : ' '
                            )
                        })}
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

function ColumnText({ year, day }: { year: number, day: number }) {
    const text = `${year % 100} ${day > 9 ? '' : ' '}${day}`;
    return (
        <div className="inline-block">
            <a href="">
                {text.split('').join('\n')}
            </a>
        </div>
    )
}