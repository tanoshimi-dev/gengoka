# Gengoka.app (Gengoka)

言語化力を鍛えるトレーニングアプリ

A training app to improve your ability to express thoughts in words.

## Overview

ゲンゴカは、限られた文字数で的確に表現する力を養うトレーニングアプリです。AIが出題するお題に対して、制限文字数内で回答し、フィードバックを受けることで言語化スキルを向上させます。

## Features

### Core Features
- **カテゴリー別トレーニング** - 5つのカテゴリーで言語化力を鍛える
  - 状況描写 (30文字)
  - 要約力 (50文字)
  - 感性の言語化 (30文字)
  - 言い換え (20文字)
  - 概念説明 (50文字)

- **AIフィードバック** - 回答に対するスコアと改善ポイント
- **文字数制限** - 限られた文字数で的確に表現する訓練

### SNS Features
- **タイムライン** - 他のユーザーの回答を閲覧
- **いいね** - 優れた回答に「いいね」
- **コメント** - 回答にコメント
- **フォロー** - ユーザーをフォロー
- **ランキング** - デイリー/ウィークリー/全期間のランキング

## Tech Stack

### Frontend (Web)
- Next.js
- TypeScript
- Tailwind CSS

### Mobile
- iOS (Swift / SwiftUI)
- Android (Kotlin / Jetpack Compose)

### Backend
- Rust
- Actix Web
- SQLx
- PostgreSQL

### Infrastructure
- Docker
- Docker Compose

## Architecture

```mermaid
graph TB
    subgraph Client
        Web[Web Browser]
        Mobile[Mobile App]
    end

    subgraph Frontend
        Next[Next.js<br/>React + TypeScript]
    end

    subgraph Backend
        API[Actix Web<br/>Rust API Server]

        subgraph Handlers
            H1[Challenge Handler]
            H2[Answer Handler]
            H3[User Handler]
            H4[Social Handler]
        end
    end

    subgraph Database
        PG[(PostgreSQL)]
    end

    subgraph External
        AI[AI Service<br/>Claude API]
    end

    Web --> Next
    Mobile --> Next
    Next -->|REST API| API
    API --> H1
    API --> H2
    API --> H3
    API --> H4
    H1 --> PG
    H2 --> PG
    H3 --> PG
    H4 --> PG
    H2 -.->|Feedback| AI
```

### Data Flow

```mermaid
sequenceDiagram
    participant U as User
    participant F as Frontend
    participant B as Backend
    participant D as Database
    participant AI as AI Service

    U->>F: Select Category
    F->>B: GET /challenges/daily
    B->>D: Query challenges
    D-->>B: Challenge data
    B-->>F: Challenge list
    F-->>U: Display challenge

    U->>F: Submit answer
    F->>B: POST /challenges/{id}/answers
    B->>D: Save answer
    B->>AI: Request feedback
    AI-->>B: Score & feedback
    B->>D: Update answer with score
    B-->>F: Answer with feedback
    F-->>U: Display result
```


## Project Structure

```
gengoka/
├── doc/
│   ├── prompt.md          # Project requirements
│   └── ui/                 # UI Prototype (HTML)
└── sys/
    └── backend/
        ├── docker-compose.dev.yml
        └── app/
            ├── Cargo.toml
            ├── Dockerfile
            └── src/
                ├── main.rs
                ├── config/
                ├── db/
                ├── handlers/
                ├── middleware/
                ├── models/
                ├── routes/
                └── utils/
```

## Getting Started

### Prerequisites

- Docker & Docker Compose
- Rust (for local development)

### Run with Docker

```bash
cd sys/backend

# Start all services
docker-compose -f docker-compose.dev.yml up -d

# View logs
docker-compose -f docker-compose.dev.yml logs -f
```

### Services

| Service | URL | Description |
|---------|-----|-------------|
| Backend API | http://localhost:8080 | Rust API Server |
| Adminer | http://localhost:5050 | Database Admin UI |
| PostgreSQL | localhost:5432 | Database |

### Seed Data

```bash
docker exec -i gengoka-db psql -U gengoka -d gengoka_db < app/seeds/seed_data.sql
```

## Development

### Local Development (Rust)

```bash
cd sys/backend/app

# Copy environment file
cp .env.example .env

# Run
cargo run
```

### Database Migrations

Migrations run automatically on startup. Tables are created if they don't exist.

## License

MIT

## Author

Created with Claude Code
