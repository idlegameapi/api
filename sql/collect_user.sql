UPDATE users
SET "balance" = $2,
  "collected_timestamp" = $3
WHERE "username" = $1
RETURNING *;
