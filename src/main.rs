use std::io::{Seek, SeekFrom, Read, Write};
use std::env;
use std::fs::{OpenOptions, File};
use std::sync::Mutex;

mod buse;

extern crate crc;

const BLOCK_SIZE: usize = 512;
const CHECKSUM_SIZE: usize = 2;

#[inline]
fn checksum(buf: &[u8]) -> [u8; CHECKSUM_SIZE] {
	let res = crc::crc16::checksum_x25(buf);
	[res as u8, (res >> 8) as u8]
}

struct State {
	sources: Vec<Source>,
	source_size: usize,
	source_block_count: usize,
	metadata_block_count: usize,
	data_block_count: usize,
}

struct Source {
	file: File,
	size: usize,
}

fn main() -> Result<(), ()> {
	let device = env::args().nth(1).expect("Expected device parameter");
	println!("device: {}", device);
	
	let sources: Vec<Source> = env::args()
		.skip(2) // skip the binary path and device parameter
		.map(|path| {
			println!("source: {}", path);
			let file = OpenOptions::new().read(true).write(true).open(path).unwrap();
			let size = file.metadata().unwrap().len() as usize;
			
			Source { file, size }
		})
		.collect();
	
	let source_size: usize = sources.iter().fold(usize::max_value(), |acc, val| acc.min(val.size));
	println!("source_size: {}", source_size);
	let source_block_count = source_size / BLOCK_SIZE;
	println!("source_block_count: {}", source_block_count);
	
	let metadata_block_count = ((CHECKSUM_SIZE as f64) / (BLOCK_SIZE) as f64 * (source_block_count as f64)).ceil() as usize;
	println!("metadata_block_count: {}", metadata_block_count);
	let data_block_count = source_block_count - metadata_block_count;
	println!("data_block_count: {}", data_block_count);
	
	let sources = Mutex::new(State {
		sources,
		source_size,
		source_block_count,
		metadata_block_count,
		data_block_count,
	});
	
	buse::main(device,
			   BLOCK_SIZE as u32,
			   data_block_count as u64,
			   |buf, offset| handle_read_buffer(&sources.lock().unwrap(), buf, offset),
			   |buf, offset| handle_write_buffer(&sources.lock().unwrap(), buf, offset))
}


fn handle_read_buffer(state: &State, buf: &mut [u8], offset: u64) -> Result<(), ()> {
	// FIXME writes don't happen in fixed block-sized buffers, use this function to split them up
	// need to handle multiple blocks being written, or buffers that aren't block-aligned (in this case, we need to read the block first, and overwrite the read buffer with the new data)
	// if writes are no longer done in blocks, then there is no need to specify the size in blocks+count, just specify the size
	handle_read(state, buf, offset)
}

fn handle_write_buffer(state: &State, buf: &[u8], offset: u64) -> Result<(), ()> {
	// FIXME same issue as above
	handle_write(state, buf, offset)
}

fn handle_read(state: &State, buf: &mut [u8], offset: u64) -> Result<(), ()> {
	println!("handle_read(..., offset: {})", offset);
	assert_eq!(buf.len(), BLOCK_SIZE);
	let source = &state.sources[0].file;
	
	// read the source block
	read(source, buf, block_location_for_offset(state.metadata_block_count, offset))?;
	
	// read the checksum
	let mut checksum_buffer = [0u8, CHECKSUM_SIZE as u8];
	read(source, &mut checksum_buffer, checksum_location_for_offset(offset))?;
	
	// verify the checksum
	let sum = checksum(buf);
	if checksum_buffer == sum {
		println!("checksum valid")
	} else {
		println!("checksum invalid");
		// TODO read from secondary disk and schedule for fixing
	}
	
	Ok(())
}

fn handle_write(state: &State, buf: &[u8], offset: u64) -> Result<(), ()> {
	println!("handle_write(..., offset: {})", offset);
	assert_eq!(buf.len(), BLOCK_SIZE);
	let source = &state.sources[0].file;
	
	// compute the checksum
	let sum = checksum(&buf);
	
	// write the source block
	write(source, buf, block_location_for_offset(state.metadata_block_count, offset))?;
	
	// write the checksum
	write(source, &sum, checksum_location_for_offset(offset))?;
	
	Ok(())
}

fn read(mut file: &File, buf: &mut [u8], offset: u64) -> Result<(), ()> {
	file.seek(SeekFrom::Start(offset)).map_err(|_| ())?;
	file.read_exact(buf).map_err(|_| ())?;
	Ok(())
}

fn write(mut file: &File, buf: &[u8], offset: u64) -> Result<(), ()> {
	file.seek(SeekFrom::Start(offset)).map_err(|_| ())?;
	file.write_all(buf).map_err(|_| ())?;
	Ok(())
}

/*fn mem_copy(src: &[u8], dst: &mut [u8]) {
	for (place, data) in dst.iter_mut().zip(src.iter()) {
		*place = *data
	}
}*/

#[inline]
fn block_location_for_offset(metadata_block_count: usize, offset: u64) -> u64 {
	metadata_block_count as u64 * BLOCK_SIZE as u64 + offset
}

#[inline]
fn checksum_location_for_offset(offset: u64) -> u64 {
	offset / BLOCK_SIZE as u64 * CHECKSUM_SIZE as u64
}