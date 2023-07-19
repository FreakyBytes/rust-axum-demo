CREATE TABLE IF NOT EXISTS links (
    link_id serial NOT NULL PRIMARY KEY,
    code text NOT NULL,
    url text NOT NULL,
    created_at timestamptz NOT NULL DEFAULT now()
);
CREATE TABLE IF NOT EXISTS link_visits (
    link_id integer NOT NULL REFERENCES links(link_id),
    ts timestamptz NOT NULL DEFAULT now()
);
CREATE INDEX IF NOT EXISTS link_id_idx ON link_visits (link_id);