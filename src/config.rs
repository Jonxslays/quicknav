use anyhow::Result;
use std::env;
use std::fs;
use std::fs::File;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct Shortcut {
    pub name: String,
    pub description: String,
    pub location: String,
    pub calls: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Options {
    #[serde(default)]
    pub create_missing_directories: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub shortcuts: Vec<Shortcut>,
    #[serde(default)]
    pub options: Options,
}

impl Config {
    fn sep<'a>() -> &'a str {
        if env::consts::OS == "windows" {
            "\\"
        } else {
            "/"
        }
    }

    fn get_config_dir_path() -> String {
        let sep = Config::sep();
        let config_dir = env::var("XDG_CONFIG_HOME")
            .or_else(|_| env::var("HOME").map(|home| format!("{}{}.config", home, sep)))
            .or_else(|_| env::var("HOMEPATH").map(|home| format!("{}{}.config", home, sep)))
            .unwrap();

        format!("{}{}quicknav", config_dir, sep)
    }

    fn get_config_file_path() -> String {
        let config_dir = Config::get_config_dir_path();
        let sep = Config::sep();

        format!("{}{}quicknav.json", config_dir, sep)
    }

    fn generate() -> Result<i32> {
        let config_dir = Config::get_config_dir_path();
        let config_file = Config::get_config_file_path();

        if !Path::new(&config_dir).exists() {
            fs::create_dir_all(&config_dir)?;
        }

        fs::write(
            config_file,
            r#"{
    "shortcuts": [],
    "options": {
        "create_missing_directories": false
    }
}"#,
        )?;

        Ok(0)
    }

    pub fn load() -> Result<Config> {
        let config_file = Config::get_config_file_path();

        if !Path::new(&config_file).exists() {
            Config::generate()?;
        }

        let data = File::open(config_file)?;
        let config: Config = serde_json::from_reader(data)?;

        Ok(config)
    }

    pub fn update(&self) -> Result<i32> {
        let config_file = Config::get_config_file_path();

        fs::write(config_file, serde_json::to_string_pretty(self).unwrap())?;

        Ok(0)
    }
}
