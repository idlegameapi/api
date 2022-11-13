use crate::prelude::*;
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
        for progressed_level in 1..=self.level {
            production += (Self::BASE_PRODUCTION
                * Self::PRODUCTION_MULTIPLIER.powi(progressed_level))
            .round();
        }
        production
    }

    /// Calculates the cost of the next level
    fn next_level_cost(&self) -> f64 {
        (Self::BASE_COST * Self::COST_MULTIPLIER.powi(self.level)).round()
    }

    /// Calculates the cost of a specific level, but keep in mind that this calculates by the level after the specified level.
    fn specific_level_cost(&self, level: i32) -> f64 {
        (Self::BASE_COST * Self::COST_MULTIPLIER.powi(level)).round()
    }

    /// Calculates the amount of possible levels you can upgrade to.
    fn upgradeable_levels(&self) -> Result<UpgradeableLevelsOutput, ()> {
        let mut level = self.level;
        let mut cost = self.next_level_cost();

        if self.balance < cost {
            return Err(());
        }

        while self.balance > cost {
            level += 1;
            cost += self.specific_level_cost(level);
        }
        cost -= self.specific_level_cost(level);
        level -= 1;

        Ok(UpgradeableLevelsOutput { level, cost })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_production() {
        let user = User {
            username: "username".to_string(),
            hashed_password: "hashed_password".to_string(),
            salt: "salt".to_string(),
            balance: 0.0,
            level: 0,
            collected_timestamp: SystemTime::now(),
        };

        assert_eq!(user.get_production(), 1.0);
    }

    #[test]
    fn test_next_level_cost() {
        let user = User {
            username: "username".to_string(),
            hashed_password: "hashed_password".to_string(),
            salt: "salt".to_string(),
            balance: 0.0,
            level: 0,
            collected_timestamp: SystemTime::now(),
        };

        assert_eq!(user.next_level_cost(), 5.0);
    }

    #[test]
    fn test_specific_level_cost() {
        let user = User {
            username: "username".to_string(),
            hashed_password: "hashed_password".to_string(),
            salt: "salt".to_string(),
            balance: 0.0,
            level: 0,
            collected_timestamp: SystemTime::now(),
        };

        assert_eq!(user.specific_level_cost(1), (5.0_f64 * 1.15).round());
        assert_eq!(user.specific_level_cost(2), (5.0_f64 * 1.15 * 1.15).round());
        assert_eq!(
            user.specific_level_cost(3),
            (5.0_f64 * 1.15 * 1.15 * 1.15).round()
        );
    }

    #[test]
    fn test_upgradeable_levels() {
        let user = User {
            username: "username".to_owned(),
            hashed_password: "hashed_password".to_owned(),
            salt: "salt".to_owned(),
            balance: 0.0,
            level: 0,
            collected_timestamp: SystemTime::now(),
        };

        assert_eq!(user.upgradeable_levels().is_err(), true);

        let user = User {
            username: "username".to_owned(),
            hashed_password: "hashed_password".to_owned(),
            salt: "salt".to_owned(),
            balance: 5.0,
            level: 0,
            collected_timestamp: SystemTime::now(),
        };

        assert_eq!(user.upgradeable_levels().is_ok(), true);
    }
}
