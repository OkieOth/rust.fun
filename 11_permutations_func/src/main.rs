
use clap::Parser;


/// Program to get the permutations for a given number of digits.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number of digits for the permutations
    #[arg(short, long)]
    digits: usize,

    /// If true, then the permutations are not printed to stdout
    #[arg(short, long, default_value_t = false)]
    skip_printing: bool,
}


fn main() {
    let args = Args::parse();
    //println!("Number of digits: {}, skip printing output: {}", args.digits, args.skip_printing);

    println!("I am going to create permutations for {} digits ...", args.digits);
    print_permutions(args.digits, args.skip_printing)
}

fn create_permutation_entry(number_count: usize) -> Vec<usize> {
    (0..number_count).collect()
}

fn create_permutations_array(number_count: usize) -> Vec<Vec<usize>> {
    let init_vec = vec![0; number_count];
    vec![init_vec.clone(); factorial(number_count)]
/*
    let permutations_count = factorial(number_count);
    let mut ret: Vec<Vec<usize>> = Vec::new();
    for _ in 0 .. permutations_count {
        let nv: Vec<usize> = vec![0; number_count];
        ret.push(nv)
    }
    ret
*/
}

fn get_rest_slice(input: &Vec<usize>, used_index: usize) -> Vec<usize> {
    input.iter()
        .enumerate()
        .filter(|elem| elem.0 != used_index )
        .map(|elem| *elem.1)
        .collect()
/*
    let mut ret: Vec<usize> = Vec::new();
	for (i,elem) in input.iter().enumerate() {
        if i != used_index {
            ret.push(*elem)
        }
	}
    ret
 */
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
        vec![input.clone(), vec![input[1], input[0]]]

/*
        let mut ret: Vec<Vec<usize>> = Vec::new();
        ret.push(input.clone());
        let mut p: Vec<usize> = Vec::new();
        p.push(input[1]);
        p.push(input[0]);
        ret.push(p);
        ret
*/
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
    elem.iter()
        .fold("".to_string(), |acc, us| acc + &us.to_string())
/*
    let mut ret = String::new();
    for e in elem.iter() {
        ret.push_str(&e.to_string())
    }
    ret
 */
}

fn print_permutions(number_count: usize, skip_printing_output: bool) {
	let input = create_permutation_entry(number_count);
    
	let permutations_vector = permutations(&input);
    println!("Number of permutations: {}\n\n", permutations_vector.len());
    if ! skip_printing_output {
        permutations_vector.iter().enumerate()
            .for_each(|i_elem| println!("{}: {}", i_elem.0, permutation_to_str(i_elem.1)));
        /*
        for (i,elem) in permutations_vector.iter().enumerate() {
            println!("{}: {}", i, permutation_to_str(&elem))
        }
        */
    }
}


fn factorial(n: usize) -> usize {
    (1 .. n+1).collect::<Vec<usize>>().iter().product::<usize>()
/*
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
 */
}

#[test]
fn test_permutations() {
	let input = create_permutation_entry(4);    
	let permutations_vector = permutations(&input);
    assert_eq!(permutations_vector.len(), 24)
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

#[test]
fn test_create_permutations_array() {
    let x1 = create_permutations_array(1);
    assert_eq!(x1.len(), 1);
    assert_eq!(x1[0].len(), 1);

    let x2 = create_permutations_array(2);
    assert_eq!(x2.len(), 2);
    assert_eq!(x2[0].len(), 2);
    assert_eq!(x2[1].len(), 2);
    let x3 = create_permutations_array(3);
    assert_eq!(x3.len(), 6);
    assert_eq!(x3[0].len(), 3);
    assert_eq!(x3[1].len(), 3);
    assert_eq!(x3[2].len(), 3);
    assert_eq!(x3[3].len(), 3);
    assert_eq!(x3[4].len(), 3);
    assert_eq!(x3[5].len(), 3);

    let x4 = create_permutations_array(4);
    assert_eq!(x4.len(), 24);
}
