#[derive(serde::Deserialize)]
pub struct Settings {
    pub output_size: String,
    pub max_lines_output: u64,
    pub duration: String,
    pub output_format: String,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let settings = config::Config::builder()
        .add_source(config::File::with_name("configuration").required(false))
        .add_source(config::Environment::with_prefix("LOG"))
        .set_default("max_lines_output", 0)?
        .set_default("output_format", "plaintext")?
        .build()?;
    settings.try_deserialize::<Settings>()
}
