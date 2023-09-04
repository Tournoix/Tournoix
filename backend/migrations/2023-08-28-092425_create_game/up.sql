-- Your SQL goes here
CREATE TABLE games (
  id INTEGER PRIMARY KEY AUTO_INCREMENT,
  fk_team1 INTEGER NOT NULL,
  fk_team2 INTEGER NOT NULL,
  score1 INTEGER NOT NULL,
  score2 INTEGER NOT NULL,
  phase INTEGER NOT NULL,
  place INTEGER NOT NULL,
  is_open BOOLEAN NOT NULL DEFAULT TRUE,
  has_gained_nut BOOLEAN NOT NULL DEFAULT FALSE,

  CONSTRAINT matchs_team1 FOREIGN KEY (fk_team1) REFERENCES teams(id) ON DELETE CASCADE,
  CONSTRAINT matchs_team2 FOREIGN KEY (fk_team2) REFERENCES teams(id) ON DELETE CASCADE
)