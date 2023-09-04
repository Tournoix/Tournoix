-- Your SQL goes here
CREATE TABLE teams (
  id INTEGER PRIMARY KEY AUTO_INCREMENT,
  fk_tournaments INTEGER NOT NULL,
  name VARCHAR(255) NOT NULL,
  `group` INTEGER NOT NULL,

  CONSTRAINT teams_tournaments FOREIGN KEY (fk_tournaments) REFERENCES tournaments(id) ON DELETE CASCADE
)