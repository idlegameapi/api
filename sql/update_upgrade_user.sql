UPDATE users
SET "balance" = $2,
  "level" = $3
WHERE "username" = $1
RETURNING *;