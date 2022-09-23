use dotenv::vars;

#[derive(Debug)]
pub struct Config {
    pub token_hasher: String,
    pub server_addr: String,
    pub pg: deadpool_postgres::Config,
}
impl Config {
    pub fn new() -> Self {
        let environment_vars: Vec<(String, String)> = vars().collect();
        let mut database_config = deadpool_postgres::Config::new();
        Config::setup_pg_config(&mut database_config, &environment_vars);
        Config {
            token_hasher: find_key(&environment_vars, "TOKEN_HASHER"),
            server_addr: find_key(&environment_vars, "SERVER_ADDR"),
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
