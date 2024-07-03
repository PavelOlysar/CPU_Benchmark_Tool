use sysinfo::{System, SystemExt};
use indicatif::ProgressBar;
use std::env;
use std::f64;
use std::thread::available_parallelism;
use std::time::Instant;

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
    let args: Vec<String> = env::args().collect();
    let num_calcs_arg: Option<&String> = args.get(1);
    let num_calcs: u64 = match num_calcs_arg {
        Some(num_calcs_arg) => num_calcs_arg.trim().parse::<u64>().unwrap(),
        None => 400000000, // runs 100 times
    };
    let num_iters: u64 = 20000;
    let total_calc: u64 = num_calcs * num_iters;
    println!(
        "Running {} calculations over {} iterations each with a total of {} calculations.",
        &num_calcs, &num_iters, &total_calc,
    );

    // getting info about system using sysinfo crate
    let mut sys = System::new_all(); // mutable variable (declares it can be changed after assigning)
    sys.refresh_all();

    // displaying the system information
    println!("System name:               {}", sys.name().unwrap_or_else(|| "Unknown".to_string()));
    println!("System kernel version:     {}", sys.kernel_version().unwrap_or_else(|| "Unknown".to_string()));
    println!("System  OS version:        {}", sys.os_version().unwrap_or_else(|| "Unknown".to_string()));
    println!("System host name:          {}", sys.host_name().unwrap_or_else(|| "Unknown".to_string()));

    // number of CPUs!!!
    println!("System of avaiable threads:{}", sys.cpus().len());


    let available_cores: u64 = available_parallelism().unwrap().get() as u64; // getting info about how many threads we can use and use half
    let iter_per_core: u64 = num_calcs / available_cores;

    let now = Instant::now();

    let bar = ProgressBar::new(num_iters);
    for _i in 0..num_iters {
        let mut results = Vec::new();
        let mut threads = Vec::new();
        for _i in 0..available_cores {
            threads.push(std::thread::spawn(move || add_loop(&iter_per_core)));
        }
        for thread in threads {
            results.extend(thread.join());
        }
        bar.inc(1);
    }

    bar.finish();
    let elapsed = now.elapsed();
    let calc_per_sec: f64 = (total_calc as f64) / (elapsed.as_secs() as f64);
    println!("Total runtime: {:.2?}", elapsed);
    println!("Calculations per second: {:.2?} seconds.", calc_per_sec);
}

