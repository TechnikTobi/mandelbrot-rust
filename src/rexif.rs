use std::io::*;
use std::io::Read;
use std::io::BufReader;
use std::fs::File;

use crate::bitreader::BitReader;

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
	reader.seek(SeekFrom::Start(signature.len().try_into().unwrap()));
	
	// Read data into buffer
	reader.read_to_end(&mut buffer);

	// Convert buffer into linked list for easier access
	let mut ll_buffer = std::collections::LinkedList::from_iter(buffer.iter());

	/*	
	for byte in ll_buffer
	{
		println!("{}", byte);
		
	}
	*/

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

	let mut ll_bitter = std::collections::LinkedList::new();
	// ll_bitter.push_front(54 as u8); // 1 as u8);
	// ll_bitter.push_front(102 as u8); // 43 as u8);
	ll_bitter.push_front(157 as u8);
	let mut bitter = BitReader::from_LL(ll_bitter);

	
	for _ in 0..8
	{
		println!("{}", bitter.read_bit());
	}

	// println!("{}", bitter.read_bits(9));

	// println!("{}", bitter.read_bytes(2));
}

// Tried, didn't work: https://github.com/SebastianS90/rust_lz77/blob/master/src/main.rs
