use dotenv::vars;
// TODO: implement other import environment variables here, perhaps improve this config file to not need creation every time
#[derive(Debug)]
pub struct Config {
    pub db_url: String,
}
impl Config {
    pub fn new() -> Self {
        let environment_vars: Vec<(String, String)> = vars().collect();
        Config {
            db_url: find_key(&environment_vars, "DATABASE_URL"),
        }
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
