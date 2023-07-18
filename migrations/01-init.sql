CREATE TABLE IF NOT EXISTS links (
    link_id BIGINT NOT NULL PRIMARY KEY,
    code text NOT NULL,
    url text NOT NULL,
    created_at timestamptz NOT NULL DEFAULT now()
)