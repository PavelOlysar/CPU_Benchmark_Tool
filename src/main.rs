use sysinfo::{System, SystemExt};
use indicatif::ProgressBar;
use std::env;
use std::f64;
use std::thread::available_parallelism;
use std::time::Instant;
use colored::*;

// function to calculate factorial of a number 
pub fn calculate_factorial(num: u128) -> u128 {
    (1..=num).product() // calculate factorial of a number
}

// function to loop and calculate factorial of a number as many times as specified
fn add_loop(&n_loops: &u64) {
    for _in in 0..n_loops {
        let _ = calculate_factorial(20);
    }
}

fn main() {
    // getting command line arguments
    let args: Vec<String> = env::args().collect();
    let num_calcs_arg: Option<&String> = args.get(1);
    // number of calculations to perform in each iteration (default 1000000000 if not provided)
    let num_calcs: u64 = match num_calcs_arg {
        Some(num_calcs_arg) => num_calcs_arg.trim().parse::<u64>().unwrap(), 
        None => 1000000000, // default 
    };
    let num_iters: u64 = 50000; // number of iterations to perform (fixed at 50000)
    let total_calc: u64 = num_calcs * num_iters; // number of calculations across all iterations
    println!(
        "{}",
        format!(
            "Running {} calculations over {} iterations each with a total of {} calculations.",
            num_calcs, num_iters, total_calc,
        ).magenta()
    );

    // getting info about system using sysinfo crate
    let mut sys = System::new_all(); // mutable variable (declares it can be changed after assigning)
    sys.refresh_all();

    // displaying the system information
    println!(
        "{}: {}",
        "System name".bold(),
        format!("{}", sys.name().unwrap_or_else(|| "Unknown".to_string())).green()
    );
    println!(
        "{}: {}",
        "System kernel version".bold(),
        format!("{}", sys.kernel_version().unwrap_or_else(|| "Unknown".to_string())).green()
    );
    println!(
        "{}: {}",
        "System OS version".bold(),
        format!("{}", sys.os_version().unwrap_or_else(|| "Unknown".to_string())).green()
    );
    println!(
        "{}: {}",
        "System host name".bold(),
        format!("{}", sys.host_name().unwrap_or_else(|| "Unknown".to_string())).green()
    );
    
    println!(
        "{}: {}",
        "System of available threads".bold(),
        sys.cpus().len().to_string().green()
    );


    let available_cores: u64 = available_parallelism().unwrap().get() as u64; // getting info about how many threads we can use
    let iter_per_core: u64 = num_calcs / available_cores; // number of calculations to perform in each thread

    let now = Instant::now(); // starting the timer

    let bar = ProgressBar::new(num_iters); // progress bar to show the progress of the calculations
    for _i in 0..num_iters {
        let mut threads = Vec::new(); // vector to store the threads
        for _i in 0..available_cores {
            threads.push(std::thread::spawn(move || add_loop(&iter_per_core))); // creating threads to perform the calculations
        }
        bar.inc(1); // update the progress bar
    }

    bar.finish(); // finishing the progress bar
    let elapsed = now.elapsed(); // stopping the timer
    let calc_per_sec: f64 = (total_calc as f64) / (elapsed.as_secs() as f64); // calculating the number of calculations per second
    println!(
        "{}",
        format!("Total runtime: {:.2?}", elapsed).bold().yellow()
    );
    println!(
        "{}",
        format!("Calculations per second: {:.2?} seconds.", calc_per_sec).bold().yellow()
    );
}

