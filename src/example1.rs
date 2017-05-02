//standard types
use std::{i8, i16, i32, i64, u8, u16, u32, u64, isize, usize, f32, f64};

pub fn example1()
{
	xprintln!("Hello, world!");

	let num = 10;
	xprintln!("num: {}", num);
	let age: i32 = 12345678;

	xprintln!("Max i8: {}", i8::MAX);
	xprintln!("Min i8: {}", i8::MIN);
	xprintln!("Max i16: {}", i16::MAX);
	xprintln!("Min i16: {}", i16::MIN);
	xprintln!("Max i32: {}", i32::MAX);
	xprintln!("Min i32: {}", i32::MIN);
	xprintln!("Max i64: {}", i64::MAX);
	xprintln!("Min i64: {}", i64::MIN);

	xprintln!("Max u8: {}", u8::MAX);
	xprintln!("Min u8: {}", u8::MIN);
	xprintln!("Max u16: {}", u16::MAX);
	xprintln!("Min u16: {}", u16::MIN);
	xprintln!("Max u32: {}", u32::MAX);
	xprintln!("Min u32: {}", u32::MIN);
	xprintln!("Max u64: {}", u64::MAX);
	xprintln!("Min u64: {}", u64::MIN);

	xprintln!("Max isize: {}", isize::MAX);
	xprintln!("Min isize: {}", isize::MIN);
	xprintln!("Max usize: {}", usize::MAX);
	xprintln!("Min usize: {}", usize::MIN);

	xprintln!("Max f32: {}", f32::MAX);
	xprintln!("Min f32: {}", f32::MIN);
	xprintln!("Max f64: {}", f64::MAX);
	xprintln!("Min f64: {}", f64::MIN);

	let is_it_true: bool = true;

	let let_x: char = 'x';

	xprintln!("I am {} years old", age);

	let (f_name, l_name) = ("Lukáš", "Hozda");
}
