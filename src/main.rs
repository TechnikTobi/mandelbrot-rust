mod mandelbrot;
mod generator;
mod png_writer;

use generator::generate_raw_data;
use png_writer::write_png;

fn main() {

	let test = generate_raw_data(
		80,
		40,
		0.0,
		0.0,
		0.4,
		100
	);

	for y in 0..40
	{
		for x in 0..80
		{
			if test[80*y + x] < 50
			{
				print!(" ");
			}
			else
			{
				print!("#");
			}
		}
		println!("");
	}

	write_png(10, 10, "test.png", Vec::new());

}
