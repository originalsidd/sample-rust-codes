fn main() {
	// for loop
	for i in 1..5 + 1 {
		println!("{}", i);
	}

	// for loop for an iterator
	let names = ["Sidd", "Jackie", "Bruh"];
	for name in names.iter() {
		println!("{}", name);
	}

	// while
	let mut x = 1;
	while x <= 5 {
		println!("{}", x);
		x += 1;
	}

	// loop
	let mut c: u8 = 0;
	let x: u8 = 5;
	loop {
		println!("This loop will run for {} time", x - c - 1);
		c += 1;
		if c == x {
			break;
		}
	}
}