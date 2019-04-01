use std::io::{Seek, SeekFrom, Read, Write};
use std::env;
use std::fs::OpenOptions;
use std::sync::Mutex;

mod buse;

fn main() -> Result<(), ()> {
    let device = env::args().nth(1).expect("Expected device parameter");
    println!("device: {}", device);

    let source = env::args().nth(2).expect("Expected source file");
    println!("source: {}", source);

    let source_file = OpenOptions::new().read(true).write(true).open(source).unwrap();
    let size = source_file.metadata().unwrap().len();

    let source_file = Mutex::new(source_file);

    buse::main(device, size, |buf, offset| -> Result<(), ()> {
        let mut source_file = source_file.lock().unwrap();
        source_file.seek(SeekFrom::Start(offset)).unwrap();
        source_file.read_exact(buf).unwrap();
        Ok(())
    }, |buf, offset| -> Result<(), ()> {
        let mut source_file = source_file.lock().unwrap();
        source_file.seek(SeekFrom::Start(offset)).unwrap();
        source_file.write_all(buf).unwrap();
        Ok(())
    })
}