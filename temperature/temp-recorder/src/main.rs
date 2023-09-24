use std::error::Error;

use clap::{Arg, Command};
use std::io::{self};
use std::thread::sleep;
use std::time::{Duration, SystemTime};

fn main() -> Result<(), Box<dyn Error>> {
    let matches = Command::new("SerialPort Recorder")
        .arg(
            Arg::new("port")
                .help("The serial port to listen to.")
                .use_value_delimiter(false)
                .required(true),
        )
        .arg(
            Arg::new("baud")
                .help("The baud rate to listen at.")
                .use_value_delimiter(false)
                .required(true)
                .value_parser(clap::value_parser!(u32)),
        )
        .get_matches();

    // Create a CSV file to record data
    let mut wtr = csv::Writer::from_path("temperature_data.csv")?;

    // Print headers to the CSV file
    wtr.write_record(["Timestamp (ms)", "Temperature (°F)"])?;

    // Record data for 5 minutes
    let start_time = SystemTime::now();
    let duration = Duration::from_secs(600); // 10 minutes
    let mut buffer: Vec<u8> = Vec::new();
    let mut read_buffer: [u8; 128] = [0; 128];

    let port_name = matches
        .get_one::<String>("port")
        .expect("Port is required.");
    let baud_rate = *matches.get_one::<u32>("baud").expect("Baud is required.");
    let timeout = Duration::from_secs(5);

    // Open the serial port
    let port = serialport::new(port_name, baud_rate)
        .timeout(timeout)
        .open();

    match port {
        Ok(mut port) => {
            while start_time.elapsed().unwrap() < duration {
                match port.read(read_buffer.as_mut_slice()) {
                    Ok(bytes_read) => {
                        buffer.extend_from_slice(&read_buffer[..bytes_read]);
                        if let Some(pos) = buffer.iter().position(|&c| c == b'\n') {
                            let buffer_clone = buffer.clone();
                            let split = buffer_clone.split_at(pos + 1);

                            let line = split.0;
                            buffer = split.1.to_vec();

                            if let Ok(line_str) = String::from_utf8(line.to_vec()) {
                                if let Some((timestamp_str, temperature_str)) =
                                    parse_data(&line_str)
                                {
                                    if let Ok(timestamp) = timestamp_str.parse::<u32>() {
                                        if let Ok(temperature) =
                                            temperature_str.trim().parse::<f32>()
                                        {
                                            // Write to standard output
                                            println!(
                                                "Timestamp: {} ms, Temperature: {:.2}°F",
                                                timestamp, temperature
                                            );

                                            // Write to the CSV file
                                            wtr.write_record(&[
                                                timestamp.to_string(),
                                                temperature.to_string(),
                                            ])?;
                                            wtr.flush()?;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                    Err(e) => eprintln!("Failed to read: {}", e),
                }

                sleep(Duration::from_millis(100));
            }
        }
        Err(e) => {
            eprintln!("Failed to open port {}. Error: {}", port_name, e);
            std::process::exit(-1);
        }
    };

    Ok(())
}

fn parse_data(line: &str) -> Option<(String, String)> {
    let parts: Vec<&str> = line.trim().split(',').collect();
    if parts.len() == 2 {
        let timestamp = parts[0].to_string();
        let temperature = parts[1].to_string();
        Some((timestamp, temperature))
    } else {
        None
    }
}
