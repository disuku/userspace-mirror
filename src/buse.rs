extern crate libc;

use self::libc::{c_int, uint32_t, uint64_t};
use std::ffi::{CString, c_void};
use std::slice;

extern "C" {
	fn buse_main_shim(device: *const u8,
					  block_size: uint32_t,
					  block_count: uint64_t,
					  read: unsafe extern "C" fn(*mut u8, uint32_t, uint64_t, *const c_void) -> c_int, read_ctx: *const c_void,
					  write: unsafe extern "C" fn(*const u8, uint32_t, uint64_t, *const c_void) -> c_int, write_ctx: *const c_void) -> c_int;
}

pub fn main<R, W>(device: String,
				  block_size: u32,
				  block_count: u64,
				  read: R,
				  write: W) -> Result<(), ()>
	where R: Fn(&mut [u8], u64) -> Result<(), ()>,
		  W: Fn(&[u8], u64) -> Result<(), ()> {
	unsafe extern "C" fn read_wrapper<F: Fn(&mut [u8], u64) -> Result<(), ()>>(buf: *mut u8, len: uint32_t, offset: uint64_t, ctx: *const c_void) -> c_int {
		match (*(ctx as *const F))(slice::from_raw_parts_mut(buf, len as usize), offset) {
			Ok(_) => 0,
			Err(_) => 1
		}
	}
	unsafe extern "C" fn write_wrapper<F: Fn(&[u8], u64) -> Result<(), ()>>(buf: *const u8, len: uint32_t, offset: uint64_t, ctx: *const c_void) -> c_int {
		match (*(ctx as *const F))(slice::from_raw_parts(buf, len as usize), offset) {
			Ok(_) => 0,
			Err(_) => 1
		}
	}
	
	match unsafe {
		buse_main_shim(CString::new(device).expect("failed").as_ptr() as *const u8,
					   block_size,
					   block_count,
					   read_wrapper::<R>, &read as *const R as *const c_void,
					   write_wrapper::<W>, &write as *const W as *const c_void)
	} {
		0 => Ok(()),
		_ => Err(())
	}
}