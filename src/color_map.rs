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
		_ => panic!("Not yet implemented"),
	}

}



fn
color_mode_blue
(
	raw_data: &Vec<u64>,
	iterations: u64
)
-> Vec<u8>
{
	// let mut rgb_data = vec![0; raw_data.len() * 3];
	// let mut rgb_data = Vec::new();
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
