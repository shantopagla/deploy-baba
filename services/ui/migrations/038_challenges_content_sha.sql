-- W-CHL: freshness tracking for the content/challenges/*.md source-of-truth pivot (ADR-036).
-- content_sha holds a hash of the raw markdown file bytes so xtask's `challenges sync` /
-- `challenges diff` / `challenges gen-migration` can detect when repo content is newer than
-- the cached SQLite row without re-parsing every field.
ALTER TABLE challenges ADD COLUMN content_sha TEXT;
