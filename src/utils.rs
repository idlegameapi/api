pub fn env_get(iteration: &[(String, String)], key_search: &'static str) -> String {
    match iteration.iter().find(|(key, _)| key == key_search) {
        Some((_, value)) => value.to_string(),
        None => panic!(
            "couldn't find '{}' in the environment variables",
            key_search
        ),
    }
}
