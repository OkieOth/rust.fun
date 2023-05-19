Example to generate permutations for a given number of digits.

I compared the runtime with Golang and Python implementations of the same
algorithm and for 10 digits I received the following results:


| Test                                        | Rust   | Golang | Python  |
| ------------------------------------------- | ------ | ------ | ------- |
| Permutations 10 digit with output to stdout | 17"    | 21"    | 60"     |
| Permutations 10 digit with `cargo run`      | 51"    |        |         |
| Permutations 11 digit with output to stdout | 3' 20" | 4' 14" | 12' 57" |
| Permutations 11 digit without output        | 23"    | 29"    | 8' 18"  |


Either the Rust and the Golang programs were build as releases.