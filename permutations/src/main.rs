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
    match digit_str.parse::<u32>() {
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

fn create_permutation_entry(number_count: u32) -> Vec<u32> {
    let mut ret: Vec<u32> = Vec::new();
    for i in 0..number_count {
        ret.push(i);
    }
    ret
}

fn permutations(input: Vec<u32>) -> Vec<Vec<u32>> {
	let inputLen = input.len();
	if inputLen == 2 {
        let mut ret: Vec<Vec<u32>> = Vec::new();
        ret.push(input.clone());
        let mut p: Vec<u32> = Vec::new();
        p.push(input[1]);
        p.push(input[0]);
        ret.push(p);
        ret
	} else {
        let ret: Vec<Vec<u32>> = Vec::new();
        ret
    }
    
/*    
     else {
		ret := createPermutationsArray(inputLen)
		permCount_0 := fakultaet(inputLen - 1)
		for i := 0; i < inputLen; i++ {
			restSlice := getRestSlice(input, i)
			restPermutations := permutations(restSlice)
			for j := 0; j < permCount_0; j++ {
				index := (i * permCount_0) + j
				ret[index][0] = input[i]
				for k := 1; k < inputLen; k++ {
					ret[index][k] = restPermutations[j][k-1]
				}
			}
		}
		return ret
	}
*/
}

fn permutation_to_str(elem: &Vec<u32>) -> String {
    let mut ret = String::new();
    for e in elem.iter() {
        ret.push_str(&e.to_string())
    }
    ret
}

fn print_permutions(number_count: u32) {
	let input = create_permutation_entry(number_count);
    
	let permutations_vector = permutations(input);
    println!("Number of permutations: {}\n\n", permutations_vector.len());
	for (i,elem) in permutations_vector.iter().enumerate() {
		println!("{}: {}", i, permutation_to_str(&elem))
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




