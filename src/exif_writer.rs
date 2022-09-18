extern crate gexiv2_sys as gexiv2;

pub fn
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
