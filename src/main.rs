mod mandelbrot;
mod generator;
mod exif_writer;
mod png_writer;
mod color_map;
mod cli;

use generator::generate_raw_data;
use png_writer::write_png;
use exif_writer::write_exif_description;
use color_map::{to_EColorMode, map_raw_to_rgb};
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
		to_EColorMode(args.color_mode)
	);

	write_png(
		args.width, 
		args.height, 
		&args.output_file_name, 
		&rgb_data
	);

	write_exif_description(
		&args.output_file_name,
		&args.to_cli_string()
	);
	
}
