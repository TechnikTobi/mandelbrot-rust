const DEFAULT_SPAN: f64 = 2.0;

fn 
generate_raw_data 
(
	width: u16,
	height: u16,
	x_mid: f64,
	y_mid: f64,
	zoom: f64,
	iterations: u64
) 
-> Vec<u64>
{

	let x_span = DEFAULT_SPAN / zoom;
	let ppi = x_span / (width as f64);
	let y_span = ppi * (height as f64);

	let x_start = x_mid - x_span/2.0;
	let y_start = y_mid + y_span/2.0;

	let mut raw_data = Vec::new();

	for y in 0..height
	{
		for x in 0..width
		{
			raw_data.push(0
			);
		}
	}

	return raw_data;

}
