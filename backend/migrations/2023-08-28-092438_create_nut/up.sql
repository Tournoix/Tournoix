-- Your SQL goes here
CREATE TABLE nuts (
  id INTEGER PRIMARY KEY AUTO_INCREMENT,
  fk_users INTEGER NOT NULL,
  fk_tournaments INTEGER NOT NULL,
  stock INTEGER NOT NULL,

  CONSTRAINT nuts_users FOREIGN KEY (fk_users) REFERENCES users(id) ON DELETE CASCADE,
  CONSTRAINT nuts_tournaments FOREIGN KEY (fk_tournaments) REFERENCES tournaments(id) ON DELETE CASCADE
)