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
    if env::args().len() != 2 {
        let args: Vec<String> = env::args().collect();
        print_usage_and_exit(&args[0]);
    }
    let args: Vec<String> = env::args().collect();
    let digit_str = &args[1];


    // Attempt to parse the string as an integer
    match digit_str.parse::<usize>() {
        Ok(digit_int) => {
            println!("I am going to create permutations for {} digits ...", digit_int);
            print_permutions(digit_int)
        },
        Err(_) => {
            println!("Failed to parse the string as an unsigned integer");
            process::exit(1);
        }
    }
}

fn create_permutation_entry(number_count: usize) -> Vec<usize> {
    let mut ret: Vec<usize> = Vec::new();
    for i in 0..number_count {
        ret.push(i);
    }
    ret
}

fn create_permutations_array(number_count: usize) -> Vec<Vec<usize>> {
    let permutations_count = factorial(number_count);
    let mut ret: Vec<Vec<usize>> = Vec::new();
    for _ in 0 .. permutations_count {
        let nv: Vec<usize> = vec![0; number_count];
        ret.push(nv)
    }
    ret
}

fn get_rest_slice(input: &Vec<usize>, used_index: usize) -> Vec<usize> {
    let mut ret: Vec<usize> = Vec::new();
	for (i,elem) in input.iter().enumerate() {
        if i != used_index {
            ret.push(*elem)
        }
	}
    ret
}

// takes an vector with uniqe digits and returns another vector that
// contains all permutations of the input
fn permutations(input: &Vec<usize>) -> Vec<Vec<usize>> {
	let input_len = input.len();
    // the algorithm ...
	if input_len == 2 {
        // basically we only know for sure, what the permutations for two digits are ...
        // ... so we create a return vector that contains two entries, with the permutations
        // of input[0] and input[1]
        let mut ret: Vec<Vec<usize>> = Vec::new();
        ret.push(input.clone());
        let mut p: Vec<usize> = Vec::new();
        p.push(input[1]);
        p.push(input[0]);
        ret.push(p);
        ret
	} else {
        // if the number of digits larger than two, we are unsure and lazy. What we know
        // for sure is that n digits have n! permutations ...

        // we create a vector of size n!
        let mut ret = create_permutations_array(input_len);
        let input_len_prev: usize = input_len - 1;

        // ... how many permutations has every unique digit, with itself in the first place
		let perm_count_0 = factorial(input_len_prev);
		for i in 0 .. input_len { // we loop over all elems of the input ...
            let rest_slice = get_rest_slice(&input, i);         // get a slice w/o the current digit
            let rest_permutations = permutations(&rest_slice);  // .. and get the permutations for the
                                                                                        // rest slice
			for j in 0 .. perm_count_0 { // we loop over all permutations for one digit ...
                let index = (i * perm_count_0) + j;
                ret[index as usize][0] = input[i];  // ... and fill in for all possible permutations
                                                    // the current digits in the result vector

                for k in 1 .. input_len {    // here we start to copy the result permutations for the rest slice
                    ret[index as usize][k] = rest_permutations[j as usize][k-1]
                }
            }
        }
        ret
    }
}

fn permutation_to_str(elem: &Vec<usize>) -> String {
    let mut ret = String::new();
    for e in elem.iter() {
        ret.push_str(&e.to_string())
    }
    ret
}

fn print_permutions(number_count: usize) {
	let input = create_permutation_entry(number_count);
    
	let permutations_vector = permutations(&input);
    println!("Number of permutations: {}\n\n", permutations_vector.len());
	// for (i,elem) in permutations_vector.iter().enumerate() {
	// 	println!("{}: {}", i, permutation_to_str(&elem))
	// }
}


fn factorial(n: usize) -> usize {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}

#[test]
fn test_permutations() {
	let input = create_permutation_entry(4);    
	let permutations_vector = permutations(&input);
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




