CREATE TABLE IF NOT EXISTS device
(
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    name        VARCHAR(250)        NOT NULL,
    active      BOOLEAN             NOT NULL DEFAULT 0
);
INSERT INTO device (name,active) VALUES ('test',0);
INSERT INTO device (name,active) VALUES ('test1',0);
INSERT INTO device (name,active) VALUES ('test2',0);