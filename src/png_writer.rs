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

	/*
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
	*/

	let mut error: *mut gexiv2::GError = std::ptr::null_mut();
	let path = std::ffi::CString::new(filename.as_bytes()).unwrap();

	// Prepare EXIF tag & value
	let tag = std::ffi::CString::new("Exif.Image.ImageDescription").unwrap();
        let value = std::ffi::CString::new(description.as_bytes()).unwrap();
	
	unsafe
	{
		// Create new Metadata struct
		let metadata = gexiv2::gexiv2_metadata_new();

		// Read in existing image file
		gexiv2::gexiv2_metadata_open_path(metadata, path.as_ptr(), &mut error);

		// Add description to Metadata
		let result: libc::c_int = gexiv2::gexiv2_metadata_set_tag_string(
			metadata,
			tag.as_ptr(),
			value.as_ptr()
		);

		// Save Metadata to image file
		gexiv2::gexiv2_metadata_save_file(metadata, path.as_ptr(), &mut error);
		
	}
	
}
