use lazy_static::lazy_static;
use serde_derive::Deserialize;
use std::fs::File;
use std::io::Read;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub listen_address: String,
    pub listen_port: String,
    pub jwt_secret: String,
    pub jwt_expire: i64,
    pub database: DB,
    pub redis: Redis,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Redis {
    pub host: String,
    pub port: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DB {
    pub url: String,
}

impl Default for Config {
    fn default() -> Self {
        let file_path = "./src/etc/config.toml";
        Config::read_from_file(file_path)
    }
}

impl Config {
    fn read_from_file(path: &str) -> Config {
        let file_path = path;
        let mut file = match File::open(file_path) {
            Ok(f) => f,
            Err(e) => panic!("no such file {} exception:{}", file_path, e),
        };

        let mut str_val = String::new();
        match file.read_to_string(&mut str_val) {
            Ok(s) => s,
            Err(e) => panic!("error reading file:{}", e),
        };
        toml::from_str(&str_val).unwrap()
    }
    pub fn get<'a>() -> &'a Self {
        lazy_static! {
            static ref CACHE: Config = Config::default();
        }
        &CACHE
    }
}
