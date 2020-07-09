use num_complex::Complex64;

fn main() {
	//test_number(Complex64::new(-1., 0.), 15);
	graph_terminal(50, 20, 0, 0, 10., 35);
}

#[allow(dead_code)]
fn graph_terminal(width: u32, height: u32, x_offset: u32, y_offset: u32, scale: f64, step_max: usize){
	// Predefine all vars for faster stack
	let mut y: f64;
	let mut x: f64;
	let mut point: Complex64;
	let mut z: Complex64;
	let mut steps: usize;

	for num_y in 0..(height + 1) {
		y = (num_y as f64 / scale) - ((height as f64 / 2.) / scale) + (y_offset as f64);

		for num_x in 0..(width + 1) {
			x = (num_x as f64 / scale) - ((width as f64 / 2.) / scale) + (x_offset as f64);

			point = Complex64::new(x, y);
			z = Complex64::new(0., 0.);
			steps = 0;

			while z.norm() < 2. && steps < step_max {
				z = z * z + point;

				steps += 1;
			}

			if steps < step_max {
				print!("#");
			}
			else {
				print!(" ");
			}
		}
		println!("");
	}
}

#[allow(dead_code)]
fn test_number(z_input: Complex64, step_max: usize){
	let mut steps = 0;
	let mut z = z_input;
	while z.norm() < 2. && steps < step_max {
		z = z * z + z_input;

		steps += 1;
	}

	if steps < step_max {
		println!("{} has blown up", z_input);
	}
	else{
		println!("{} has not blown up", z_input);
	}
}