use clap::{Arg, App, value_t};

pub struct CliArgs
{
	pub width: u16,
	pub height: u16,
	pub x_mid: f64,
	pub y_mid: f64,
	pub zoom: f64,
	pub iterations: u64,
	pub color_mode: u8,
	pub output_file_name: String
}

impl CliArgs
{
	pub fn
	get()
	-> CliArgs
	{
		let cli = App::new("Mandelbrot Generator in Rust")
			.author("Tobias Prisching")
			.about("Generates Mandelbrot images as PNGs")
			.arg(Arg::with_name("width")
				.short('w')
				.long("width")
				.takes_value(true)
				.help("Width of the output image")
				.default_value("700")
			)
			.arg(Arg::with_name("height")
				.short('h')
				.long("height")
				.takes_value(true)
				.help("Height of the output image")
				.default_value("400")
			)
			.arg(Arg::with_name("x_mid")
				.short('x')
				.long("x_mid")
				.takes_value(true)
				.help("x-coordinate in the middle of the image")
				.default_value("0.0")
			)
			.arg(Arg::with_name("y_mid")
				.short('y')
				.long("y_mid")
				.takes_value(true)
				.help("y-coordinate in the middle of the image")
				.default_value("0.0")
			)
			.arg(Arg::with_name("zoom")
				.short('z')
				.long("zoom")
				.takes_value(true)
				.help("The zoom factor for the image")
				.default_value("0.5")
			)
			.arg(Arg::with_name("iterations")
				.short('i')
				.long("iterations")
				.takes_value(true)
				.help("Number of max iterations to use for generation")
				.default_value("255")
			)
			.arg(Arg::with_name("color_mode")
				.short('c')
				.long("color_mode")
				.takes_value(true)
				.help("The color mode to use for mapping raw to image data")
				.default_value("0")
			)
			.arg(Arg::with_name("output_file")
				.short('o')
				.long("output_file")
				.takes_value(true)
				.help("The name of the output image file")
				.default_value("image.png")
			)
			.get_matches();
		
		// See about 'value_t' macro at: https://docs.rs/clap/2.31.1/clap/macro.value_t.html
		let args = CliArgs {
			width: value_t!(cli.value_of("width"), u16).unwrap_or(400 as u16),
			height: value_t!(cli.value_of("height"), u16).unwrap_or(400 as u16),
			x_mid: value_t!(cli.value_of("x_mid"), f64).unwrap_or(0.0 as f64),
			y_mid: value_t!(cli.value_of("y_mid"), f64).unwrap_or(0.0 as f64),
			zoom: value_t!(cli.value_of("zoom"), f64).unwrap_or(0.5 as f64),
			iterations: value_t!(cli.value_of("iterations"), u64).unwrap_or(100 as u64),
			color_mode: value_t!(cli.value_of("color_mode"), u8).unwrap_or(0 as u8),
			output_file_name: value_t!(cli.value_of("output_file"), String).unwrap_or(String::from("image.png"))
		};

		return args;
	}

	pub fn
	print(&self)
	{
		println!("Width:      {}", self.width);
		println!("Height:     {}", self.height);
		println!("x_mid:      {}", self.x_mid);
		println!("y_mid:      {}", self.y_mid);
		println!("Zoom:       {}", self.zoom);
		println!("Iterations: {}", self.iterations);
		println!("Output:     {}", self.output_file_name);
	}
}
