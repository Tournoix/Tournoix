-- Your SQL goes here
CREATE TABLE bets (
  id INTEGER PRIMARY KEY AUTO_INCREMENT,
  fk_matchs INTEGER NOT NULL,
  fk_teams INTEGER NOT NULL,
  fk_nuts INTEGER NOT NULL,
  nbNut INTEGER NOT NULL,

  CONSTRAINT bets_matchs FOREIGN KEY (fk_matchs) REFERENCES matchs(id),
  CONSTRAINT bets_teams FOREIGN KEY (fk_teams) REFERENCES teams(id),
  CONSTRAINT bets_nuts FOREIGN KEY (fk_nuts) REFERENCES nuts(id)
)