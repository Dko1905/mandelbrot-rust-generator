use num::complex::Complex64;

fn main() {
	let mut c = Complex64::new(1., 0.); // Create complex number ( -1 + 0i )

	let mut n = 0;
	while c.re < std::u32::MAX as f64{ // Try using f(z) on the complex number with the real number as c
		println!("{} : {}", n, c);
		
		c = f(c, c.re);		

		n += 1;
	}
}

fn f(z: Complex64, c: f64) -> Complex64 {
	z.powu(2) + c
}