mod mandelbrot;
mod generator;
mod png_writer;
mod color_map;
mod cli;

use generator::generate_raw_data;
use png_writer::write_png;
use color_map::{EColorMode, map_raw_to_rgb};
use cli::CliArgs;

fn main() {

	let args = CliArgs::get();
	args.print();

	let raw_data = generate_raw_data(
		args.width,
		args.height,
		args.x_mid,
		args.y_mid,
		args.zoom,
		args.iterations
	);

	let rgb_data = map_raw_to_rgb(
		&raw_data, 
		args.iterations,
		EColorMode::DEFAULT // BLUE
	);

	/*
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
	*/

	write_png(
		args.width, 
		args.height, 
		args.output_file_name, 
		&rgb_data
	);
}
