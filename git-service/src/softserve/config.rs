use std::env;
use std::path::PathBuf;

pub fn get_softserve_host() -> String {
    env::var("SOFTSERVE_HOST").unwrap_or_else(|_| "localhost".to_string())
}

pub fn get_softserve_port() -> u16 {
    env::var("SOFTSERVE_PORT")
        .unwrap_or_else(|_| "23231".to_string())
        .parse()
        .expect("SOFTSERVE_PORT must be a number")
}

pub fn get_softserve_user() -> String {
    env::var("SOFTSERVE_USER").unwrap_or_else(|_| "admin".to_string())
}

pub fn get_softserve_key_path() -> String {
    env::var("SOFTSERVE_KEY_PATH").unwrap_or_else(|_| "~/.ssh/id_ed25519".to_string())
}

pub fn expand_tilde(path: &str) -> PathBuf {
    if path.starts_with("~") {
        let home = env::var("HOME").expect("HOME environment variable not set");
        PathBuf::from(home).join(&path[2..])
    } else {
        PathBuf::from(path)
    }
}
