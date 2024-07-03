//use inditicatif::ProgressBar;

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
    
}
