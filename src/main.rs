use std::io::{Seek, SeekFrom, Read, Write};
use std::env;
use std::fs::{OpenOptions, File};
use std::sync::Mutex;

mod buse;

struct Source {
	file: File,
	size: u64,
}

fn main() -> Result<(), ()> {
	let device = env::args().nth(1).expect("Expected device parameter");
	println!("device: {}", device);
	
	let sources: Vec<Source> = env::args()
		.skip(2) // skip the binary path and device parameter
		.map(|path| {
			println!("source: {}", path);
			let file = OpenOptions::new().read(true).write(true).open(path).unwrap();
			let size = file.metadata().unwrap().len();
			
			Source { file, size }
		})
		.collect();
	
	let size = sources.iter().fold(0, |acc, val| acc.max(val.size));
	println!("size: {}", size);
	
	let sources = Mutex::new(sources);
	
	buse::main(device, size,
			   |buf, offset| read(&sources.lock().unwrap(), buf, offset),
			   |buf, offset| write(&sources.lock().unwrap(), buf, offset))
}

fn read(sources: &Vec<Source>, buf: &mut [u8], offset: u64) -> Result<(), ()> {
	let mut source = &sources[0].file;
	
	source.seek(SeekFrom::Start(offset)).unwrap();
	source.read_exact(buf).unwrap();
	Ok(())
}

fn write(sources: &Vec<Source>, buf: &[u8], offset: u64) -> Result<(), ()> {
	let mut source = &sources[0].file;
	
	source.seek(SeekFrom::Start(offset)).unwrap();
	source.write_all(buf).unwrap();
	Ok(())
}