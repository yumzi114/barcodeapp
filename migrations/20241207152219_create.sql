CREATE TABLE IF NOT EXISTS device
(
    id          INTEGER PRIMARY KEY NOT NULL,
    name        VARCHAR(250)        NOT NULL,
    active      BOOLEAN             NOT NULL DEFAULT 0
);
INSERT INTO device values(1,"dd",1);