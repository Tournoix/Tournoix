-- Your SQL goes here
CREATE TABLE subscribers (
  id INTEGER PRIMARY KEY AUTO_INCREMENT,
  fk_users INTEGER NOT NULL,
  fk_tournaments INTEGER NOT NULL,

  CONSTRAINT subscribers_users FOREIGN KEY (fk_users) REFERENCES users(id),
  CONSTRAINT subscribers_tournaments FOREIGN KEY (fk_tournaments) REFERENCES tournaments(id)
)