-- Your SQL goes here
CREATE TABLE tournaments (
  id INTEGER PRIMARY KEY AUTO_INCREMENT,
  fk_users INTEGER NOT NULL,
  name VARCHAR(255) NOT NULL,
  description VARCHAR(255) NOT NULL,
  date DATETIME,
  location VARCHAR(255),
  phase INTEGER NOT NULL,
  size_group INTEGER,
  code VARCHAR(16) NOT NULL,

  CONSTRAINT tournaments_users FOREIGN KEY (fk_users) REFERENCES users(id)
)