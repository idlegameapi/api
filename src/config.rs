use crate::prelude::*;
use dotenv::vars;

#[derive(Debug)]
pub struct Config {
    pub pg: deadpool_postgres::Config,
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

impl Config {
    pub fn new() -> Self {
        let environment_vars: Vec<(String, String)> = vars().collect();
        let mut database_config = deadpool_postgres::Config::new();
        Config::setup_pg_config(&mut database_config, &environment_vars);
        Config {
            pg: database_config,
        }
    }

    fn setup_pg_config<'a>(
        db_config: &'a mut deadpool_postgres::Config,
        env_vars: &'a [(String, String)],
    ) -> &'a mut deadpool_postgres::Config {
        db_config.user = Some(env_get(env_vars, "PG_USER"));
        db_config.password = Some(env_get(env_vars, "PG_PASSWORD"));
        db_config.host = Some(env_get(env_vars, "PG_HOST"));
        db_config.port = Some(env_get(env_vars, "PG_PORT").parse().unwrap());
        db_config.dbname = Some(env_get(env_vars, "PG_DBNAME"));
        db_config
    }
}
