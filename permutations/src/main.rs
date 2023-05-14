use std::env;
use std::process;


fn print_usage_and_exit(program_name: &str) {
    println!("Usage: {} {{NUMBER_OF_DIGITS}}", program_name);
    println!();
    println!("The program prints the permutations for a given number of digits.");
    println!("e.g. `{} 4` prints all possible different combinations of 1,2,3,4.", program_name);
    process::exit(1);
}

fn main() {
    if env::args().len() == 1 {
        let args: Vec<String> = env::args().collect();
        print_usage_and_exit(&args[0]);
    }
    for arg in env::args() {
        println!("  command line parameter: {}", arg);
    }
}

fn factorial(n: u64) -> u64 {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}

#[test]
fn test_factorial() {
    assert_eq!(factorial(1), 1);
    assert_eq!(factorial(2), 2);
    assert_eq!(factorial(3), 6);
    assert_eq!(factorial(4), 24);
    assert_eq!(factorial(5), 120);
    assert_eq!(factorial(6), 720);
    assert_ne!(factorial(6), 721);
}

fn permutations(input: Vec<u64>) -> Vec<Vec<u64>> {
    // TODO
    return Vec::new();
}




