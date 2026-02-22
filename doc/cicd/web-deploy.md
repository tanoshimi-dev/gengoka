# Web Deploy (GitHub Actions)

## Overview

Automated deployment of the Next.js web frontend (`sys/frontend/user/web/`) to VPS on push to `main`.

**Workflow file:** `.github/workflows/web-deploy.yml`

## Trigger

- **Branch:** `main`
- **Path filter:** `sys/frontend/user/web/**`

Only runs when web frontend files are changed.

## Deployment Flow

```
GitHub Actions                          VPS
─────────────────                       ──────────────────────────
1. Checkout repo
2. SCP web source ──────────────────▶  DEPLOY_PATH_WEB/
   (strip_components: 4)               (e.g. /home/xxx/traefik/gengoka.app/web)
3. SSH ─────────────────────────────▶  docker compose build web
                                       docker compose up -d web
```

### Steps

| Step | Action | Tool |
|------|--------|------|
| 1 | Checkout repository | `actions/checkout@v4` |
| 2 | Copy `sys/frontend/user/web/` to VPS | `appleboy/scp-action@v0.1.7` |
| 3 | Rebuild and restart `web` container | `appleboy/ssh-action@v1` |

### How SCP Works

`strip_components: 4` removes the `sys/frontend/user/web/` prefix so files are copied directly into the target directory:

```
sys/frontend/user/web/Dockerfile    →  DEPLOY_PATH_WEB/Dockerfile
sys/frontend/user/web/package.json  →  DEPLOY_PATH_WEB/package.json
sys/frontend/user/web/src/...       →  DEPLOY_PATH_WEB/src/...
```

### How Docker Build Works

On VPS, the `docker-compose.prod.yml` defines the `web` service:

- **Build context:** `./web` (the directory where source was copied)
- **Dockerfile:** Multi-stage build (deps → build → production runner)
- **Output:** `standalone` Next.js server on port 3000
- **Network:** `traefik-network` (external) + `gengoka-network` (internal)
- **Routing:** Traefik reverse proxy via labels → `gengoka.app`

## Required GitHub Secrets

| Secret | Description |
|--------|-------------|
| `VPS_HOST` | VPS server hostname or IP |
| `VPS_USER` | SSH username |
| `VPS_SSH_KEY` | SSH private key |
| `DEPLOY_PATH_WEB` | Target path on VPS (e.g. `/home/xxx/traefik/gengoka.app/web`) |

## VPS Prerequisites

The following must already exist on VPS before the workflow runs:

- `docker-compose.prod.yml` at the parent of `DEPLOY_PATH_WEB`
- `.env` — database and backend secrets
- `.env.web` — web-specific environment variables (`NEXT_PUBLIC_GOOGLE_CLIENT_ID`, `NEXT_PUBLIC_LINE_CHANNEL_ID`, etc.)
- `traefik-network` Docker network (external)

## Previous Issues (Fixed)

The original workflow had these problems:

| Issue | Detail |
|-------|--------|
| `git pull` on VPS | Repo was not cloned on VPS — replaced with SCP |
| `${{ secrets.PROJECT_PATH }}` | Undefined secret — replaced with `DEPLOY_PATH_WEB` |
| `docker-compose.prod.yml` not found | Workflow ran from wrong directory — fixed to `cd` into correct path |
