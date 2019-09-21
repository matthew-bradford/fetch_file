# Fetch File

Quick trait to just impl on struct to turn it into a quick config file. Json, Ron, or Bincode can be used.

```rust
use std::env;
use std::error::Error;
use std::path::PathBuf;
use serde::*;
use serde::de::DeserializeOwned;

use fetch_file::Fetchable;

#[derive(Deserialize, Serialize)]
pub struct Config {
    setting1: usize,
    setting2: usize
}

impl Default for Config {
    fn default() -> Self {
        Config {
            setting1: 0,
            setting2: 5,
        }
    }
}

impl Fetchable for Config {
    fn deserialize_l<T>(f_path: &PathBuf) -> Result<T, Box<dyn Error>>
        where T: DeserializeOwned + Default + Fetchable {
        // Ron
        Config::deserialize_ron(f_path)
        // Json
        //Config::deserialize_json(f_path)
        // bin
        //Config::deserialize_bin(f_path)
    }
    fn serialize_l(&self) -> Result<Vec<u8>, Box<dyn Error>>
        where Self: serde::Serialize + Fetchable {
        // Ron
         self.serialize_ron()
        // Json
        // self.serialize_json()
        // Bin
        // self.serialize_bin()
    }
}

fn main() -> std::result::Result<(),  Box<dyn Error>> {
    // Example directory
    let mut path = env::current_dir()?;
    // adding file name
    path.push("config.ron");
    // fetch or default will either open file from disk and deserialize
    // or return the default for Config and a boolean indicating the
    // config is default.
    let config: (Config, bool) = Config::fetch_or_default(&path)?;
    if config.1 {
        config.0.save(&path);
    }
    let config = config.0;
    println!("Config: {}, {}", config.setting1, config.setting2);
    Ok(())
}

```