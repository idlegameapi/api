INSERT INTO users (username, token, salt, balance, collected_timestamp)
VALUES ($1, $2, $3, $4, $5) RETURNING *;