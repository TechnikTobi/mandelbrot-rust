mod mandelbrot;
mod generator;

use generator::generate_raw_data;

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

}
