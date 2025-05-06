use crate::configuration::get_configuration;
use bytesize::ByteSize;
use chrono;
use logvolumegenerator::{
    calculate_iteration_pause, generate_sequential_string, get_log_entry_printer,
};
use parse_duration;
use std::ops::Add;
use std::str::FromStr;
use std::time::Instant;

mod configuration;

fn main() {
    // set ctrl-c handler
    ctrlc::set_handler(move || {
        println!("Caught Ctrl-C... exiting.");
        std::process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    // get configuration
    let config = get_configuration().unwrap_or_else(|err| {
        println!("configuration error: {}", err);
        std::process::exit(2);
    });

    // check output format is valid
    if config.output_format != "plaintext" && config.output_format != "json" {
        println!(
            "invalid output format: {} (valid: plaintext, json)",
            config.output_format
        );
        std::process::exit(2);
    }

    // parse duration
    let duration = parse_duration::parse(config.duration.as_str()).unwrap_or_else(|err| {
        println!("error parsing duration: {}", err);
        std::process::exit(2);
    });

    // parse output size
    let output_size = ByteSize::from_str(config.output_size.as_str()).unwrap_or_else(|err| {
        println!("error parsing desired output_size parameter: {}", err);
        std::process::exit(2);
    });

    // generate a string, then a string to determine the output size for this format
    let s = generate_sequential_string(80);
    let entry_printer = get_log_entry_printer(config.output_format);
    let entry = entry_printer(&s);

    // calculate entry size
    let entry_size_bytes = entry.len();

    // calculate target duration for loop sleep to achieve required line output rate
    let target_duration =
        calculate_iteration_pause(output_size.as_u64(), duration.as_secs(), entry_size_bytes)
            .unwrap_or_else(|err| {
                println!("runtime error: {}", err);
                std::process::exit(2);
            });

    let mut lines_output: u64 = 0;
    let end_datetime = chrono::Utc::now().add(chrono::Duration::seconds(duration.as_secs() as i64));

    loop {
        println!("{}", entry_printer(&s));
        lines_output += 1;

        if (config.max_lines_output != 0 && lines_output >= config.max_lines_output)
            || (chrono::Utc::now().ge(&end_datetime))
        {
            std::process::exit(0);
        }
        // Calculate sleep time
        let elapsed = Instant::now().elapsed();
        if let Some(sleep_time) = target_duration.checked_sub(elapsed) {
            std::thread::sleep(sleep_time);
        }
    }
}
