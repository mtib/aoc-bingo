# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

AoC Bingo is a web application for Advent of Code leaderboard management and bingo card generation. The project consists of a Rust backend using Rocket and a Next.js frontend with Material-UI.

## Development Commands

### Backend (Rust + Rocket)
```bash
cd backend
cargo build                    # Build the backend
cargo run                      # Run the backend server (port 8000)
cargo test                     # Run all tests
cargo test <test_name>         # Run specific test by name
```

### Frontend (Next.js + TypeScript)
```bash
cd frontend
bun dev                        # Run development server (port 3000)
bun build                      # Build for production
bun start                      # Start production server
bun run lint                   # Run ESLint
```

### Docker Deployment
```bash
docker-compose up              # Start both services
docker-compose up --build      # Rebuild and start
docker-compose down            # Stop services
```

Services run on:
- Backend: http://localhost:8000
- Frontend: http://localhost:8001 (in Docker) or http://localhost:3000 (local dev)

## Architecture

### Backend Structure (Rust)

The backend follows a layered architecture pattern:

**API Layer** (`src/api/`) - Rocket HTTP endpoints
- `health.rs` - Health check endpoint
- `leaderboard.rs` - Leaderboard API routes (`/leaderboard` POST, `/leaderboard/bingo/all` POST)

**Service Layer** (`src/service/`) - Business logic
- `leaderboard.rs` - LeaderboardService handles leaderboard operations and caching
- `aoc_utils.rs` - Advent of Code utilities and calendar logic

**Repository Layer** (`src/repository/`) - Data access
- `leaderboard.rs` - LeaderboardRepository for database operations

**Client Layer** (`src/client/`) - External API integrations
- `aoc.rs` - Advent of Code API client with session token authentication

**Database** (`src/db/`)
- `manager.rs` - DatabaseManager handles SQLite connections and migrations
- `migrations/` - SQL migration files executed on startup
- Database file: `./data/db.sqlite` (created automatically)
- Uses WAL mode for better concurrency

**Models** (`src/model/`)
- `aoc.rs` - Core domain types: `AocPuzzle`, `PuzzleDate`, `AocPart`
- `leaderboard.rs` - DTOs for leaderboard data exchange

### Frontend Structure (Next.js)

**App Router** (`src/app/`)
- `page.tsx` - Main application page with health check polling
- `layout.tsx` - Root layout with Material-UI theme provider
- Uses dynamic import to disable SSR for LeaderboardFetcher

**Components** (`src/components/`)
- `LeaderboardFetcher.tsx` - Form to fetch leaderboard data with persistent state
- `BingoFetcher.tsx` - Form to generate bingo options with difficulty slider

**Contexts** (`src/contexts/`)
- `LeaderboardContext.tsx` - Shared state context for year, boardId, and sessionToken
- All contexts use `react-use-storage-state` for persistence in localStorage
- Context keys: `leaderboard-year`, `leaderboard-boardId`, `leaderboard-sessionToken`

**Library** (`src/lib/`)
- `api.ts` - Typed API client wrapper (GET, POST, PUT, DELETE)
- `config.ts` - Backend URL configuration from environment

### Key Design Patterns

**Backend:**
- Trait-based configuration: `ConfigureRocket` trait for composable Rocket setup
- Migration system: Embedded SQL files using `include_dir`, applied on startup
- Error handling: `thiserror` for typed error enums with automatic conversion
- CORS: Configured via `rocket_cors` for cross-origin requests

**Frontend:**
- Client-side only components with `'use client'` directive
- API health polling every 1 second with visual status indicator
- Material-UI for consistent design system
- TypeScript for type safety
- **Preferred patterns:**
  - Use `Stack` for layouts instead of `Card` where possible
  - Use React Contexts (`src/contexts/`) for shared state across components
  - Use Material-UI transitions (https://mui.com/material-ui/transitions/) for showing/hiding UI

### API Endpoints

**Backend:**
- `GET /health` - Returns `{"status": "OK"}`
- `POST /leaderboard` - Fetch/cache leaderboard data
  - Body: `{year: u32, board_id: u32, session_token: string}`
- `POST /leaderboard/bingo/all` - Generate bingo options
  - Body: `{board_id: u32, session_token: string, difficulty?: f32}`

### Database Migrations

Migrations are automatically applied on backend startup. Migration files in `backend/src/db/migrations/` must be named with a sortable prefix (e.g., `2025_12_06_22_12_leaderboard_cache.sql`). The system tracks applied migrations in the `migrations` table.

### Environment Configuration

**Frontend** (`.env.example`):
- `NEXT_PUBLIC_BACKEND_URL` - Backend API URL (default: http://localhost:8000)

**Backend** (docker-compose.yml):
- `ROCKET_ADDRESS` - Listen address (0.0.0.0 in Docker)
- `ROCKET_PORT` - Port number (8000)
- `ROCKET_MAX_BLOCKING` - Max blocking threads (4)

### Testing

The backend uses Rust's built-in test framework. Run all tests with `cargo test` or specific tests with `cargo test <name>`.

### Data Persistence

The backend stores data in `./data/db.sqlite`. This directory is volume-mounted in Docker to persist data across container restarts.