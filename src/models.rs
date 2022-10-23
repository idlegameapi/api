use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "users")]
pub struct User {
    pub username: String,
    pub hashed_password: String,
    pub salt: String,
    pub balance: f64,
    pub level: i32,
    pub collected_timestamp: SystemTime,
}

impl From<User> for UserWithoutSecrets {
    fn from(user: User) -> Self {
        UserWithoutSecrets {
            username: user.username,
            balance: user.balance,
            level: user.level,
            collected_timestamp: user.collected_timestamp,
        }
    }
}

#[derive(Serialize)]
pub struct UserWithoutSecrets {
    pub username: String,
    pub balance: f64,
    pub level: i32,
    pub collected_timestamp: SystemTime,
}

pub struct UpgradeableLevelsOutput {
    pub level: i32,
    pub cost: f64,
}

pub trait GameCalculations {
    const BASE_COST: f64;
    const BASE_PRODUCTION: f64;
    const COST_MULTIPLIER: f64;
    const PRODUCTION_MULTIPLIER: f64;

    fn get_production(&self) -> f64;
    fn next_level_cost(&self) -> f64;
    fn specific_level_cost(&self, level: i32) -> f64;
    fn upgradeable_levels(&self) -> Result<UpgradeableLevelsOutput, ()>;
}

impl GameCalculations for User {
    const BASE_COST: f64 = 5.0;
    const BASE_PRODUCTION: f64 = 1.0;
    const COST_MULTIPLIER: f64 = 1.15;
    const PRODUCTION_MULTIPLIER: f64 = 1.1;

    /// calculates the production of the user
    fn get_production(&self) -> f64 {
        let mut production = Self::BASE_PRODUCTION;
        for progressed_level in 1..self.level {
            production += Self::BASE_PRODUCTION * Self::PRODUCTION_MULTIPLIER.powi(progressed_level);
        }
        production
    }

    /// Calculates the cost of the next level
    fn next_level_cost(&self) -> f64 {
        Self::BASE_COST * Self::COST_MULTIPLIER.powi(self.level)
    }

    /// Calculates the cost of a specific level, but keep in mind that this calculates by the level after the specified level.
    fn specific_level_cost(&self, level: i32) -> f64 {
        Self::BASE_COST * Self::COST_MULTIPLIER.powi(level)
    }

    /// Calculates the amount of possible levels you can upgrade to.
    fn upgradeable_levels(&self) -> Result<UpgradeableLevelsOutput, ()> {
        let mut level = self.level;
        let mut cost = self.next_level_cost();

        if self.balance < cost {
            return Err(());
        }

        while self.balance >= cost {
            level += 1;
            cost += self.specific_level_cost(self.level);
        }
        level -= 1;
        cost -= self.specific_level_cost(self.level);

        Ok(UpgradeableLevelsOutput { level, cost })
    }
}
