'use client';

import { createTheme } from '@mui/material/styles';

export const theme = createTheme({
  palette: {
    mode: 'dark',
    primary: {
      main: '#90caf9',
    },
    secondary: {
      main: '#f48fb1',
    },
    background: {
      default: '#0f0f23',
      paper: '#11112f',
    },
  },
  typography: {
    fontFamily: '"Source Code Pro", monospace',
  },
});
