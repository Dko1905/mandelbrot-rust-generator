use num::complex::Complex64;

fn main() {
	//test_number( Complex64::new(-1./2., 0.) , u32::MAX as f64, 15);
	graph_terminal(50, 20, 0, 0, 10., 35);
}

fn graph_terminal(width: u32, height: u32, x_offset: u32, y_offset: u32, scale: f64, step_max: usize){
	


	for num_y in 0..(height + 1) {
		let y: f64 = (num_y as f64 / scale) - ((height as f64 / 2.) / scale) + (y_offset as f64);

		for num_x in 0..(width + 1) {
			let x: f64 = (num_x as f64 / scale) - ((width as f64 / 2.) / scale) + (x_offset as f64);

			let point = Complex64::new(x, y);
			let mut z = Complex64::new(0., 0.);
			let mut steps = 0;

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
/*
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
}*/