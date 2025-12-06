'use client';

import { useEffect, useState } from 'react';
import { Box, Container, Paper, Typography, Chip, CircularProgress } from '@mui/material';
import { api } from '@/lib/api';

interface HealthResponse {
  status: string;
}

export default function Home() {
  const [health, setHealth] = useState<HealthResponse | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [lastCheck, setLastCheck] = useState<Date | null>(null);

  useEffect(() => {
    const checkHealth = async () => {
      try {
        setLoading(true);
        setError(null);
        const response = await api.get<HealthResponse>('/health');
        setHealth(response);
        setLastCheck(new Date());
      } catch (err) {
        setError(err instanceof Error ? err.message : 'Failed to fetch health status');
        setHealth(null);
      } finally {
        setLoading(false);
      }
    };

    // Check immediately on mount
    checkHealth();

    // Set up interval to check every second
    const interval = setInterval(checkHealth, 1000);

    // Cleanup interval on unmount
    return () => clearInterval(interval);
  }, []);

  const isHealthy = health?.status === 'OK';

  return (
    <Container maxWidth="sm" sx={{ mt: 8 }}>
      <Paper elevation={3} sx={{ p: 4 }}>
        <Box textAlign="center">
          <Typography variant="h4" component="h1" gutterBottom>
            Backend Health Monitor
          </Typography>

          <Box sx={{ my: 4 }}>
            {loading && !health ? (
              <CircularProgress />
            ) : error ? (
              <Chip
                label="Error"
                color="error"
                sx={{ fontSize: '1.2rem', py: 3, px: 2 }}
              />
            ) : (
              <Chip
                label={isHealthy ? 'Healthy' : 'Unhealthy'}
                color={isHealthy ? 'success' : 'warning'}
                sx={{ fontSize: '1.2rem', py: 3, px: 2 }}
              />
            )}
          </Box>

          {error && (
            <Typography variant="body2" color="error" sx={{ mb: 2 }}>
              {error}
            </Typography>
          )}

          {health && (
            <Box sx={{ mt: 3 }}>
              <Typography variant="body1" color="text.secondary">
                Status: <strong>{health.status}</strong>
              </Typography>
            </Box>
          )}

          {lastCheck && (
            <Typography variant="caption" color="text.secondary" sx={{ mt: 2, display: 'block' }}>
              Last checked: {lastCheck.toLocaleTimeString()}
            </Typography>
          )}
        </Box>
      </Paper>
    </Container>
  );
}
