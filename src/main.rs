use std::{thread, time};
use structopt::StructOpt;
use indicatif::{ProgressBar, ProgressStyle};

// start a timer that will ding when complete
#[derive(StructOpt)]
struct Cli {
    // the number
    duration: String,
    // the units of time (seconds or minutes)
    time_unit: String,
}

/// Returns the string as a float
/// ```
/// assert_eq!(read_num_from_string("12.2"), 12.2);
/// ```
fn read_num_from_string(input: String) -> f32 {
    let num: f32 = input.parse().unwrap();
    return num;
}

/// Returns whether or not the input wants minutes
/// ```
/// assert_eq!(is_minutes("years"), false);
/// ```
fn is_minutes(input: &String) -> bool {
    if input == "mins" || input == "min" {
        return true
    }
    return false;
}

/// Returns whether or not the input wants seconds
/// ```
/// assert_eq!(is_seconds("decades"), false) 
/// ```
fn is_seconds(input: &String) -> bool {
    if input == "secs" || input == "sec" {
        return true;
    }
    return false
}

fn progress_bar(seconds: u64) {
    let pb = ProgressBar::new(seconds);
     pb.set_style(ProgressStyle::default_bar()
                 .template("{spinner:.green} [{elapsed_precise}] [{bar:80.cyan/blue}] ({eta})")
                         .progress_chars("#>-"));

    let one_second = time::Duration::from_secs(1);

    for _i in 0..seconds {
        thread::sleep(one_second);
        pb.inc(1);
    }
    pb.finish_with_message("done");
}


fn main() {
    let args = Cli::from_args();

    let num = read_num_from_string(args.duration);
    let mut timer: f32 = 0.0;

    // read secs or mins
    if is_minutes(&args.time_unit) {
        timer = num * 60.0;
    } else if is_seconds(&args.time_unit){
        timer = num;
    } else if !is_minutes(&args.time_unit) && !is_minutes(&args.time_unit) {
        eprintln!("Invalid time unit specified. Must be `mins` or `secs`");
        return;
    }

    let timer = timer as u64;
    progress_bar(timer);
    print!("\x07");
}
