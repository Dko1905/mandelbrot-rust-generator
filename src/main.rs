// For reading and opening files
use std::path::Path;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;

use num::Complex;

fn main() {
	let path = Path::new(r"./image.png");
	
	generator(path, 400, 400).expect("Failed to generate png file");
}

fn generator(output_path: &Path, width: u32, height: u32) -> Result<(), std::boxed::Box<dyn std::error::Error>> {
	let file: File = File::create(output_path)?;
	let mut png_encoder = png::Encoder::new(BufWriter::new(file), width, height);
	png_encoder.set_color(png::ColorType::Grayscale);
	png_encoder.set_depth(png::BitDepth::Eight);

	let mut png_writer = png_encoder.write_header()?;
	let mut swriter = png_writer.stream_writer();

	let seed = 1.;
	let precision = 100.;
	let scale = 200.;
	let x_offset = 0;
	let y_offset = 0;

	for num_y in 0..(height) {
		let y: f64 = num_y as f64 / scale -  ( height as f64 / 2. ) / scale + y_offset as f64;
		
		for num_x in 0..(width) {
			let mut count: u64 = 0;
			let mut check: u64 = 0;
			
			let x: f64 = num_x as f64 / scale -  ( width as f64 / 2. ) / scale + x_offset as f64;

			let mut z = Complex::new(x, y).powf(2.) - seed;
			
			let color: u8;
			while count < precision as u64 {
				z = z.powf(2.) - seed;

				if z.norm() > 1000000. {
					check = count;
					count = precision as u64;
				}

				count += 1;
			}

			if z.norm() <= 0. {
				color = 0;
			}
			else {
				color = (255 - check % 255) as u8;
			}
			swriter.write(&[color]).expect("Failed to write");
		}
	}

	swriter.flush().expect("Failed to flush");
	swriter.finish().expect("Failed to finish");
	
	Ok(())
}