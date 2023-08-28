-- Your SQL goes here
CREATE TABLE tournaments (
  id INTEGER PRIMARY KEY AUTO_INCREMENT,
  fk_users INTEGER NOT NULL,
  name VARCHAR(255) NOT NULL,
  description VARCHAR(255) NOT NULL,
  date DATE,
  location VARCHAR(255),
  phase INTEGER NOT NULL,
  sizeGroupe INTEGER,

  CONSTRAINT tournaments_users FOREIGN KEY (fk_users) REFERENCES users(id)
)