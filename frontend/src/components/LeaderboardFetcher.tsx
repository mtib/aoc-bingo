'use client';

import { useState } from 'react';
import { Box, Card, CardContent, Typography, TextField, Button, Alert, Paper } from '@mui/material';
import { api } from '@/lib/api';

export default function LeaderboardFetcher() {
  const [year, setYear] = useState<string>('2025');
  const [boardId, setBoardId] = useState<string>('');
  const [sessionToken, setSessionToken] = useState<string>('');
  const [leaderboardData, setLeaderboardData] = useState<any>(null);
  const [leaderboardError, setLeaderboardError] = useState<string | null>(null);
  const [leaderboardLoading, setLeaderboardLoading] = useState(false);

  const fetchLeaderboard = async () => {
    setLeaderboardLoading(true);
    setLeaderboardError(null);
    setLeaderboardData(null);

    try {
      const response = await api.post('/leaderboard', {
        year: parseInt(year),
        board_id: parseInt(boardId),
        session_token: sessionToken
      });
      console.log('Leaderboard response:', response);
      setLeaderboardData(response);
    } catch (err) {
      const errorMsg = err instanceof Error ? err.message : 'Failed to fetch leaderboard';
      console.error('Leaderboard error:', err);
      setLeaderboardError(errorMsg);
    } finally {
      setLeaderboardLoading(false);
    }
  };

  return (
    <Card sx={{ maxWidth: 600, mx: 'auto' }}>
      <CardContent>
        <Typography variant="h5" component="h2" gutterBottom>
          Fetch Leaderboard
        </Typography>
        <Box sx={{ display: 'flex', flexDirection: 'column', gap: 2, mt: 2 }}>
          <TextField
            label="Year"
            type="number"
            value={year}
            onChange={(e) => setYear(e.target.value)}
            fullWidth
          />
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
            variant="contained"
            onClick={fetchLeaderboard}
            disabled={leaderboardLoading || !year || !boardId || !sessionToken}
          >
            {leaderboardLoading ? 'Loading...' : 'Fetch Leaderboard'}
          </Button>
        </Box>

        {leaderboardError && (
          <Alert severity="error" sx={{ mt: 2 }}>
            {leaderboardError}
          </Alert>
        )}

        {leaderboardData && (
          <Box sx={{ mt: 2 }}>
            <Typography variant="h6" gutterBottom>
              Response:
            </Typography>
            <Paper sx={{ p: 2, maxHeight: 400, overflow: 'auto' }} variant="outlined">
              <pre style={{ margin: 0, fontSize: '0.875rem' }}>
                {JSON.stringify(leaderboardData, null, 2)}
              </pre>
            </Paper>
          </Box>
        )}
      </CardContent>
    </Card>
  );
}
