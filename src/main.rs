mod mandelbrot;
mod generator;
mod exif_writer;
mod png_writer;
mod color_map;
mod cli;
mod rexif;
mod exif;
mod exiftag;
mod bitreader;

use generator::generate_raw_data;
use png_writer::write_png;
use exif_writer::write_exif_description;
use color_map::{to_EColorMode, map_raw_to_rgb};
use cli::CliArgs;

use rexif::{add_zTXt, rexif_fn};
use exif::{ExifDate, create_exif_vec};
use exiftag::ExifTag;
// use miniz_oxide::deflate::compress_to_vec;
// use deflate::deflate_bytes_zlib;

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

	/*	
	write_exif_description(
		&args.output_file_name,
		/* &String::from("hJ") */ &args.to_cli_string()
	);
	
	*/

	let some_vec = create_exif_vec(
		&vec![ExifDate::new(ExifTag::ImageDescription, &args.to_cli_string())]
	);

	add_zTXt(
		&args.output_file_name,
		&some_vec
	);

	// let compressed_vec = deflate_bytes_zlib(&some_vec);

	/*	
	rexif_fn(
		/* &String::from("hJ.png") */ &args.output_file_name
	);
	*/

}
