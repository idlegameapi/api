CREATE TABLE "users" (
    "username" VARCHAR(100) NOT NULL UNIQUE,
    "hashed_password" TEXT NOT NULL UNIQUE,
    "salt" TEXT NOT NULL,
    "balance" DOUBLE PRECISION DEFAULT 0,
    "level" INTEGER DEFAULT 0,
    "collected_timestamp" TIMESTAMP(3) DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT "users_pkey" PRIMARY KEY ("username")
);
