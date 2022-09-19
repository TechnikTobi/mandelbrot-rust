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

	pub fn
	read_byte
	(
		&mut self
	)
	-> u8
	{
		// Discard unread bits
		self.number_remaining_bits_in_byte = 0;
		self.memory.pop_front().unwrap()
	}

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

	pub fn
	read_bits
	(
		&mut self,
		n: u8
	)
	-> u64
	{
		let mut bits: u64 = 0;

		/*
		OLD - wrong order of reading
		for _ in 0..n
		{
			bits = bits * 2 + (self.read_bit() as u64);
		}
		*/

		for i in 0..n
		{
			bits = bits + (self.read_bit() as u64) * 2_u64.pow(i as u32);
		}

		return bits;
	}

	pub fn
	read_bytes
	(
		&mut self,
		n: u8
	)
	-> u64
	{
		let mut bytes: u64 = 0;

		/*
		OLD - wrong order of reading
		for i in 0..n
		{
			bytes = bytes + (256_u64.pow(3-i as u32)) * (self.read_byte() as u64);
		}
		*/

		for i in 0..n
		{
			bytes = bytes + (256_u64.pow(i as u32)) * (self.read_byte() as u64);
		}

		return bytes;
	}

}
