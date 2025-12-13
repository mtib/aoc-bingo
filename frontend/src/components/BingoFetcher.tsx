'use client';

import { useState } from 'react';
import { Box, Card, CardContent, Typography, TextField, Button, Alert, Paper, Slider, Collapse } from '@mui/material';
import { api } from '@/lib/api';
import { useLeaderboardContext } from '@/contexts/LeaderboardContext';

function BingoFetcher() {
  const { boardId, setBoardId, sessionToken, setSessionToken } = useLeaderboardContext();
  const [difficulty, setDifficulty] = useState<number>(0.5);
  const [bingoData, setBingoData] = useState<any>(null);
  const [bingoError, setBingoError] = useState<string | null>(null);
  const [bingoLoading, setBingoLoading] = useState(false);

  const fetchBingo = async () => {
    setBingoLoading(true);
    setBingoError(null);
    setBingoData(null);

    try {
      const response = await api.post('/leaderboard/bingo/all', {
        board_id: parseInt(boardId),
        session_token: sessionToken,
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
            disabled={bingoLoading || !boardId || !sessionToken}
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
