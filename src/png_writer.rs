use image;

pub fn
write_png
(
	width: u16,
	height: u16,
	filename: &String,
	rgb_data: &Vec<u8>
)
{
	let mut image_buffer = image::ImageBuffer::new(
		width.into(), 
		height.into()
	);

	assert!(width as usize * height as usize * 3 == rgb_data.len());

	for (x, y, pixel) in image_buffer.enumerate_pixels_mut() 
	{

		let pixel_index = (width as usize * y as usize + x as usize) * 3;
		*pixel = image::Rgb([
			rgb_data[pixel_index + 0], 
			rgb_data[pixel_index + 1], 
			rgb_data[pixel_index + 2] 
		]);
	}

	image_buffer.save(filename).unwrap();
}
