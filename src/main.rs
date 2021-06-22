use std::error::Error;
use std::env;
use std::fs;

use serde::{Deserialize, Serialize};
use toml;
use std::process::exit;


#[derive(Serialize, Deserialize, Debug, Clone)]
struct ExtraSection {
    pub another_number: usize,
    pub another_parameter: String,
}

impl Default for ExtraSection {
    fn default() -> Self {
        ExtraSection {
            another_number: 33,
            another_parameter: "default_extra".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(default)]
struct Config {
    pub some_number: usize,
    pub some_parameter: String,
    pub extra_section: ExtraSection,
}

impl Config {
    pub fn from_file(path: &str) -> Result<Config, Box<dyn Error>> {
        let data = fs::read_to_string(path)?;
        let config = toml::from_str(&data)?;
        Ok(config)
    }

    pub fn generate_default_toml() -> Result<String, Box<dyn Error>> {
        let config = Config::default();
        Ok(toml::to_string::<Config>(&config)?)
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            some_number: 111,
            some_parameter: "default top level".to_string(),
            extra_section: ExtraSection::default(),
        }
    }
}


fn main() {
    if env::args().len() != 2 {
        println!("There should be exactly one argument");
        exit(1);
    }
    let path = env::args().nth(1).unwrap();
    let conf = Config::from_file(&path);
    println!("Parsed toml: {:?}", conf);
    println!("===========");
    let default_toml = Config::generate_default_toml().unwrap();
    println!("{}", default_toml);
}
