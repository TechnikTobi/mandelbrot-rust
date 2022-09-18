use image;
extern crate gexiv2_sys as gexiv2;

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

	image_buffer.save(filename).unwrap();
	write_exif_description(filename, description);
}

fn
write_exif_description
(
	filename: &String,
	description: &String
)
{

	// Variables needed to interface gexiv2, which is a C library
	let mut error: *mut gexiv2::GError = std::ptr::null_mut();
	let path = std::ffi::CString::new(filename.as_bytes()).unwrap();

	// Prepare EXIF tag & value as strings for C
	let tag = std::ffi::CString::new("Exif.Image.ImageDescription").unwrap();
        let value = std::ffi::CString::new(description.as_bytes()).unwrap();
	
	unsafe
	{
		// Create new Metadata struct
		let metadata = gexiv2::gexiv2_metadata_new();

		// Read in existing image file
		if 1 != gexiv2::gexiv2_metadata_open_path(metadata, path.as_ptr(), &mut error)
		{
			panic!("{}", std::ffi::CStr::from_ptr((*error).message)
				.to_str()
				.unwrap()
			);
		}

		// Add description to Metadata
		if 0 == gexiv2::gexiv2_metadata_set_tag_string(
			metadata,
			tag.as_ptr(),
			value.as_ptr()
		)
		{
			panic!("Error while setting description tag string");
		}

		// Save Metadata to image file
		if 1 != gexiv2::gexiv2_metadata_save_file(metadata, path.as_ptr(), &mut error)
		{
			panic!("{}", std::ffi::CStr::from_ptr((*error).message)
				.to_str()
				.unwrap()
			);
		}
		
	}
	
}
