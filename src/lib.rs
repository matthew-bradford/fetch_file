extern crate ron;
#[allow(unused_imports)]
#[macro_use]
extern crate serde;
extern crate serde_json;

use std::error::Error;
use std::fs::{ File };
use std::io::{Read, Write};
use std::path::PathBuf;

use ron::de::from_str;
use serde::de::DeserializeOwned;

///
/// # Fetch
///
/// Simple trait that can be implemented on structs/enums for
/// serialisation to and from disk.
///
/// * deserialize_l: impl to pick specific
///
/// * serialize_l: ..
///
/// ```
/// use std::env;
/// use std::error::Error;
/// use std::path::PathBuf;
/// use serde::*;
/// use serde::de::DeserializeOwned;
/// use fetch_file::Fetchable;
///
/// #[derive(Deserialize, Serialize)]
/// pub struct Config {
///     setting1: usize,
///     setting2: usize
/// }
///
/// impl Default for Config {
///     fn default() -> Self {
///         Config {
///             setting1: 0,
///             setting2: 5,
///         }
///     }
/// }
///
/// impl Fetchable for Config {
///     fn deserialize_l<T>(f_path: &PathBuf) -> Result<T, Box<dyn Error>>
///         where T: DeserializeOwned + Default + Fetchable {
///         //Config::deserialize_ron(f_path)
///         //Config::deserialize_json(f_path)
///         Config::deserialize_bin(f_path)
///      }
///
///     fn serialize_l(&self) -> Result<Vec<u8>, Box<dyn Error>>
///         where Self: serde::Serialize + Fetchable {
///         //self.serialize_ron()
///         //self.serialize_json()
///         self.serialize_bin()
///     }
/// }
///
/// fn main() -> std::result::Result<(),  Box<dyn Error>> {
///     // Example directory
///     let mut path = env::current_dir()?;
///     // adding file name
///     path.push("config.bin");
///     // fetch or default will either open file from disk and deserialize
///     // or return the default for Config and a boolean indicating the
///     // config is default.
///     let config: (Config, bool) = Config::fetch_or_default(&path)?;
///     if config.1 {
///         config.0.save(&path);
///     }
///     let config = config.0;
///     println!("Config: {}, {}", config.setting1, config.setting2);
///     Ok(())
/// }
///
/// ```
///
pub trait Fetchable {
    ///
    /// # Impl method
    ///
    /// Impl this method to define behavior.
    ///
    fn deserialize_l<T: DeserializeOwned + Default + Fetchable>(
        f_path: &PathBuf,
    ) -> Result<T, Box<dyn Error>>;

    ///
    /// # Bin
    ///
    /// Deserialize from bincode format.
    ///
    /// * f_path: Path to file to open /home/$USER/folder/file.txt
    ///
    /// * Panics if it fails to open file.
    ///
    fn deserialize_bin<T>(f_path: &PathBuf) -> Result<T, Box<dyn Error>>
        where T: DeserializeOwned + Default {
        let mut file = File::open(f_path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        let item: T = match bincode::deserialize(&buffer) {
            Ok(item) => item,
            Err(_e) => T::default(),
        };
        buffer.clear();
        Ok(item)
    }

    ///
    /// # Json
    ///
    /// Deserialize from Json format.
    ///
    /// * f_path: Path to file to open /home/$USER/folder/file.txt
    ///
    /// * Panics if it fails to open file.
    ///
    fn deserialize_json<T>(f_path: &PathBuf) -> Result<T, Box<dyn Error>>
        where T: DeserializeOwned + Default {
        let mut file = File::open(f_path)?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;
        let item: T = match serde_json::from_str(&buffer) {
            Ok(item) => item,
            Err(_e) => T::default(),
        };
        buffer.clear();
        Ok(item)
    }

    ///
    /// # RON
    ///
    /// Deserialize from ron format.
    ///
    /// * f_path: Path to file to open /home/$USER/folder/file.txt
    ///
    /// * Panics if it fails to open file.
    ///
    fn deserialize_ron<T>(f_path: &PathBuf) -> Result<T, Box<dyn Error>>
        where T: DeserializeOwned + Default {
        let mut file = File::open(f_path)?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;
        let item: T = match from_str(&buffer) {
            Ok(item) => item,
            Err(_e) => T::default(),
        };
        buffer.clear();
        Ok(item)
    }

    ///
    /// # Impl method
    ///
    /// Impl this method to define behavior.
    ///
    fn serialize_l(&self) -> Result<Vec<u8>, Box<dyn Error>>
    where
        Self: serde::Serialize + Fetchable;

    ///
    /// # Bin
    ///
    fn serialize_bin(&self) -> Result<Vec<u8>, Box<dyn Error>>
    where
        Self: serde::Serialize,
    {
        let mut buf = vec![];
        buf.clear();
        buf = bincode::serialize(self)?;
        Ok(buf)
    }

    ///
    /// # RON
    ///
    fn serialize_ron(&self) -> Result<Vec<u8>, Box<dyn Error>>
    where
        Self: serde::Serialize,
    {
        Ok(
            ron::ser::to_string_pretty(&self, ron::ser::PrettyConfig::default())
                .expect("Failed pretty string.")
                .as_bytes()
                .to_vec(),
        )
    }

    ///
    /// # Json
    ///
    fn serialize_json(&self) -> Result<Vec<u8>, Box<dyn Error>>
    where
        Self: serde::Serialize,
    {
        Ok(
            serde_json::ser::to_string_pretty(&self)
                .expect("Failed pretty string.")
                .as_bytes()
                .to_vec(),
        )
    }

    ///
    /// # Save
    ///
    /// Use this version of save when container cannot be implemented.
    ///
    fn save(&self, path: &PathBuf) -> Result<(), Box<dyn Error>>
    where
        Self: serde::Serialize + Fetchable,
    {
        let mut f = File::create(&path)?;
        let content = self.serialize_l()?;
        f.write_all(content.as_slice())?;
        match f.sync_all() {
            Ok(_) => Ok(()),
            Err(e) => Err(Box::new(e)),
        }
    }

    ///
    /// Fetch an item serialized to file
    /// # Arguments
    ///
    /// * `file_path` - Reference to path buffer of file of type T
    ///
    /// # Returns
    ///
    /// * `Result<(T, bool)>` - Returns tuple containing T and a bool value indicating if a config was retrieved or default. True for default config use.
    ///
    fn fetch_or_default<T>(file_path: &PathBuf) -> Result<(T, bool), Box<dyn Error>>
        where T: Default + DeserializeOwned + Fetchable {
        let path = file_path.to_path_buf();
        let result: (T, bool) = if path.exists() {
            match T::deserialize_l(&path) {
                Ok(t) => (t, false),
                Err(_) => (T::default(), true),
            }
        } else {
            (T::default(), true)
        };

        Ok(result)
    }
}
