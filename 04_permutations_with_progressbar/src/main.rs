
use clap::Parser;

use indicatif::{ProgressBar, ProgressStyle};


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

trait ProgressHandler {
    fn progress(&mut self, step: usize);
}


fn main() {
    let args = Args::parse();
    //println!("Number of digits: {}, skip printing output: {}", args.digits, args.skip_printing);

    //println!("I am going to create permutations for {} digits ...", args.digits);
    print_permutions(args.digits, args.skip_printing)
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
fn permutations<'a>(input: &Vec<usize>, progress_handler: &mut Option<&'a mut impl ProgressHandler>) -> Vec<Vec<usize>> {
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
            let rest_permutations = permutations2(&rest_slice);  // .. and get the permutations for the
                                                                                        // rest slice
			for j in 0 .. perm_count_0 { // we loop over all permutations for one digit ...
                let index = (i * perm_count_0) + j;
                ret[index as usize][0] = input[i];  // ... and fill in for all possible permutations
                                                    // the current digits in the result vector

                for k in 1 .. input_len {    // here we start to copy the result permutations for the rest slice
                    ret[index as usize][k] = rest_permutations[j as usize][k-1]
                }
            }
            if progress_handler.is_some() {
                progress_handler.as_mut().unwrap().progress(i);
            }
        }
        ret
    }
}

fn permutations2(input: &Vec<usize>) -> Vec<Vec<usize>> {
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
            let rest_permutations = permutations2(&rest_slice);  // .. and get the permutations for the
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

struct ProgressBarProgressHandler<'a> {
    pb: Option<&'a ProgressBar>
}

impl <'a> ProgressBarProgressHandler<'a> {
    fn new(progress_bar: &'a ProgressBar) -> ProgressBarProgressHandler {
        ProgressBarProgressHandler {
            pb: Some(progress_bar)
        }
    }
}

impl <'a>ProgressHandler for ProgressBarProgressHandler<'a> {
    fn progress(&mut self, _step: usize) {
        if self.pb.is_some()  {
            self.pb.as_mut().unwrap().inc(1);
        }
    }
}

fn print_permutions(number_count: usize, skip_printing_output: bool) {
	let input = create_permutation_entry(number_count);

    let mut progress_bar = ProgressBar::new(number_count as u64);


    let sty = ProgressStyle::with_template(
        "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
    )
    .unwrap()
    .progress_chars("##-");
    let mut progress_handler = ProgressBarProgressHandler::new(& progress_bar);
    progress_bar.set_style(sty.clone());


    let mut ph_option = Some(&mut progress_handler);

	let permutations_vector = permutations(&input, &mut ph_option);
    progress_bar.finish();
    println!("Number of permutations: {}\n\n", permutations_vector.len());
    if ! skip_printing_output {
        for (i,elem) in permutations_vector.iter().enumerate() {
            println!("{}: {}", i, permutation_to_str(&elem))
        }
    }
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
	let permutations_vector = permutations2(&input);
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




