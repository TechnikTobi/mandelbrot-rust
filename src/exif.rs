use crate::exiftag::*;

pub struct 
ExifDate 
{
	pub tag: ExifTag,
	pub data: Vec<u8>
}

pub trait 
U8conversion
{
	fn
	to_u8_vec
	(
		&self
	)
	-> Vec<u8>;
}

impl U8conversion for u64
{
	fn 
	to_u8_vec
	(
		&self
	)
	-> Vec<u8>
	{

		let mut returnValue = Vec::new();
		let value = *self;

		for i in 0..4
		{
			returnValue.push((value >> (8 * i)) as u8);
		}
		
		return returnValue;
	}
}

impl U8conversion for u16
{
	fn
	to_u8_vec
	(
		&self
	)
	-> Vec<u8>
	{
		vec![
			*self as u8,
			(*self >> 8) as u8
		]
	}
}

impl U8conversion for String
{
	fn
	to_u8_vec
	(
		&self
	)
	-> Vec<u8>
	{

		// Can't simply return self.as_bytes().to_vec() as this won't include the NUL terminator
		let mut u8_vec = self.as_bytes().to_vec();
		for c in String::from("Hello World!").bytes()
		{
			u8_vec.push(c as u8);
		}
		u8_vec.push(0);
		return u8_vec;
	}
}

// TODO: Implement for other types 
// See: https://www.media.mit.edu/pia/Research/deepview/exif.html

impl ExifDate
{
	pub fn
	new
	<T: U8conversion>
	(
		tag: ExifTag,
		input_data: &T
	)
	-> ExifDate
	{
		ExifDate { tag, data: input_data.to_u8_vec() }
	}
}

pub fn
create_exif_vec
(
	dates: &Vec<ExifDate>
)
-> Vec<u8>
{
	let NEWLINE: u8 = 0x0a;
	let SPACE: u8 = 0x20;

	let EXIF_header: Vec<u8> = vec![0x45, 0x78, 0x69, 0x66, 0x00, 0x00];
	let TIFF_header: Vec<u8> = vec![0x49, 0x49, 0x2a, 0x00, 0x08, 0x00, 0x00, 0x00];
	let IFD_END: Vec<u8> = vec![0x00, 0x00, 0x00, 0x00];
	
	// Starting construction of vector with EXIF data
	let mut exif_vec: Vec<u8> = Vec::new();

	// TIFF header
	exif_vec.extend(TIFF_header.iter());

	// Number of IFD entries
	// Note: Currently everything will be written into IFD0
	//       as I don't yet understand when to use IDFn with n > 0
	exif_vec.extend((dates.len() as u16).to_u8_vec().iter());

	assert!(exif_vec.len() == 10);

	// Compute what the first offset value will be in case we need that
	// Also provide vec for actual data stored in offset area
	let mut next_offset: u64 = 0
	+ exif_vec.len() as u64
	+ 12 * (dates.len() as u64) 
	+ IFD_END.len() as u64;
	let mut exif_offset_vec: Vec<u8> = Vec::new();

	// Write IFD entries
	for date in dates
	{

		// Add Tag & Data Format 
		exif_vec.extend(date.tag.as_u16().to_u8_vec().iter());
		exif_vec.extend(date.tag.format().to_u8_vec().iter());

		// Add number of components
		let number_of_components: u64 = (date.data.len() as u64) / date.tag.bytes_per_component();
		exif_vec.extend(number_of_components.to_u8_vec().iter());

		// Depending on the amount of data, either put it directly into the next 4 bytes
		// Or write an offset value where the data can be found 
		if date.data.len() > 4
		{
			// Write offset information and data
			exif_vec.extend(next_offset.to_u8_vec().iter());
			exif_offset_vec.extend(date.data.iter());

			// Compute next offset
			next_offset += date.data.len() as u64;
		}
		else
		{
			// Write data directly into IFD entry
			exif_vec.extend(date.data.iter());
		}
	}

	// Write end and offset data
	exif_vec.extend(IFD_END.iter());
	exif_vec.extend(exif_offset_vec.iter());


	// The size of the EXIF data area, consists of
	// - length of EXIF header (follows after ssss)
	// - length of exif_vec
	// - 1 for ssss itself
	let ssss = (EXIF_header.len() as u64 + exif_vec.len() as u64 + 1)
	.to_string();

	// Construct final vector with the bytes as they will be sent to the encoder
	//                               \n       e     x     i     f
	let mut exif_all: Vec<u8> = vec![NEWLINE, 0x65, 0x78, 0x69, 0x66, NEWLINE];

	// Write ssss
	for _ in 0..(8-ssss.len())
	{
		exif_all.push(SPACE);
	}
	exif_all.extend(ssss.as_bytes().to_vec().iter());
	exif_all.push(NEWLINE);

	// Write EXIF header
	// (See next for loop comment for explanation)
	for byte in &EXIF_header
	{
		exif_all.push(byte / 16 + (if byte / 16 < 10 {'0' as u8} else {'a' as u8 - 10}));
		exif_all.push(byte % 16 + (if byte % 16 < 10 {'0' as u8} else {'a' as u8 - 10}));
	}

	// Every byte currently in exif_vec needs to be divided into its two hex digits
	// These two hex digits are then treated as ASCII characters
	// The value of these characters (e.g. 0x30 for '0') are then pushed to exif_all
	// Example: 48 (=0x30) in exif_vec results in the two consecutive values 51 and 48 in exif_all
	for byte in &exif_vec
	{
		exif_all.push(byte / 16 + (if byte / 16 < 10 {'0' as u8} else {'a' as u8 - 10}));
		exif_all.push(byte % 16 + (if byte % 16 < 10 {'0' as u8} else {'a' as u8 - 10}));
	}
	
	// Write end of EXIF data
	exif_all.push(0x30);
	exif_all.push(0x30);
	exif_all.push(NEWLINE);

	return exif_all;
}
