use std::path::PathBuf;

#[derive(Debug)]
pub(crate) enum SettingsError {
    EnvVarMissing(&'static str),
    Config(config::ConfigError),
}

pub(crate) struct Settings {
    pub(crate) cheatsheets_path: PathBuf,
    pub(crate) width: usize,
    pub(crate) editor: String,
}

impl Settings {
    pub(crate) fn load() -> Result<Self, SettingsError> {
        let mut settings_builder = config::Config::builder()
            .set_default("cheatsheets_path", get_default_cheatsheets_path()?)
            .map_err(SettingsError::Config)?
            .set_default("width", get_default_total_width())
            .map_err(SettingsError::Config)?
            .set_default("editor", get_default_editor()?)
            .map_err(SettingsError::Config)?;

        settings_builder = settings_builder.add_source(config::Environment::with_prefix("SCHEAT"));

        let settings = settings_builder.build().map_err(SettingsError::Config)?;

        settings.try_into()
    }
}

impl TryFrom<config::Config> for Settings {
    type Error = SettingsError;

    fn try_from(value: config::Config) -> Result<Self, Self::Error> {
        let cheatsheets_path = value
            .get_string("cheatsheets_path")
            .map_err(SettingsError::Config)?;

        let width = value.get_int("width").map_err(SettingsError::Config)?;

        let editor = value.get_string("editor").map_err(SettingsError::Config)?;

        Ok(Self {
            cheatsheets_path: cheatsheets_path.into(),
            width: width as usize,
            editor,
        })
    }
}

fn get_default_cheatsheets_path() -> Result<String, SettingsError> {
    let home = std::env::var("HOME")
        .map_err(|_| SettingsError::EnvVarMissing("HOME Environment Variable not found"))?;

    Ok(format!("{home}/.cheatsheets"))
}

fn get_default_total_width() -> u64 {
    80
}

fn get_default_editor() -> Result<String, SettingsError> {
    std::env::var("EDITOR")
        .map_err(|_| SettingsError::EnvVarMissing("EDITOR Environment Variable not found"))
}
