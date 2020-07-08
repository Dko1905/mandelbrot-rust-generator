// For reading and opening files
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;

use num::Complex;

fn main() {
	let size: u32 = 1200;
	let max = 10;
	let mut scale: f64 = 100.;

	let mut thread_handles: Vec<std::thread::JoinHandle<()>> = Vec::new(); 

	for n in 0..max{
		let handle = std::thread::spawn(move || {
			generator(&format!("image{}.png", n), size, size, scale).expect("Failed to generate images");
		});
		thread_handles.push(handle);
		
		scale = scale * 2.;
		println!("n = {}", n);
	}

	let mut n = 0;
	for handle in thread_handles {
		handle.join().expect("Failed to join handle");
		println!("{}", n);
		n += 1;
	}
}

fn generator(output_path: &str, width: u32, height: u32, scale: f64) -> Result<(), std::boxed::Box<dyn std::error::Error>> {
	let file: File = File::create(output_path)?;
	let mut png_encoder = png::Encoder::new(BufWriter::new(file), width, height);
	png_encoder.set_color(png::ColorType::RGB);
	png_encoder.set_depth(png::BitDepth::Eight);

	let mut png_writer = png_encoder.write_header()?;
	let mut swriter = png_writer.stream_writer();

	let seed: f64 = 1.;
	let precision = 100.;
	let x_offset = 2./3. - (486325./10000000.);
	let y_offset = 0;

	for num_y in 0..(height) {
		let y: f64 = num_y as f64 / scale -  ( height as f64 / 2. ) / scale + y_offset as f64;
		
		for num_x in 0..(width) {
			let mut count: u64 = 0;
			let mut check: u64 = 0;
			
			let x: f64 = num_x as f64 / scale -  ( width as f64 / 2. ) / scale + x_offset as f64;

			let mut z = Complex::new(x, y).powf(2.) - seed;
			
			
			while count < precision as u64 {
				z = z.powf(2.) - seed;

				if z.norm() > 1000000. {
					check = count;
					count = precision as u64;
				}

				count += 1;
			}

			let mut color: [u8; 3] = [0; 3];
			if z.norm() <= 0. {
				color[0] = 0;
				color[1] = 0;
				color[2] = 0;
			}
			else {
				let mut hsl_color = colorsys::Hsl::default();
				hsl_color.set_hue((check * 10 % 255) as f64);
				hsl_color.set_saturation(100.);
				hsl_color.set_lightness(50.);
				let a = colorsys::Rgb::from( hsl_color );
				color[0] = a.get_red() as u8;
				color[1] = a.get_green() as u8;
				color[2] = a.get_blue() as u8;
			}
			
			swriter.write(&color).expect("Failed to write");
		}
	}

	swriter.flush().expect("Failed to flush");
	swriter.finish().expect("Failed to finish");
	
	Ok(())
}