pub enum EColorMode
{
	DEFAULT,
	BLUE,
	BW
}

pub fn
map_raw_to_rgb
(
	raw_data: &Vec<u64>,
	iterations: u64,
	mode: EColorMode
)
-> Vec<u8>
{

	match mode
	{
		EColorMode::BLUE => color_mode_blue(raw_data, iterations),
		EColorMode::DEFAULT => color_mode_default(raw_data, iterations),
		EColorMode::BW => panic!("Not yet implemented"),
	}

}



fn color_mode_default(raw_data: &Vec<u64>, iterations: u64) -> Vec<u8> {
		
	let mut rgb_data = Vec::with_capacity(raw_data.len() * 3);

	for value in raw_data
	{
		let t_value = ((*value as f64) / (iterations as f64) * 255.0).ceil() as u8;

		let mut red: u8   = 0;
		let mut green: u8 = 0;
		let mut blue:u8   = 0;

		match t_value
		{
			0  ..=25  => { red = 10 * t_value; },
			26 ..=50  => { red = 255; 			green = 10*(t_value-25); },
			51 ..=75  => { red = 255-10*(t_value-50); 	green = 255; },
			76 ..=100 => { 					green = 255-10*(t_value-75); },
			101..=125 => { 									blue = 10*(t_value-100); },
			126..=150 => { 					green = 10*(t_value-125); 	blue = 255; },
			151..=175 => { red = 10*(t_value-150);		green = 255; 			blue = 255; },
			176..=200 => { red = 255; 			green = 255-10*(t_value-175);	blue = 255; },
			201..=225 => { red = 255;							blue = 255-10*(t_value-200); },
			226..=255 => { red = std::cmp::max(0, (255.0-8.8*(t_value as f32 - 226.0)) as i16) as u8; },   	
		};
		
		rgb_data.push(red);
		rgb_data.push(green);
		rgb_data.push(blue);
	}

	return rgb_data;
}



fn
color_mode_blue
(
	raw_data: &Vec<u64>,
	iterations: u64
)
-> Vec<u8>
{
	let mut rgb_data = Vec::with_capacity(raw_data.len() * 3);

	for value in raw_data
	{
		let transformed_value = ((*value as f64) / (iterations as f64) * 765.0).ceil() as u16;

		match transformed_value
		{
			0..=254 => {
				rgb_data.push(0);
				rgb_data.push(0);
				rgb_data.push((transformed_value).try_into().unwrap());
			},
			255..=510 => {
				rgb_data.push(0);
				rgb_data.push((transformed_value - 255).try_into().unwrap());
				rgb_data.push(255);
			},
			511..=765 => {
				rgb_data.push((transformed_value - 510).try_into().unwrap());
				rgb_data.push(255);
				rgb_data.push(255);
			},
			_ => panic!("Illegal value for 'transformed_value'"),
		}
	}

	return rgb_data;
}
