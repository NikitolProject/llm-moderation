CREATE TABLE IF NOT EXISTS moderations (
    id UUID PRIMARY KEY,
    message TEXT NOT NULL,
    danger_score REAL NOT NULL,
    categories TEXT[] NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_moderations_created_at ON moderations(created_at DESC);

CREATE INDEX IF NOT EXISTS idx_moderations_score ON moderations(danger_score);
