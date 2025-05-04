use std::ops::Add;
use std::time::Instant;
use crate::configuration::get_configuration;
use chrono;
use logvolumegenerator::{calculate_iteration_pause, create_log_entry, generate_sequential_string};

mod configuration;

fn main() {
    // get configuration
    let config = get_configuration().unwrap_or_else(|err| {
        println!("configuration error: {}", err);
        std::process::exit(2);
    });
    
    // generate a string
    let s = generate_sequential_string(80);
    let entry = create_log_entry(&s);
    
    // calculate entry size
    let entry_size_bytes = entry.len();
    
    // calculate target duration for loop sleep to achieve required line output rate
    let target_duration = calculate_iteration_pause(config.total_output_bytes, config.total_output_duration_secs, entry_size_bytes).unwrap_or_else(|err| {
        println!("runtime error: {}", err);
        std::process::exit(2);
    });

    let mut lines_output: u64 = 0;
    let end_datetime = chrono::Utc::now().add(chrono::Duration::seconds(config.total_output_duration_secs as i64));
    
    loop {
        println!("{}", entry);
        lines_output += 1;
        
        if (config.max_lines_output != 0 && lines_output >= config.max_lines_output) || (chrono::Utc::now().ge(&end_datetime)) {
            std::process::exit(0);
        }
        // Calculate sleep time
        let elapsed = Instant::now().elapsed();
        if let Some(sleep_time) = target_duration.checked_sub(elapsed) {
            std::thread::sleep(sleep_time);
        }
    }
}
