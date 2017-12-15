#![no_std]
// #![feature(lang_items)]

// #[macro_use] extern crate vga_buffer; // for println_unsafe!
// #[macro_use] extern crate log;

// extern crate kernel_config;

// pub static HELLO_STRING: &'static str = "hello";

#[inline(never)]
pub fn test_generic<T>(arg: T) -> T {
	arg
}


pub fn pub_test() -> u8 {
	test_generic(16u32);
	test_generic(16u64);
	test_generic("hello");
	
	5
}


// #[inline(never)]
fn test_lib_private(arg: u8) -> u8 {
	arg * 10
}


pub struct DeezNuts {
	item1: u32,
	item2: u64,
}
impl DeezNuts {
	#[inline(never)]
	pub fn new(i1: u32, i2: u64) -> DeezNuts {
		DeezNuts {
			item1: i1,
			item2: i2,
		}
	}
}

