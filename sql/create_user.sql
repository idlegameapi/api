INSERT INTO users (username, token, salt)
VALUES ($1, $2, $3)
RETURNING *;
