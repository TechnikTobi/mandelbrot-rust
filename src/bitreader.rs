use std::collections::LinkedList;

pub struct BitReader
{
	pub memory: LinkedList<u8>,
	pub current_byte: u8,
	pub number_remaining_bits_in_byte: u8,
}

impl BitReader
{
	pub fn
	from_LL
	(
		memory: LinkedList<u8>
	)
	-> BitReader
	{
		return BitReader {
			memory: memory,
			current_byte: 0, 
			number_remaining_bits_in_byte: 0
		};
	}

	// Reads a byte from the memory (NOT setting self.current_byte)
	// Discarding any unread bits from the current byte
	pub fn
	read_byte
	(
		&mut self
	)
	-> u8
	{
		self.number_remaining_bits_in_byte = 0;
		self.memory.pop_front().unwrap()
	}

	// Reads a bit from the the current byte
	// Fetching a new byte if the current contains no more bits
	pub fn
	read_bit
	(
		&mut self
	)
	-> u8
	{
		// If no more bits remain, read a new byte
		if self.number_remaining_bits_in_byte == 0
		{
			self.current_byte = self.read_byte();
			self.number_remaining_bits_in_byte = 8;
		}
		
		let bit = self.current_byte & (1 as u8);
		self.current_byte /= 2;	
		self.number_remaining_bits_in_byte -= 1;
		return bit;
		
	}

	// Reads a given number of bits and constructs a u64 from them
	// Starting with the first bit read as the least significant one
	// Stops after reading 64 bits because u64 can't handle more 
	pub fn
	read_bits
	(
		&mut self,
		n: u8
	)
	-> u64
	{
		let mut bits: u64 = 0;

		for i in 0..std::cmp::min(64, n)
		{
			bits = bits + (self.read_bit() as u64) * 2_u64.pow(i.into());
		}

		return bits;
	}

	// Reads a given number of bytes and constructs a u64 from them
	// Starting with the first byte read as the least significant one
	// Stops after reading 4 bytes because u64 can't handle more
	pub fn
	read_bytes
	(
		&mut self,
		n: u8
	)
	-> u64
	{
		let mut bytes: u64 = 0;

		for i in 0..std::cmp::min(4, n)
		{
			bytes = bytes + (256_u64.pow(i.into())) * (self.read_byte() as u64);
		}

		return bytes;
	}

}

pub fn
decompress
(
	input: LinkedList<u8>
)
{
	let mut bit_reader = BitReader::from_LL(input);

	/*
	// Read byte regarding compression method
	let CMF = bit_reader.read_byte();
	
	// Read information about 
	// - Compression Method (CM) 
	// - Compression Info (CINFO)
	let CM = CMF & (15 as u8);
	let CINFO = CMF & (240 as u8);
	*/

	// Read CMF, containing information about 
        // - Compression Method (CM) 
        // - Compression Info (CINFO)
	let CM = bit_reader.read_bits(4) as u8;
	let CINFO = bit_reader.read_bits(4) as u8;

	if CM != 8
	{
		panic!("Invalid Compression Method (CM): {}", CM);
	}
	
	if CINFO > 7
	{
		panic!("Invalid Compression Info (CINFO): {}", CINFO);
	}

	// Read FLG, containing information about
	// - Checksum for CMF * 256 + FLG (FCHECK)
	// - Whether a dictionary is following (FDICT)
	// - Compression level (FLEVEL)
	let FCHECK = bit_reader.read_bits(5) as u64;
	let FDICT = (bit_reader.read_bits(1) != 0) as bool;
	let FLEVEL = bit_reader.read_bits(2) as u8;

	if ((CINFO * 16 + CM) as u64 * 256 + FCHECK) % 31 != 0
	{
		panic!("Failed checksum");
	}

	if FDICT
	{
		panic!("Preset dictionary currently not supported!");
	}

	// let output = inflate(bit_reader);
	
	
}

/*
fn
inflate
(
	bit_reader: BitReader
)
-> Vec<u8>
*/
