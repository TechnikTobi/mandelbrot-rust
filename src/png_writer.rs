use image;
use rexiv2;

pub fn
write_png
(
	width: u16,
	height: u16,
	filename: &String,
	description: &String,
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

	image_buffer.save(&filename).unwrap();
	write_exif_description(filename, description);
}

fn
write_exif_description
(
	filename: &String,
	description: &String
)
{

	// Initialize library before it can be used
	// Maybe not required at all?
	// rexiv2::initialize().unwrap(); // .expect("Unable to initialize rexiv2");
	
	if let Ok(meta) = rexiv2::Metadata::new_from_path(&filename)
	{
		if meta
		.set_tag_string("Exif.Image.ImageDescription", &description)
		.is_ok()
		{
			meta.save_to_file(&filename).expect("Could not write metadata to image file");
		}
	}

}
