fn main() {
	let cond = 2 < 3;
	println!("{}", cond);
	
	// error
	// let cond = 2 < 3.0;
	
	// fix
	let cond = (2 as f32) < 3.0;
	println!("{}", cond);

	let cond = 2f32 < 3.0;
	println!("{}", cond);
	
	// logical operators
	let cond2 = false && cond;
	let cond3 = false || cond;
	let cond4 = !cond;
	println!("{}, {}, {}, {}", cond, cond2, cond3, cond4);

	// control statements
	let food = "cookie";

	if food == "cookie" {
		println!("I like cookies!");
	} else {
		println!("Bruh");
	}
}