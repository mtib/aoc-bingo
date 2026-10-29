#!/usr/bin/env bash
# Deploy aoc-bingo to mtib-nas: CI has built the amd64 images and pushed registry.vpn.mm/aoc-bingo-backend:latest and
# registry.vpn.mm/aoc-bingo-frontend:latest. The NAS pulls them from its own registry (localhost:4025), swaps the containers, waits
# for them to be healthy and only then drops the old images. Nothing is built on the NAS. The first run also switches the two
# compose services from `build:` to the registry images (copy edited, validated, original backed up).
set -euo pipefail

# A dropped ssh session must not kill the swap halfway: the remote script ignores SIGHUP, ssh keeps the connection alive.
ssh -o ServerAliveInterval=15 -o ServerAliveCountMax=10 mtib-nas 'bash -s' <<'REMOTE'
trap '' HUP PIPE
set -euo pipefail
cd ~/containers
stamp=$(date +%Y%m%d-%H%M%S)
BE=localhost:4025/aoc-bingo-backend:latest
FE=localhost:4025/aoc-bingo-frontend:latest
dc() { docker compose --profile aoc-bingo "$@"; }

# until now both services were built on this host from ./aoc-bingo (a git checkout): they now use the registry images CI pushes.
# The edit happens on a copy, is validated, and only then replaces the shared compose (original kept as a backup).
if grep -q "context: ./aoc-bingo/" docker-compose.yml; then
  cp -p docker-compose.yml docker-compose.yml.new
  python3 - <<'PY'
p = "docker-compose.yml.new"
s = open(p).read()
be = "    build:\n      context: ./aoc-bingo/backend\n      dockerfile: Dockerfile\n"
fe = "    build:\n      context: ./aoc-bingo/frontend\n      dockerfile: Dockerfile\n      args:\n        - VITE_BACKEND_URL=https://api.aoc.mtib.dev\n"
assert s.count(be) == 1 and s.count(fe) == 1, "aoc-bingo build blocks not in the expected shape"
s = s.replace(be, "    image: localhost:4025/aoc-bingo-backend:latest\n")
s = s.replace(fe, "    image: localhost:4025/aoc-bingo-frontend:latest\n")
open(p, "w").write(s)
PY
  if docker compose --profile aoc-bingo -f docker-compose.yml.new config --quiet; then
    cp -p docker-compose.yml "docker-compose.yml.bak-aoc-registry-$stamp"
    mv docker-compose.yml.new docker-compose.yml
    echo "aoc-bingo now uses the registry images (backup docker-compose.yml.bak-aoc-registry-$stamp)"
  else
    rm -f docker-compose.yml.new
    echo "edited compose invalid, left the original untouched" >&2
    exit 1
  fi
fi

# the containers that run now, to drop their images once the new ones are healthy
old_be=$(docker inspect -f '{{.Image}}' "$(dc ps -q aoc-bingo-backend)" 2>/dev/null || true)
old_fe=$(docker inspect -f '{{.Image}}' "$(dc ps -q aoc-bingo-frontend)" 2>/dev/null || true)
dc pull --quiet aoc-bingo-backend aoc-bingo-frontend
dc up -d --no-deps aoc-bingo-backend aoc-bingo-frontend </dev/null

# backend /health answers {"status":"OK"}, the frontend serves the SPA at / (both on their published host ports)
wait_ok() {  # <name> <url>
  for i in $(seq 1 30); do
    curl -fsS --max-time 3 "$2" >/dev/null 2>&1 && { echo "$1 healthy"; return 0; }
    sleep 2
  done
  return 1
}
healthy=0
wait_ok backend http://localhost:4801/health && wait_ok frontend http://localhost:4802/ && healthy=1
# an unhealthy container fails the deploy and the previous images stay on the host (the way back)
[ "$healthy" = 1 ] || { echo "deploy failed: aoc-bingo is not healthy; previous images $old_be $old_fe are still on the host" >&2; exit 1; }
# the running containers must really use the images just pulled
for pair in "aoc-bingo-backend $BE" "aoc-bingo-frontend $FE"; do
  set -- $pair
  [ "$(docker inspect -f '{{.Image}}' "$(dc ps -q "$1")")" = "$(docker image inspect -f '{{.Id}}' "$2")" ] \
    || { echo "deploy failed: $1 still runs an older image" >&2; exit 1; }
done
# healthy: the old images go. Plain rmi, so an image something still uses stays; a failed cleanup never fails the deploy.
for pair in "backend $old_be $BE" "frontend $old_fe $FE"; do
  set -- $pair
  new=$(docker image inspect -f '{{.Id}}' "$3")
  if [ -n "$2" ] && [ "$2" != "$new" ]; then docker rmi "$2" 2>&1 | tail -1 || true; else echo "no old $1 image"; fi
done
docker image prune -f 2>&1 | tail -1 || true
# ~/containers/aoc-bingo is a git checkout of the whole repo and backend/data (the sqlite database, mounted into the backend)
# lives inside it: it stays. Only the leftover on-host build contexts of the images could ever go, by hand.
REMOTE

for i in $(seq 1 10); do code=$(curl -s -o /dev/null -w '%{http_code}' https://api.aoc.mtib.dev/health || true); [ "$code" = 200 ] && break; sleep 3; done
echo "public backend health: $code"
[ "$code" = 200 ]
