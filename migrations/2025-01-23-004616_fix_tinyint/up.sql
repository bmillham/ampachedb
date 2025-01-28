-- Diesel can't handle tinyint, so modify existing colums

CREATE TABLE requestlist_tmp LIKE requestlist;
ALTER TABLE requestlist_tmp MODIFY COLUMN code INT NOT NULL DEFAULT 0;
ALTER TABLE requestlist RENAME requestlist_old;
INSERT INTO requestlist_tmp SELECT * FROM requestlist_old;
ALTER TABLE requestlist_tmp RENAME requestlist;
CREATE TABLE song_tmp LIKE song;
ALTER TABLE song_tmp MODIFY COLUMN year INT NOT NULL DEFAULT 0;
ALTER TABLE song_tmp MODIFY COLUMN bitrate INT NOT NULL DEFAULT 0;
ALTER TABLE song_tmp MODIFY COLUMN rate INT NOT NULL DEFAULT 0;
ALTER TABLE song RENAME song_old;
INSERT INTO song_tmp SELECT * FROM song_old;
ALTER TABLE song_tmp RENAME song;
DROP TABLE requestlist_old;
DROP TABLE song_old;
