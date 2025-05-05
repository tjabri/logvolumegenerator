#[derive(serde::Deserialize)]
pub struct Settings {
    pub total_output_bytes: u64,
    pub total_output_duration_secs: u64,
    pub max_lines_output: u64,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let settings = config::Config::builder()
        .add_source(
            config::File::with_name("configuration")
                .required(false)
        )
        .add_source(config::Environment::with_prefix("LOGSIM"))
        .set_default("max_lines_output", 0)?
        .build()?;
    settings.try_deserialize::<Settings>()
}