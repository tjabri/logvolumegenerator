use rand::distr::{Alphanumeric, SampleString};
use std::error::Error;
use std::fmt;

#[derive(PartialEq, Debug)]
pub enum RuntimeError {
    InvalidArgument,
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let description = match *self {
            RuntimeError::InvalidArgument => "Invalid argument",
        };
        f.write_str(description)
    }
}

impl Error for RuntimeError {}

pub fn generate_sequential_string(byte_size: usize) -> String {
    if byte_size == 0 {
        return String::new();
    }
    Alphanumeric.sample_string(&mut rand::rng(), byte_size)
}

pub fn create_plaintext_log_entry(s: &String) -> String {
    format!(
        "{} - INFO - {}",
        chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
        s.as_str()
    )
}

pub fn create_json_log_entry(s: &String) -> String {
    format!(
        "{{\"timestamp\": \"{}\", \"level\": \"INFO\", \"message\": \"{}\"}}",
        chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
        s.as_str()
    )
}

pub fn get_log_entry_printer(format: String) -> fn(&String) -> String {
    match format.as_str() {
        "plaintext" => create_plaintext_log_entry,
        "json" => create_json_log_entry,
        _ => create_plaintext_log_entry,
    }
}

pub fn calculate_iteration_pause(
    total_output_bytes: u64,
    total_output_duration_secs: u64,
    entry_size_bytes: usize,
) -> Result<std::time::Duration, RuntimeError> {
    if total_output_bytes == 0 || total_output_duration_secs == 0 || entry_size_bytes == 0 {
        return Err(RuntimeError::InvalidArgument);
    }
    let lines_per_sec =
        (total_output_bytes as f64 / entry_size_bytes as f64) / total_output_duration_secs as f64;
    let secs_per_line = 1f64 / lines_per_sec;
    Ok(std::time::Duration::from_secs_f64(secs_per_line))
}
