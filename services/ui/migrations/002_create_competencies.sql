CREATE TABLE IF NOT EXISTS competencies (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    slug        TEXT    NOT NULL UNIQUE,
    name        TEXT    NOT NULL,
    description TEXT    NOT NULL,
    icon        TEXT,
    sort_order  INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS competency_evidence (
    id             INTEGER PRIMARY KEY AUTOINCREMENT,
    competency_id  INTEGER NOT NULL REFERENCES competencies(id),
    job_id         INTEGER NOT NULL REFERENCES jobs(id),
    detail_id      INTEGER REFERENCES job_details(id),
    highlight_text TEXT,
    sort_order     INTEGER NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_ce_competency_id ON competency_evidence(competency_id);
CREATE INDEX IF NOT EXISTS idx_ce_job_id        ON competency_evidence(job_id);
