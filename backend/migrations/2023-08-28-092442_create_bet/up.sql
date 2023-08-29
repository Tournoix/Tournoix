-- Your SQL goes here
CREATE TABLE bets (
  id INTEGER PRIMARY KEY AUTO_INCREMENT,
  fk_games INTEGER NOT NULL,
  fk_teams INTEGER NOT NULL,
  fk_nuts INTEGER NOT NULL,
  nb_nut INTEGER NOT NULL,

  CONSTRAINT bets_games FOREIGN KEY (fk_games) REFERENCES games(id),
  CONSTRAINT bets_teams FOREIGN KEY (fk_teams) REFERENCES teams(id),
  CONSTRAINT bets_nuts FOREIGN KEY (fk_nuts) REFERENCES nuts(id)
)