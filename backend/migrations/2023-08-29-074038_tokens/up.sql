-- Your SQL goes here
CREATE TABLE tokens (
    token VARCHAR(255) PRIMARY KEY,
    fk_users INT NOT NULL,
    created_at DATETIME NOT NULL DEFAULT NOW(),
    expiration_date DATETIME NOT NULL,

    FOREIGN KEY (fk_users) REFERENCES users(id)
);

CREATE TRIGGER drop_trigger
BEFORE INSERT ON tokens
FOR EACH ROW
BEGIN
    DELETE FROM tokens WHERE expiration_date < NOW();
END;