use std::path::PathBuf;

#[derive(Debug)]
pub enum ConfigError {
    EnvVarMissing(&'static str),
    ConfigError(config::ConfigError)
}

pub struct Settings {
    pub cheatsheets_path: PathBuf
}

impl TryFrom<config::Config> for Settings {
    type Error = ConfigError;

    fn try_from(value: config::Config) -> Result<Self, Self::Error> {
        let cheatsheets_path = value.get_string("cheatsheets_path").map_err(|e| ConfigError::ConfigError(e))?;

        Ok(Self {
            cheatsheets_path: cheatsheets_path.into()
        })
    }
}

pub fn get_config() -> Result<Settings, ConfigError> {
    let home = std::env::var("HOME")
        .map_err(|_| ConfigError::EnvVarMissing("HOME Environment Variable not found"))?;

    let settings_builder = config::Config::builder()
        .set_default("cheatsheets_path", format!("{}/.cheatsheets", home)).map_err(|e| ConfigError::ConfigError(e))?;


    let settings = settings_builder.build().map_err(|e| ConfigError::ConfigError(e))?;

    settings.try_into()
}

