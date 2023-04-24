use std::io;
// use crate::module
// using standard library crate and importing input/output module
// :: is the path separator operator

fn main() {
	println!("Hello, World!");
	let mut input = String::new();
	// here string is the module and new() is kind of a constructor
	// which is goin to create a new empty string object

	// calling stdin() i.e. the input at standard output console from io module
	// calling read_line function and creating a mutable reference to input variable
	// till read_line returns a result object on which we call expect for handling errors
	io::stdin().read_line(&mut input).expect("failed to read line");
	println!("{}", input);

}