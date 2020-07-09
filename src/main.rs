// For reading and opening files

use num::complex::Complex64;

fn main() {
	let i = Complex64::new(0., 1.);

	println!("i = {}", i);

	println!("i * 2 = {}", i * 2.);

	println!("i * i = {}", i * i);
}