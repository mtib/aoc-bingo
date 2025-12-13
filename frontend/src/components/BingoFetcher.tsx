'use client';

import { useState } from 'react';
import { Box, Card, CardContent, Typography, TextField, Button, Alert, Paper, Slider, Collapse, Autocomplete, Chip } from '@mui/material';
import { api } from '@/lib/api';
import { useLeaderboardContext } from '@/contexts/LeaderboardContext';

interface Member {
  id: number;
  name: string;
}

interface LeaderboardData {
  data: {
    members: Record<string, Member>;
  };
}

function BingoFetcher() {
  const { boardId, setBoardId, sessionToken, setSessionToken } = useLeaderboardContext();
  const [difficulty, setDifficulty] = useState<number>(0.5);
  const [bingoData, setBingoData] = useState<any>(null);
  const [bingoError, setBingoError] = useState<string | null>(null);
  const [bingoLoading, setBingoLoading] = useState(false);

  const [members, setMembers] = useState<Member[]>([]);
  const [selectedMembers, setSelectedMembers] = useState<Member[]>([]);
  const [fetchingUsers, setFetchingUsers] = useState(false);
  const [fetchUsersError, setFetchUsersError] = useState<string | null>(null);

  const fetchUsers = async () => {
    setFetchingUsers(true);
    setFetchUsersError(null);

    try {
      const response = await api.post<LeaderboardData>('/leaderboard', {
        year: 2015,
        board_id: parseInt(boardId),
        session_token: sessionToken
      });

      const membersArray = Object.values(response.data.members);
      setMembers(membersArray);
      console.log('Fetched users:', membersArray);
    } catch (err) {
      const errorMsg = err instanceof Error ? err.message : 'Failed to fetch users';
      console.error('Fetch users error:', err);
      setFetchUsersError(errorMsg);
    } finally {
      setFetchingUsers(false);
    }
  };

  const fetchBingo = async () => {
    setBingoLoading(true);
    setBingoError(null);
    setBingoData(null);

    try {
      const response = await api.post('/leaderboard/bingo/all', {
        board_id: parseInt(boardId),
        session_token: sessionToken,
        member_ids: selectedMembers.map(m => m.id),
        difficulty: difficulty
      });
      console.log('Bingo response:', response);
      setBingoData(response);
    } catch (err) {
      const errorMsg = err instanceof Error ? err.message : 'Failed to fetch bingo data';
      console.error('Bingo error:', err);
      setBingoError(errorMsg);
    } finally {
      setBingoLoading(false);
    }
  };

  const difficultyMarks = [
    { value: 0.1, label: 'Easy' },
    { value: 0.5, label: 'Medium' },
    { value: 0.9, label: 'Hard' },
  ];

  return (
    <Card>
      <CardContent>
        <Typography variant="h5" component="h2" gutterBottom>
          Generate Bingo Options
        </Typography>
        <Box sx={{ display: 'flex', flexDirection: 'column', gap: 2, mt: 2 }}>
          <TextField
            label="Board ID"
            type="number"
            value={boardId}
            onChange={(e) => setBoardId(e.target.value)}
            fullWidth
          />
          <TextField
            label="Session Token"
            type="password"
            value={sessionToken}
            onChange={(e) => setSessionToken(e.target.value)}
            fullWidth
          />

          <Button
            variant="outlined"
            onClick={fetchUsers}
            disabled={fetchingUsers || !boardId || !sessionToken}
          >
            {fetchingUsers ? 'Loading Users...' : 'Fetch Users'}
          </Button>

          <Collapse in={!!fetchUsersError}>
            <Alert severity="error">
              {fetchUsersError}
            </Alert>
          </Collapse>

          <Collapse in={members.length > 0}>
            <Autocomplete
              multiple
              options={members}
              getOptionLabel={(option) => option.name}
              value={selectedMembers}
              onChange={(_, newValue) => setSelectedMembers(newValue)}
              renderInput={(params) => (
                <TextField
                  {...params}
                  label="Select Users"
                  placeholder="Choose users..."
                />
              )}
              renderTags={(value, getTagProps) =>
                value.map((option, index) => (
                  <Chip
                    {...getTagProps({ index })}
                    key={option.id}
                    label={option.name}
                  />
                ))
              }
            />
          </Collapse>
          <Box sx={{ px: 1 }}>
            <Typography gutterBottom>
              Difficulty: {difficulty.toFixed(1)}
            </Typography>
            <Slider
              value={difficulty}
              onChange={(_, value) => setDifficulty(value as number)}
              min={0.1}
              max={0.9}
              step={0.1}
              marks={difficultyMarks}
              valueLabelDisplay="auto"
            />
            <Typography variant="caption" color="text.secondary">
              {difficulty < 0.5
                ? `${((1 - difficulty) * 100).toFixed(0)}% chance to skip hard puzzles`
                : `${(difficulty * 100).toFixed(0)}% chance to skip easy puzzles`}
            </Typography>
          </Box>
          <Button
            variant="contained"
            onClick={fetchBingo}
            disabled={bingoLoading || !boardId || !sessionToken || selectedMembers.length === 0}
          >
            {bingoLoading ? 'Loading...' : 'Generate Bingo Options'}
          </Button>
        </Box>

        <Collapse in={!!bingoError}>
          <Alert severity="error" sx={{ mt: 2 }}>
            {bingoError}
          </Alert>
        </Collapse>

        <Collapse in={!!bingoData}>
          <Box sx={{ mt: 2 }}>
            <Typography variant="h6" gutterBottom>
              Response:
            </Typography>
            <Paper sx={{ p: 2, maxHeight: 400, overflow: 'auto' }} variant="outlined">
              <pre style={{ margin: 0, fontSize: '0.875rem' }}>
                {JSON.stringify(bingoData, null, 2)}
              </pre>
            </Paper>
          </Box>
        </Collapse>
      </CardContent>
    </Card>
  );
}

export default BingoFetcher;
