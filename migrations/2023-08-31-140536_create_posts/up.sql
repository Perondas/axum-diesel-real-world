CREATE TABLE posts
(
    id        uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    title     VARCHAR NOT NULL,
    body      TEXT    NOT NULL,
    published BOOLEAN NOT NULL DEFAULT FALSE
)
