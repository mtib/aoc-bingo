import { createFileRoute } from '@tanstack/react-router'
import _ from 'lodash';
import { useMemo, useEffect, useState } from 'react';
import { useLeaderboardContext } from '@/contexts/LeaderboardContext';
import { getGameMembers, getGamePuzzles, getGameCompletion, addGameMember, removeGameMember, type GameLeaderboardMemberDto, type GameMembershipDto } from '@/lib/api';
import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query';

export const Route = createFileRoute('/game/$id')({
    component: RouteComponent,
})

function RouteComponent() {
    const { id } = Route.useParams();
    const queryClient = useQueryClient();
    const { gameMemberships, setGameMemberships } = useLeaderboardContext();
    const [members, setMembers] = useState<GameMembershipDto[] | null>(null);
    const [possibleMembers, setPossibleMembers] = useState<GameLeaderboardMemberDto[] | null>(null);
    const [scoredMembers, setScoredMembers] = useState<(GameMembershipDto & { score: number })[] | null>(null);
    const [isAdmin, setIsAdmin] = useState(false);

    // Add this game to memberships when visited
    useEffect(() => {
        if (!id || !gameMemberships) return;
        if (!gameMemberships.some(g => g.id === id)) {
            setGameMemberships([...gameMemberships, { id, admin: false }]);
        }
        if (gameMemberships.some(g => g.id === id && g.admin)) {
            setIsAdmin(true);
        }
    }, [id, gameMemberships, setGameMemberships]);

    const { data: gameMembersData } = useQuery({
        queryKey: ['gameMembers', id],
        queryFn: async ({ queryKey }) => {
            const id = queryKey[1];
            if (!id) return;
            const data = await getGameMembers(id);
            data.members.sort((a, b) => b.member_id - a.member_id);
            data.possible_members.sort((a, b) => b.id - a.id);
            return data;
        },
        enabled: !!id,
    })

    useEffect(() => {
        if (!gameMembersData) return;
        setMembers(gameMembersData.members);
        setPossibleMembers(gameMembersData.possible_members);
        setScoredMembers(gameMembersData.members.map(m => ({
            ...m,
            score: 0
        })));
    }, [gameMembersData]);

    const { data: puzzlesData } = useQuery({
        queryKey: ['gamePuzzles', id],
        queryFn: async ({ queryKey }) => {
            const id = queryKey[1];
            if (!id) return;
            const data = await getGamePuzzles(id);
            return data;
        },
        enabled: !!id,
    })

    /**
     * Query completion data for all game members
     * Returns which puzzles each member has completed
     */
    const { data: completionData } = useQuery({
        queryKey: ['gameCompletion', id],
        queryFn: async ({ queryKey }) => {
            const id = queryKey[1];
            if (!id) return;
            const data = await getGameCompletion(id);
            return data;
        },
        enabled: !!id,
        refetchInterval: 900_000,
    })

    /**
     * Update member scores based on completion data
     * Points are awarded for each puzzle based on:
     * - Number of other members who haven't solved it
     * - Number of other members who solved it after this member
     */
    useEffect(() => {
        if (!gameMembersData || !completionData) return;

        const memberIds = gameMembersData.members.map(m => m.member_id);

        setScoredMembers(gameMembersData.members.map(m => {
            let totalScore = 0;
            const memberCompletions = completionData[m.member_id] || [];

            // For each puzzle this member completed
            for (const [year, day, part, timestamp] of memberCompletions) {
                let pointsForPuzzle = 0;

                // Check each other member
                for (const otherId of memberIds) {
                    if (otherId === m.member_id) continue;

                    const otherCompletions = completionData[otherId] || [];
                    const otherCompletion = otherCompletions.find(
                        ([y, d, p]) => y === year && d === day && p === part
                    );

                    if (!otherCompletion) {
                        // Other member hasn't solved it - 1 point
                        pointsForPuzzle++;
                    } else {
                        // Compare timestamps
                        const [, , , otherTimestamp] = otherCompletion;
                        if (new Date(otherTimestamp) > new Date(timestamp)) {
                            // Other member solved it later - 1 point
                            pointsForPuzzle++;
                        }
                    }
                }

                totalScore += pointsForPuzzle;
            }

            return {
                ...m,
                score: totalScore
            };
        }));
    }, [gameMembersData, completionData]);

    const addMemberMutation = useMutation({
        mutationFn: async ({ memberId, memberName }: { memberId: number; memberName: string }) => {
            if (!id) throw new Error('Game ID is required');
            return await addGameMember(id, memberId, memberName);
        },
        onSuccess: () => {
            queryClient.invalidateQueries({ queryKey: ['gameMembers', id] });
            queryClient.invalidateQueries({ queryKey: ['gamePuzzles', id] });
            queryClient.invalidateQueries({ queryKey: ['gameCompletion', id] });
        },
    });

    const removeMemberMutation = useMutation({
        mutationFn: async (memberId: number) => {
            if (!id) throw new Error('Game ID is required');
            return await removeGameMember(id, memberId);
        },
        onSuccess: () => {
            queryClient.invalidateQueries({ queryKey: ['gameMembers', id] });
            queryClient.invalidateQueries({ queryKey: ['gamePuzzles', id] });
            queryClient.invalidateQueries({ queryKey: ['gameCompletion', id] });
        },
    });

    const handleAddMember = (memberId: number, memberName: string) => {
        addMemberMutation.mutate({ memberId, memberName });
    };

    const handleRemoveMember = (memberId: number) => {
        removeMemberMutation.mutate(memberId);
    };

    const sortedMembers = useMemo(() => {
        return _.orderBy(scoredMembers, ['score'], ['desc']);
    }, [scoredMembers]);

    const maxScore = useMemo(() => {
        return sortedMembers.length > 0 ? sortedMembers[0].score : 0;
    }, [sortedMembers])

    if (!id || !gameMemberships) return null;
    return (
        <>
            <h1>Game Room {id}</h1>
            <div>This is <em>game room</em> id={id}.</div>
            {isAdmin && (<>
                <h2>Admin Panel</h2>
                <p>Add or remove members:</p>
                {possibleMembers ? possibleMembers.map((pm) => {
                    let isMember = members?.some(m => m.member_id === pm.id);
                    return (
                        <div key={pm.id}>
                            {isMember ? <em>{pm.name}</em> : <span className='quiet'>{pm.name}</span>} {' '} {isMember ? (
                                <button onClick={() => handleRemoveMember(pm.id)} disabled={removeMemberMutation.isPending}>
                                    {removeMemberMutation.isPending ? 'Removing...' : 'Remove'}
                                </button>
                            ) : (
                                <button onClick={() => handleAddMember(pm.id, pm.name)} disabled={addMemberMutation.isPending}>
                                    {addMemberMutation.isPending ? 'Adding...' : 'Add'}
                                </button>
                            )}
                        </div>
                    );
                }) : (<p>Loading possible members...</p>)}
            </>)}
            <h2>Leaderboard</h2>
            <div>
                <div className='pre'>
                    {" ".repeat((members || []).length.toString().length + maxScore.toString().length + 1)}<div className='inline-block'>{'Y\nY\n\nD\nD\n\nP '}</div>{puzzlesData?.puzzles.map((puzzle) => {
                        return (
                            <ColumnText key={`${puzzle.date.year}-${puzzle.date.day}-${puzzle.part}`} year={puzzle.date.year} day={puzzle.date.day} part={puzzle.part == 'One' ? 1 : 2} />
                        )
                    })}
                </div>

                {sortedMembers.map((member, index) => (
                    <div key={member.id} className='pre'>
                        <Position index={index} members={sortedMembers} />{member.score.toString().padStart(maxScore.toString().length, ' ')}{' '}
                        {puzzlesData?.puzzles.map((puzzle, i) => {
                            // Check if this member has completed this puzzle
                            const memberCompletions = completionData?.[member.member_id] || [];
                            const solved = memberCompletions.some(([year, day, part]) =>
                                year === puzzle.date.year &&
                                day === puzzle.date.day &&
                                part === puzzle.part
                            );
                            const firstOnly = false;
                            return (
                                solved ? (<span key={`${member.id}-${i}`} className={firstOnly ? 'first-only' : 'gold'}>*</span>) : ' '
                            )
                        })}
                        {' '}{member.member_name}
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