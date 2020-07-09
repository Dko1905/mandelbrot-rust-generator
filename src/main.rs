use num_complex::Complex64;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;


fn main() {
	//test_number(Complex64::new(-1., 0.), 15);
	let mut scale = 400.;
	for n in 0..10 {
		graph_png(&format!("image{}.png", n), 1920, 1080, -1. + 1./4., 0., scale, (scale/10.) as usize).expect("Failed to graph");
		scale *= 2.;
		println!("{}", n);
	}
}

#[allow(dead_code)]
fn graph_terminal(width: u32, height: u32, x_offset: f64, y_offset: f64, scale: f64, step_max: usize){
	// Predefine all vars for faster stack
	let mut y: f64;
	let mut x: f64;
	let mut point: Complex64;
	let mut z: Complex64;
	let mut steps: usize;

	for num_y in 0..height {
		y = (num_y as f64 / scale) - ((height as f64 / 2.) / scale) + (y_offset as f64);

		for num_x in 0..width {
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
fn graph_png(filename: &str, width: u32, height: u32, x_offset: f64, y_offset: f64, scale: f64, step_max: usize) -> Result<(), std::boxed::Box<dyn std::error::Error>>{
	let file: File = File::create(filename)?;
	let mut png_encoder = png::Encoder::new(BufWriter::new(file), width, height);
	png_encoder.set_color(png::ColorType::RGB);
	png_encoder.set_depth(png::BitDepth::Eight);

	let mut png_writer = png_encoder.write_header()?;
	let mut swriter = png_writer.stream_writer();

	// Predefine all vars for faster stack
	let mut y: f64;
	let mut x: f64;
	let mut point: Complex64;
	let mut z: Complex64;
	let mut steps: usize;
	let mut rgb: colorsys::Rgb;
	let mut line: Vec<u8> = vec!(0; (3 * width) as usize);

	for num_y in 0..height {
		y = (num_y as f64 / scale) - ((height as f64 / 2.) / scale) + (y_offset as f64);

		for num_x in 0..width {
			x = (num_x as f64 / scale) - ((width as f64 / 2.) / scale) + (x_offset as f64);

			point = Complex64::new(x, y);
			z = Complex64::new(0., 0.);
			steps = 0;

			while z.norm() < 2. && steps < step_max {
				z = z * z + point;

				steps += 1;
			}

			if steps < step_max {
				rgb = colorsys::Rgb::from(colorsys::Hsl::new((10.*(steps as f64))%255., 100., 50., std::option::Option::None));
				line[(num_x * 3) as usize] = rgb.get_red() as u8;
				line[(num_x * 3 + 1) as usize] = rgb.get_green() as u8;
				line[(num_x * 3 + 2) as usize] = rgb.get_blue() as u8;
			}
			else {
				line[(num_x * 3) as usize] = 0;
				line[(num_x * 3 + 1) as usize] = 0;
				line[(num_x * 3 + 2) as usize] = 0;
			}
		}
		swriter.write(&line)?;
	}

	swriter.flush()?;
	swriter.finish()?;

	Ok(())
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
