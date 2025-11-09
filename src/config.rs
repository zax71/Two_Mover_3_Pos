use std::{
    fs::{self, OpenOptions},
    io::{Read, Write},
    path::PathBuf,
};

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::components::preferences::osc_preferences::OscPreferences;

pub struct ConfigFile {
    path: PathBuf,
    config: Config,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Config {
    pub osc: OscPreferences,
}

impl ConfigFile {
    /// Creates a new config file if one does not exist at the given location. If one exists then it's contents is read and returned as a `ConfigFile` struct
    /// path: where to put the file. It should be a file ending in `.toml`.
    pub fn new(path: PathBuf) -> Result<Self> {
        let mut file = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .open(&path)?;

        // If there is no config file yet, write default values to it
        if file.metadata()?.len() == 0 {
            file.write_all(toml::to_string_pretty(&Config::default())?.as_bytes())?;
            // Wait for metadata to be written to disk
            file.sync_all()?;
        }

        // Get file contents
        let mut file_contents = "".to_string();
        file.read_to_string(&mut file_contents)?;

        // Create the config object with a new serialised copy of the config file
        Ok(Self {
            path,
            config: toml::from_str(&file_contents)?,
        })
    }

    /// Writes the specified config to the file & updates in memory state of config
    pub fn write_osc(&mut self, new_osc_config: OscPreferences) -> Result<()> {
        self.config.osc = new_osc_config;

        fs::write(&self.path, toml::to_string_pretty(&self.config)?)?;

        Ok(())
    }

    pub fn read(&mut self) -> Result<Config> {
        self.config = toml::from_str(&fs::read_to_string(&self.path)?)?;
        Ok(self.config.clone())
    }
}
