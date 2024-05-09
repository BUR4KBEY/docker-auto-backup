use std::{error::Error, fs::File, io::Read, str::FromStr};

use serde::{Deserialize, Deserializer};
use tracing::error;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub containers: Vec<ContainerConfig>,
}

#[derive(Debug, Deserialize)]
pub struct ContainerConfig {
    pub name: String,

    #[serde(deserialize_with = "deserialize_file_config")]
    pub files: Vec<BackupFile>,

    #[serde(default, deserialize_with = "deserialize_script")]
    pub pre_build_script: Option<Vec<String>>,

    #[serde(default, deserialize_with = "deserialize_script")]
    pub post_build_script: Option<Vec<String>>,
}

#[derive(Debug)]
pub struct BackupFile(pub String, pub String);

impl Default for ContainerConfig {
    fn default() -> Self {
        ContainerConfig {
            name: String::new(),
            files: Vec::new(),
            pre_build_script: None,
            post_build_script: None,
        }
    }
}

impl std::str::FromStr for BackupFile {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(':').collect();

        if parts.len() != 2 {
            return Err("invalid format");
        }

        Ok(BackupFile(parts[0].to_string(), parts[1].to_string()))
    }
}

fn read_config() -> Result<String, Box<dyn Error>> {
    let mut file = File::open("config.yml")?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    Ok(contents)
}

pub fn get_config() -> Option<Config> {
    match read_config() {
        Ok(config) => match serde_yaml::from_str(&config) {
            Ok(parsed_config) => Some(parsed_config),
            Err(e) => {
                error!("error parsing config: {}", e);
                None
            }
        },
        Err(_) => None,
    }
}

fn deserialize_file_config<'de, D>(deserializer: D) -> Result<Vec<BackupFile>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Vec<String> = Deserialize::deserialize(deserializer)?;
    let mut files = Vec::new();

    for item in s {
        if let Ok(file_config) = BackupFile::from_str(&item) {
            files.push(file_config);
        }
    }

    Ok(files)
}

fn deserialize_script<'de, D>(deserializer: D) -> Result<Option<Vec<String>>, D::Error>
where
    D: Deserializer<'de>,
{
    let script: Option<String> = Option::deserialize(deserializer)?;

    match script {
        Some(script) => {
            let script_lines: Vec<String> = script.lines().map(String::from).collect();
            Ok(Some(script_lines))
        }

        None => Ok(None),
    }
}
