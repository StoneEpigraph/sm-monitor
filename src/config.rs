use std::fs;
use std::io::Error;
use log::info;
use toml::Value;

pub fn parse_config() -> Result<Vec::<Config>, Error> {

    // read config file
    let contents = fs::read_to_string("config.toml").expect("Failed to read config file, filename: config.toml");
    let config: Value = toml::from_str(&contents).expect("Failed to parse config file");

    let mut res = vec![];
    if let Some(monitors) = config.get("monitors") {

        let monitors = monitors.as_array().expect("Failed to parse monitors array in config file");
        monitors.iter().for_each(|monitor| {
            let name = monitor.get("name").expect("Failed to parse name in monitor config");
            let path = monitor.get("path").expect("Failed to parse path in monitor config");
            let exclude_types = monitor.get("exclude_types").expect("Failed to parse exclude_types in monitor config");
            let exclude_dirs = monitor.get("exclude_dirs").expect("Failed to parse exclude_dirs in monitor config");
            res.push(Config::new(
                name.as_str().unwrap().to_string(),
                path.as_str().unwrap().to_string(),
                exclude_types.as_array().unwrap().iter().map(|v| v.as_str().unwrap().to_string()).collect(),
                exclude_dirs.as_array().unwrap().iter().map(|v| v.as_str().unwrap().to_string()).collect()
            ));
        });
    }
    Ok(res)
}

#[derive(Debug)]
pub struct Config {
    // TODO: Define Config struct
    pub name: String,
    pub path: String,
    pub exclude_types: Vec<String>,
    pub exclude_dirs: Vec<String>,
}

impl Config {
    pub fn new(name: String, path: String, exclude_types: Vec<String>, exclude_dirs: Vec<String>) -> Self {
        Config {
            name,
            path,
            exclude_types,
            exclude_dirs,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            name: "".to_string(),
            path: "/".to_string(),
            exclude_types: vec![".swp".to_string()],
            exclude_dirs: vec![".git".to_string()],
        }
    }
}

// 写一个测试类
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_config() {
        let config = parse_config();
        assert_eq!(config.is_ok(), true);
    }
}