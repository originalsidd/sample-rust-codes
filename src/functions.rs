fn main() {
	println!("Hello, world!");
	test();
	add_numbers(3, 4);

	// statement vs expression
	// statement are mostly assignment vs expressions evaluate and return something

// dont put semicolon after the expression which is going to return something
	let num = {
		let x = 3;
		x + 1
	};
	println!("num: {}", num);
	
	let res = add_numbers_2(4, 6);
	println!("x + y: {}", res);
	
	let y = return_20();
	println!("return_20: {}", y);

	println!("Hundred: {}", can_use_return_keyword_with_semicolon());

}

fn test() {
	println!("test() called");
}

fn add_numbers(x: i32, y: i32) {
	println!("The sum is: {}", x + y);
}

// delcare return type after ->
fn add_numbers_2(x: i32, y: i32) -> i32 {
	x + y
}

fn return_20() -> String {
	let x = String::from("20 number");
	x
}

// when explicitly using return statement
// we can either have or remove the semicolon at the end of the returning expression
fn can_use_return_keyword_with_semicolon() -> i32 {
	return 100;
}

// expected default return value is ()