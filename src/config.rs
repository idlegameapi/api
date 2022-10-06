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
        db_config.user = Some(find_key(env_vars, "DBUSER"));
        db_config.password = Some(find_key(env_vars, "PASSWORD"));
        db_config.host = Some(find_key(env_vars, "HOST"));
        db_config.port = Some(find_key(env_vars, "PORT").parse().unwrap());
        db_config.dbname = Some(find_key(env_vars, "DBNAME"));
        db_config
    }
}

pub fn find_key(iteration: &[(String, String)], key_search: &'static str) -> String {
    match iteration.iter().find(|(key, _)| key == key_search) {
        Some((_, value)) => value.to_string(),
        None => panic!(
            "couldn't find '{}' in the environment variables",
            key_search
        ),
    }
}
