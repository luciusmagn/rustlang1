//standard types
//use std::{i8, i16, i32, i64, u8, u16, u32, u64, isize, usize, f32, f64}

use std::io::stdin;

pub fn example2()
{
	let is_it_true: bool = true;
	let let_x: char = 'x';

	xprintln!("It is {0}, that {1} is {0}", is_it_true, let_x);
	xprintln!("{:.2}", 1.234);
	xprintln!("B: {:b} H: {:x} O: {:o}", 10, 10, 10);
	xprintln!("\"{ten:>ws$}\"", ten=10, ws=5);
	xprintln!("\"{ten:>0ws$}\"", ten=10, ws=5);
}
