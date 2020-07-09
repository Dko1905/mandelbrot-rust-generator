use num::complex::Complex64;

fn main() {
	test_number( Complex64::new(-1./2., 0.) , u32::MAX as f64, 15);
	//graf_terminal(-2., 2., 0.1, 15, u32::MAX as f64);
}

fn graf_terminal(start: f64, stop: f64, step: f64, raise_max: usize, raise_stop: f64){
	let mut y = stop;
	let mut x = start;

	let mut z: Complex64;
	let mut steps: usize;

	while y > start {
		while x < stop {
			z = Complex64::new(x, y);

			steps = 0;
			while z.re < raise_stop && steps < raise_max {
				z = f(z, z.re);

				steps += 1;
			}

			if steps < raise_max {
				print!("#");
			}
			else{
				print!(" ");
			}

			x += step;
		}
		println!("");
		x = start;
		y -= step;
	}
}

fn test_number(z_input: Complex64, raise_stop: f64, raise_max: usize){
	let mut steps = 0;
	let mut z = z_input;
	while z.re < raise_stop && steps < raise_max {
		z = f(z, z_input.re);

		steps += 1;
	}

	if steps < raise_max {
		println!("{} has blown up", z_input);
	}
	else{
		println!("{} has not blown up", z_input);
	}
}

#[inline]
fn f(z: Complex64, c: f64) -> Complex64 {
	z.powu(2) + c
}