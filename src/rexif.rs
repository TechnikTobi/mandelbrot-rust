use std::io::*;
use std::io::Read;
use std::io::BufReader;
use std::fs::File;
use crc::Crc;

use crate::bitreader::BitReader;

pub fn
add_zTXt
(
	filename: &String,
	raw_data: &Vec<u8>
)
{
	let file = File::open(filename).expect("Could not open file");
	let PNG_signature = vec![137, 80, 78, 71, 13, 10, 26, 10];

	// Create reader of the file and read length of IHDR chunk
	let mut reader = BufReader::new(file);
	let mut IHDR_chunk_start = [0; 8];

	// Seek to after the PNG signature, read 8 bytes 
	// and assert that, in fact, 8 bytes were read
	// Also check that this is in fact the IHDR chunk
	reader.seek(SeekFrom::Start(PNG_signature.len().try_into().unwrap()));
	let IHDR_chunk_start_bytes_read = reader.read(&mut IHDR_chunk_start).unwrap();
	assert!(IHDR_chunk_start_bytes_read == 8);
	if std::str::from_utf8(&IHDR_chunk_start[4..]) != Ok("IHDR")
	{
		panic!("Invalid PNG file: First chunk is NOT of type IHDR");
	}

	// Compute length of IHDR chunk
	// Adding 4 at the end for the actual ASCII chars "IHDR"
	let mut IHDR_chunk_length = 0 as u64;
	for byte in &IHDR_chunk_start[0..4]
	{
		IHDR_chunk_length = IHDR_chunk_length * 256 + *byte as u64;
	}
	IHDR_chunk_length += 4;

	// Seek to the next chunk and read everything stored there
	reader.seek_relative(IHDR_chunk_length.into());
	let mut post_zTXt_buffer = Vec::new();
	reader.read_to_end(&mut post_ZTXt_buffer);


	
	println!("add_zTXt - Success!");
		
}

pub fn
rexif_fn
(
	filename: &String
)
{
	let file = File::open(filename).unwrap();
	let signature = vec![137, 80, 78, 71, 13, 10, 26, 10];

	let mut reader = BufReader::new(file);
	let mut buffer = Vec::new();

	// Start reading after signature
	// reader.seek(SeekFrom::Start(signature.len().try_into().unwrap()));
	
	// Read data into buffer
	reader.read_to_end(&mut buffer);

	// Convert buffer into linked list for easier access
	let mut ll_buffer = std::collections::LinkedList::from_iter(buffer.iter());

	println!("BUFFER BYTES - START");
	for byte in &ll_buffer
	{
		println!("{}", byte);
		
	}
	println!("BUFFER BYTES - END");

	// Traverse linked list buffer
	while ll_buffer.len() >= 12
	{

		// let mut length = Vec::new();
		let mut length: u64 = 0;
		let mut chunk_type = String::from("");
		let mut data = Vec::new();

		// Read the length of the chunk
		for i in 0..4
		{
			length = length + (256_u64.pow(3-i)) * (*ll_buffer.pop_front().unwrap() as u64)
		}

		// Read the chunk type
		for i in 0..4
		{
			chunk_type.push(*ll_buffer.pop_front().unwrap() as char);
		}

		// Read the actual chunk data
		for _ in 0..length
		{
			data.push(*ll_buffer.pop_front().unwrap() as u8);
		}

		// Consume the four checksum bytes
		for _ in 0..4
		{
			ll_buffer.pop_front().unwrap();
		}


		if chunk_type == "zTXt"
		{
			// let test = decompress(&data[24..]);
			for val in &data
			{
				println!("{}", val);
			}
			println!("DATA END");
		}

		if chunk_type == "iTXt" // || chunk_type == "eXIf"
		{
			println!("{}", length);
			println!("{}", chunk_type);

			for val in data
			{
				print!("{}", val as char);
			}
		}
	}

	/*
	let mut ll_bitter = std::collections::LinkedList::new();
	// ll_bitter.push_front(54 as u8); // 1 as u8);
	// ll_bitter.push_front(102 as u8); // 43 as u8);
	ll_bitter.push_front(157 as u8);
	let mut bitter = BitReader::from_LL(ll_bitter);

	
	for _ in 0..8
	{
		println!("{}", bitter.read_bit());
	}
	*/

	// println!("{}", bitter.read_bits(9));

	// println!("{}", bitter.read_bytes(2));
}

// Tried, didn't work: https://github.com/SebastianS90/rust_lz77/blob/master/src/main.rs
