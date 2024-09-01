use std::io;
use std::thread::sleep;
use std::time::{Duration, SystemTime};
use chrono::{DateTime, Local, NaiveDateTime, Timelike};

fn main() {
    println!("Enter the date and time in the format YYYY-MM-DD HH:MM (e.g., 2024-12-31 23:59): ");
    
    let mut input_str = String::new();
    io::stdin().read_line(&mut input_str).expect("Failed to read input");
    let input_str = input_str.trim();

    let end_time = match NaiveDateTime::parse_from_str(input_str, "%Y-%m-%d %H:%M") {
        Ok(dt) => DateTime::<Local>::from_utc(dt, *Local::now().offset()),
        Err(_) => {
            println!("Error parsing date and time. Please ensure the format is correct.");
            return;
        }
    };

    println!("End time: {}", end_time);

    loop {
        let now = Local::now();

        if now >= end_time {
            println!("Time is up!");
            break;
        }

        let remaining = end_time - now;
        let remaining_seconds = remaining.num_seconds();

        let hours = remaining_seconds / 3600;
        let minutes = (remaining_seconds % 3600) / 60;
        let seconds = remaining_seconds % 60;

        print!("\rRemaining: {} hr {} min {} sec", hours, minutes, seconds);
        io::Write::flush(&mut io::stdout()).expect("Output error");
        sleep(Duration::from_secs(1));
    }

    println!("\nPress Enter to exit...");
    let mut exit_input = String::new();
    io::stdin().read_line(&mut exit_input).expect("Failed to read input");
}
