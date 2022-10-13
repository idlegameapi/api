INSERT INTO users (username, token, salt, balance, collected_timestamp)
VALUES ($1, $2, $3, 0, $4)
RETURNING *;
