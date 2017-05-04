extern crate ansi_term;

#[macro_use]
mod macros;

mod examples;
use examples::*;
use ansi_term::Colour::*;

static mut EXAMPLE_COUNT:u8 = 0;

pub fn example(f: &Fn())
{
	unsafe
	{
		EXAMPLE_COUNT += 1;
		println!("{}",
			Yellow.paint(
				format!("Example {} output:",
						 EXAMPLE_COUNT
				)
			)
		);
		f();
	}
}

fn main()
{
	example(&example1);
	example(&example2);
	example(&example3);
	example(&example4);
	example(&example5);
	example(&example6);
	example(&example7);
	example(&example8);
	example(&example9);
	example(&example10);
}
