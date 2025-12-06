'use client';

import { useEffect, useState, useMemo } from 'react';
import { CheckCircle, XCircle, AlertCircle, Loader } from 'lucide-react';
import { AppBar, Toolbar, Typography, Box, Tooltip, Stack, Paper } from '@mui/material';
import { api } from '@/lib/api';
import LeaderboardFetcher from '@/components/LeaderboardFetcher';

interface HealthResponse {
  status: string;
}

export default function Home() {
  const [health, setHealth] = useState<HealthResponse | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    let isInitialLoad = true;

    const checkHealth = async () => {
      try {
        if (isInitialLoad) {
          setLoading(true);
        }
        const response = await api.get<HealthResponse>('/health');
        setError(null);
        setHealth(response);
      } catch (err) {
        setError(err instanceof Error ? err.message : 'Failed to fetch health status');
        setHealth(null);
      } finally {
        if (isInitialLoad) {
          setLoading(false);
          isInitialLoad = false;
        }
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

  const statusIcon = useMemo(() => {
    if (loading && !health) {
      return <Loader size={20} style={{ animation: 'spin 1s linear infinite' }} />;
    }
    if (error) {
      return <XCircle size={20} color="#f44336" />;
    }
    if (isHealthy) {
      return <CheckCircle size={20} color="#4caf50" />;
    }
    return <AlertCircle size={20} color="#ff9800" />;
  }, [loading, health, error, isHealthy]);

  const statusTitle = useMemo(() => {
    if (loading && !health) return 'Checking connection...';
    if (error) return `Error: ${error}`;
    if (isHealthy) return 'Connected';
    return 'Unhealthy';
  }, [loading, health, error, isHealthy]);

  console.log(statusTitle);

  return (
    <>
      <Paper square>
        <Stack direction="row" spacing={2} alignItems="center" px={3} py={2}>
          <Typography variant="h6" component="div" sx={{ flexGrow: 1 }} flexGrow={1}>
            AOC Bingo
          </Typography>
          <Stack direction="row" alignItems="center" spacing={2}>
            <Tooltip title={statusTitle}>
              {statusIcon}
            </Tooltip>
          </Stack>
        </Stack >
      </Paper>
      <Box sx={{ p: 3 }}>
        <LeaderboardFetcher />
      </Box>
    </>
  );
}
