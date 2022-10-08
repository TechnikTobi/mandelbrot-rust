use little_exif::metadata::Metadata;
use little_exif::exif_tag::ExifTag;

pub fn
write_exif_description
(
        filename: &String,
        description: &String
)
{

	let file_path = std::path::Path::new(filename);

	// Create metadata struct
	let mut metadata = Metadata::new_from_path(file_path);

	// Add the image description tag
	metadata.set_tag(
		ExifTag::ImageDescription(description.to_string())
	);

	// Write metadata to image file
	metadata.write_to_file(file_path);

}
