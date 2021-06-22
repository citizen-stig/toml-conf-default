use std::error::Error;
use std::env;
use std::fs;

use serde::{Deserialize, Serialize};
use toml;
use std::process::exit;


#[derive(Serialize, Deserialize, Debug, Clone)]
struct ExtraSection {
    pub another_number: usize,
    pub another_parameter: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Config {
    pub some_number: usize,
    pub some_parameter: String,
    pub extra_section: ExtraSection,

}

impl Config {
    pub fn from_file(path: &str) -> Result<Config, Box<dyn Error>> {
        let data = fs::read_to_string(path)?;
        println!("data: {:?}", data);
        let config = toml::from_str(&data)?;
        Ok(config)
    }
}



fn main() {
    println!("Hello, world!");
    println!("{:?}", env::args().len());
    if env::args().len() != 2 {
        println!("There should be exactly one argument");
        exit(1);
    }
    let path = env::args().nth(1).unwrap();
    let conf = Config::from_file(&path);
    println!("Conf: {:?}", conf)
}
