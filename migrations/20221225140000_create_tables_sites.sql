CREATE TABLE IF NOT EXISTS sites
(
    id      INTEGER PRIMARY KEY NOT NULL,
    name    TEXT    UNIQUE      NOT NULL,
    url     TEXT                NOT NULL
);
