CREATE TABLE IF NOT EXISTS jobs (
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    slug       TEXT    NOT NULL UNIQUE,
    company    TEXT    NOT NULL,
    title      TEXT    NOT NULL,
    location   TEXT,
    start_date TEXT    NOT NULL,
    end_date   TEXT,
    summary    TEXT    NOT NULL,
    tech_stack TEXT,
    sort_order INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS job_details (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    job_id      INTEGER NOT NULL REFERENCES jobs(id),
    detail_text TEXT    NOT NULL,
    category    TEXT,
    sort_order  INTEGER NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_job_details_job_id ON job_details(job_id);
