use std::time::{Duration, Instant};
use config::ConfigError;
use crate::configuration::get_configuration;
use chrono;

mod configuration;

use rand::distr::{Alphanumeric, SampleString};

fn generate_sequential_string(byte_size: usize) -> String {
    if byte_size == 0 {
        return String::new();
    }
    let result = Alphanumeric.sample_string(&mut rand::rng(), byte_size);
    result
}

fn create_log_entry(s: &String) -> String {
    format!("{} - INFO - {}", chrono::Utc::now().to_rfc3339(),  s.as_str())
}

fn main() -> Result<(), ConfigError> {
    // get configuration
    let config = get_configuration()?;

    // compute requested log throughput
    let throughput = config.total_output_bytes / config.total_output_duration_secs;
    println!("Desired log throughput: {:.2} B/s", throughput);
    
    // generate a string
    let s = generate_sequential_string(80);
    let entry = create_log_entry(&s);
    
    // calculate entry size
    let entry_size_bytes = entry.len();
    
    // print entry size
    println!("Log entry size: {} bytes", entry_size_bytes);
    
    // print entry
    println!("{}", entry);
    
    // calculate how many entries per second we need to generate
    let output_rate = throughput as f64 / entry_size_bytes as f64;
    println!("Need to write {:.2} entries/s to satisfy requirement", output_rate);

    let target_duration = Duration::from_secs_f64(output_rate);
    println!("target duration: {:.2}", target_duration.as_millis());

    let mut lines_output: u64 = 0;
    
    loop {
        println!("{}", entry);
        lines_output += 1;
        
        if lines_output >= config.max_lines_output {
            return Ok(());
        }
        // Calculate sleep time
        let elapsed = Instant::now().elapsed();
        if let Some(sleep_time) = target_duration.checked_sub(elapsed) {
            std::thread::sleep(sleep_time);
        }
    }
    // Ok(())
}
