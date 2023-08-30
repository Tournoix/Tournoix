-- Your SQL goes here
CREATE TABLE subscriptions (
  id INTEGER PRIMARY KEY AUTO_INCREMENT,
  fk_users INTEGER NOT NULL,
  fk_tournaments INTEGER NOT NULL,

  CONSTRAINT subscriptions_users FOREIGN KEY (fk_users) REFERENCES users(id),
  CONSTRAINT subscriptions_tournaments FOREIGN KEY (fk_tournaments) REFERENCES tournaments(id)
)