use std::error::Error;

use clap::{Arg, Command};
use std::io::{self};
use std::thread::sleep;
use std::time::Duration;

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
    let mut wtr = csv::Writer::from_path("rpm_data.csv")?;

    // Print headers to the CSV file
    wtr.write_record(["RPM"])?;

    let mut buffer: Vec<u8> = Vec::new();
    let mut read_buffer: [u8; 128] = [0; 128];

    let port_name = matches
        .get_one::<String>("port")
        .expect("Port is required.");
    let baud_rate = *matches.get_one::<u32>("baud").expect("Baud is required.");
    let timeout = Duration::from_secs(1);

    // Open the serial port
    let port = serialport::new(port_name, baud_rate)
        .timeout(timeout)
        .open();

    match port {
        Ok(mut port) => {
            loop {
                match port.read(read_buffer.as_mut_slice()) {
                    Ok(bytes_read) => {
                        buffer.extend_from_slice(&read_buffer[..bytes_read]);
                        if let Some(pos) = buffer.iter().position(|&c| c == b'\n') {
                            let buffer_clone = buffer.clone();
                            let split = buffer_clone.split_at(pos + 1);

                            let line = split.0;
                            buffer = split.1.to_vec();

                            if let Ok(line_str) = String::from_utf8(line.to_vec()) {
                                let rpm_str = line_str.trim();
                                    if let Ok(rpm) = rpm_str.parse::<u32>() {

                                            // Write to standard output
                                            println!(
                                                "RPM: {}",
                                                rpm
                                            );

                                            // Write to the CSV file
                                            wtr.write_record(&[
                                                rpm.to_string()
                                            ])?;
                                            wtr.flush()?;
                                    }
                                    else {
                                        println!("Failed parse!");
                                    }
                            } else {
                                println!("Failed from_utf8!");
                            }
                        }
                    }
                    Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                    Err(e) => eprintln!("Failed to read: {}", e),
                }

                //sleep(Duration::from_millis(100));
            }
        }
        Err(e) => {
            eprintln!("Failed to open port {}. Error: {}", port_name, e);
            std::process::exit(-1);
        }
    };
}
