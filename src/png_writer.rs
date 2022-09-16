use image;

pub fn
write_png
(
	width: u16,
	height: u16,
	filename: &str,
	rgb_data: std::vec::Vec<u8>
)
{
	let mut image_buffer = image::ImageBuffer::new(
		width.into(), 
		height.into()
	);

	for (_, _, pixel) in image_buffer.enumerate_pixels_mut() 
	{
		*pixel = image::Rgb([
			0 as u8, 
			0 as u8, 
			0 as u8
		]);
	}

	image_buffer.save(filename).unwrap();
}
