-- TODO(SirH): remove the editing of collected timestamp once Toga actually provides me the User sql that is lost in our discord server
INSERT INTO users (username, token, salt, balance, collected_timestamp)
VALUES ($1, $2, $3, 0, $4)
RETURNING *;
