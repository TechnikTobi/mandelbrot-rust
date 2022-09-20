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

use rexif::rexif_fn;
use exif::{ExifDate, create_exif_vec};
use exiftag::ExifTag;
use miniz_oxide::deflate::compress_to_vec;
use deflate::deflate_bytes_zlib;

fn main() {

	let args = CliArgs::get();
	args.print();

	/*
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
	*/

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

	/*
	write_png(
		args.width, 
		args.height, 
		&args.output_file_name, 
		&rgb_data
	);

	
	write_exif_description(
		&args.output_file_name,
		/* &String::from("hJ") */ &args.to_cli_string()
	);
	

	rexif_fn(
		&String::from("hJ.png") // &args.output_file_name
	);
	*/

	let some_vec = create_exif_vec(
		&vec![ExifDate::new(ExifTag::ImageDescription, &args.to_cli_string())]
	);

	/*
	for val in &some_vec{
		println!("{}", val);
	}

	println!("DECODING");

	let mut other_val = u8::MAX as char;
	for val in &some_vec
	{

		if other_val == (u8::MAX as char) && *val != 10
		{
			other_val = *val as char;
		}
		else if *val == 10
		{
			println!("NEWLINE");
		}
		else if other_val != (u8::MAX as char)
		{
			let mut hex_val = String::from("0x");
			hex_val.push(other_val);
			hex_val.push(*val as char);

			if 
			hex_val != "0xex" && 
			hex_val != "0xif" &&
			hex_val != "0x  "
			{
				println!(
					"{} {}", 
					hex_val, 
					(i64::from_str_radix(&hex_val.trim_start_matches("0x"), 16).unwrap() as u8) as char
				);
			}
			else
			{
				println!("{}", hex_val);
			}
 
			other_val = u8::MAX as char;
		}

	}
	*/

	// let compressed_vec = compress_to_vec(&some_vec, 2);
	let compressed_vec = deflate_bytes_zlib(&some_vec);

	println!("COMPRESSED DATA");
	for val in &compressed_vec
	{
		println!("{}", val);
	}
}
